//! Motor de captura secuencial por teclado (ADR-0008).
//!
//! Cada operación describe sus pasos con un [`FlowSpec`]; [`FlowScreen`] se
//! encarga del teclado y de las fases:
//!
//! 1. **Captura**: un paso activo; Enter valida y avanza, Esc retrocede, F1
//!    abre la consulta del paso.
//! 2. **Revisión**: resumen; G graba, C corrige desde el primer paso.
//! 3. **Resultado**: mensaje y «¿Continuar? S/N».

use std::collections::BTreeMap;

use chrono::NaiveDate;
use egui::{
    Key, Modifiers, RichText, TextEdit, Ui,
    text::{CCursor, CCursorRange},
};
use pos_core::{
    application::{OpError, PosStore},
    domain::{Money, Rate, Weight},
};

use super::screens::{Outcome, Screen};
use crate::{
    components::{
        BadgeTone, Card, LookupItem, LookupOutcome, LookupState, PrimaryButton, SecondaryButton,
        StatusBadge, enter_value, filter_items, key_hints, lookup_dialog, single_line,
        suggestion_list,
    },
    strings::flow as text,
    theme::tokens::{color, font_size, space},
};

/// Ancho a partir del cual el contexto va al lado de la captura.
const SIDE_BY_SIDE_MIN_WIDTH: f32 = 900.0;

/// Fecha y almacén que ven los flujos.
pub struct FlowCx<'a> {
    pub store: &'a PosStore,
    pub today: NaiveDate,
}

/// Catálogo que se consulta con F1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupSource {
    Customers,
    Merchandise,
    GoldQualities,
    Concepts,
}

impl LookupSource {
    fn title(self) -> &'static str {
        match self {
            Self::Customers => text::LOOKUP_CUSTOMERS,
            Self::Merchandise => text::LOOKUP_MERCHANDISE,
            Self::GoldQualities => text::LOOKUP_GOLD,
            Self::Concepts => text::LOOKUP_CONCEPTS,
        }
    }

    /// Elementos de la consulta.
    #[must_use]
    pub fn items_for(self, store: &PosStore) -> Vec<LookupItem> {
        match self {
            Self::Customers => store
                .customers()
                .iter()
                .map(|c| LookupItem {
                    value: c.id_number.clone(),
                    label: c.name.clone(),
                    detail: c.phone.clone(),
                })
                .collect(),
            Self::Merchandise => store
                .merchandise()
                .iter()
                .map(|m| LookupItem {
                    value: m.code.to_string(),
                    label: m.name.clone(),
                    detail: String::new(),
                })
                .collect(),
            Self::GoldQualities => store
                .gold_qualities()
                .iter()
                .map(|g| LookupItem {
                    value: g.karat.to_string(),
                    label: g.name.clone(),
                    detail: String::new(),
                })
                .collect(),
            Self::Concepts => store
                .concepts()
                .iter()
                .map(|c| LookupItem {
                    value: c.code.to_string(),
                    label: c.name.clone(),
                    detail: c.category.label().to_owned(),
                })
                .collect(),
        }
    }

    /// Nombre del elemento con ese valor, si existe.
    fn name(self, store: &PosStore, value: &str) -> Option<String> {
        match self {
            Self::Customers => store.customer(value).map(|c| c.name.clone()),
            Self::Merchandise => value
                .parse()
                .ok()
                .and_then(|code| store.merchandise_code(code))
                .map(|m| m.name.clone()),
            Self::GoldQualities => value
                .parse::<u8>()
                .ok()
                .and_then(|k| store.gold_qualities().iter().find(|g| g.karat == k))
                .map(|g| g.name.clone()),
            Self::Concepts => value
                .parse()
                .ok()
                .and_then(|code| store.concept(code))
                .map(|c| c.name.clone()),
        }
    }
}

/// Tipo de dato de un paso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepKind {
    Text {
        optional: bool,
    },
    Integer {
        min: u32,
        max: u32,
    },
    Money,
    Rate,
    Weight {
        optional: bool,
    },
    Date,
    /// Opciones (valor, texto); se eligen con su número o con las flechas.
    Choice(Vec<(&'static str, &'static str)>),
    YesNo,
    Lookup {
        source: LookupSource,
        optional: bool,
    },
}

/// Un paso de captura.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepDef {
    pub id: &'static str,
    pub label: String,
    pub kind: StepKind,
    pub hint: Option<String>,
    /// Valor inicial, ya en su forma canónica.
    pub default: Option<String>,
    /// Registros sugeridos bajo el campo y en la consulta F1 (título, elementos).
    pub suggestions: Option<(&'static str, Vec<LookupItem>)>,
}

impl StepDef {
    #[must_use]
    pub fn new(id: &'static str, label: impl Into<String>, kind: StepKind) -> Self {
        Self {
            id,
            label: label.into(),
            kind,
            hint: None,
            default: None,
            suggestions: None,
        }
    }

    /// Registros que se muestran bajo el campo y en la consulta F1.
    #[must_use]
    pub fn suggest(mut self, title: &'static str, items: Vec<LookupItem>) -> Self {
        self.suggestions = Some((title, items));
        self
    }

    /// Título y elementos consultables del paso, si tiene.
    fn items(&self, store: &PosStore) -> Option<(&'static str, Vec<LookupItem>)> {
        match (&self.suggestions, &self.kind) {
            (Some((title, items)), _) => Some((title, items.clone())),
            (None, StepKind::Lookup { source, .. }) => {
                Some((source.title(), source.items_for(store)))
            }
            _ => None,
        }
    }

    fn searchable(&self) -> bool {
        self.suggestions.is_some() || matches!(self.kind, StepKind::Lookup { .. })
    }

    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    #[must_use]
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }
}

/// Valores capturados, en forma canónica: importes en pesos, porcentajes en
/// centésimas, pesos en miligramos, fechas ISO, sí/no como «S»/«N».
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Values(BTreeMap<&'static str, String>);

impl Values {
    #[must_use]
    pub fn get(&self, id: &str) -> &str {
        self.0.get(id).map_or("", String::as_str)
    }

    #[must_use]
    pub fn has(&self, id: &str) -> bool {
        !self.get(id).is_empty()
    }

    #[must_use]
    pub fn money(&self, id: &str) -> Money {
        Money::pesos(self.get(id).parse().unwrap_or(0))
    }

    #[must_use]
    pub fn rate(&self, id: &str) -> Rate {
        Rate::from_hundredths(self.get(id).parse().unwrap_or(0))
    }

    #[must_use]
    pub fn weight(&self, id: &str) -> Option<Weight> {
        self.get(id).parse().ok().map(Weight::milligrams)
    }

    #[must_use]
    pub fn date(&self, id: &str) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(self.get(id), "%Y-%m-%d").ok()
    }

    #[must_use]
    pub fn number(&self, id: &str) -> u32 {
        self.get(id).parse().unwrap_or(0)
    }

    #[must_use]
    pub fn yes(&self, id: &str) -> bool {
        self.get(id) == "S"
    }

    fn set(&mut self, id: &'static str, value: String) {
        self.0.insert(id, value);
    }
}

/// Descripción de una operación capturada por pasos.
pub trait FlowSpec {
    /// Pasos aplicables con los valores actuales (pueden depender de ellos).
    fn steps(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef>;

    /// Validación de negocio de un paso, con el valor ya en forma canónica.
    ///
    /// # Errors
    /// Mensaje para el usuario si el valor no sirve.
    fn check(
        &self,
        _cx: &FlowCx<'_>,
        _values: &Values,
        _step: &'static str,
        _value: &str,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Información de apoyo junto a la captura (ficha del contrato, listados…).
    fn context(&self, _ui: &mut Ui, _cx: &FlowCx<'_>, _values: &Values) {}

    /// Líneas adicionales del resumen (cálculos), además de los valores.
    fn review_extra(&self, _cx: &FlowCx<'_>, _values: &Values) -> Vec<(String, String)> {
        Vec::new()
    }

    /// Ejecuta la operación. Devuelve el mensaje de éxito.
    ///
    /// # Errors
    /// El error de la operación, que se muestra en el resumen.
    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError>;

    /// Pregunta final («¿Desea ingresar otro contrato?»).
    fn again_question(&self) -> &'static str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Phase {
    Editing,
    Review,
    Done(String),
}

/// Pantalla que ejecuta un [`FlowSpec`].
pub struct FlowScreen<S> {
    spec: S,
    values: Values,
    current: usize,
    buffer: String,
    choice: usize,
    phase: Phase,
    error: Option<String>,
    focus: bool,
    lookup: Option<LookupState>,
    /// Sugerencia seleccionada con ↑ ↓.
    highlight: Option<usize>,
}

impl<S: FlowSpec> FlowScreen<S> {
    #[must_use]
    pub fn new(spec: S) -> Self {
        Self {
            spec,
            values: Values::default(),
            current: 0,
            buffer: String::new(),
            choice: 0,
            phase: Phase::Editing,
            error: None,
            focus: true,
            lookup: None,
            highlight: None,
        }
    }

    /// Vuelve al primer paso con todo vacío, para una nueva operación.
    fn restart(&mut self) {
        self.values = Values::default();
        self.current = 0;
        self.buffer.clear();
        self.choice = 0;
        self.phase = Phase::Editing;
        self.error = None;
        self.focus = true;
        self.lookup = None;
    }

    /// Prepara el campo de entrada del paso `index`.
    fn enter_step(&mut self, index: usize, steps: &[StepDef]) {
        self.current = index;
        self.error = None;
        self.focus = true;
        self.highlight = None;
        let Some(step) = steps.get(index) else { return };
        let canonical = self
            .values
            .0
            .get(step.id)
            .cloned()
            .or_else(|| step.default.clone())
            .unwrap_or_default();
        self.buffer = input_text(&step.kind, &canonical);
        if let StepKind::Choice(options) = &step.kind {
            self.choice = options
                .iter()
                .position(|(value, _)| *value == canonical)
                .unwrap_or(0);
        }
    }

    /// Valida el valor del paso actual y avanza.
    fn submit(&mut self, raw: &str, steps: &[StepDef], cx: &FlowCx<'_>) {
        let Some(step) = steps.get(self.current) else {
            return;
        };
        let canonical = match normalize(&step.kind, raw, cx) {
            Ok(value) => value,
            Err(message) => {
                self.error = Some(message);
                return;
            }
        };
        if let Err(message) = self.spec.check(cx, &self.values, step.id, &canonical) {
            self.error = Some(message);
            return;
        }
        self.values.set(step.id, canonical);
        let steps = self.spec.steps(cx, &self.values);
        if self.current + 1 < steps.len() {
            self.enter_step(self.current + 1, &steps);
        } else {
            self.phase = Phase::Review;
            self.error = None;
        }
    }

    fn step_back(&mut self, steps: &[StepDef]) -> Outcome {
        if self.current == 0 {
            return Outcome::Back;
        }
        self.enter_step(self.current - 1, steps);
        Outcome::Stay
    }

    fn completed_ui(&self, ui: &mut Ui, steps: &[StepDef], cx: &FlowCx<'_>, upto: usize) {
        if upto == 0 {
            return;
        }
        egui::Grid::new("flow_completed")
            .num_columns(2)
            .spacing([space::L, space::XS])
            .show(ui, |ui| {
                for step in steps.iter().take(upto) {
                    ui.label(RichText::new(&step.label).color(color::TEXT_MUTED));
                    ui.label(
                        RichText::new(display(&step.kind, self.values.get(step.id), cx))
                            .color(color::TEXT),
                    );
                    ui.end_row();
                }
            });
        ui.add_space(space::M);
    }

    /// Paso actual. Devuelve lo que pidió el usuario.
    fn editing_ui(&mut self, ui: &mut Ui, steps: &[StepDef], cx: &FlowCx<'_>) -> Option<Action> {
        let step = steps.get(self.current)?.clone();
        let mut action = None;
        let position = format!(
            "{} {} {} {}",
            text::STEP,
            self.current + 1,
            text::OF,
            steps.len()
        );
        Card::new().show(ui, |ui| {
            ui.label(
                RichText::new(position)
                    .size(font_size::SMALL)
                    .color(color::TEXT_MUTED),
            );
            let label = ui.label(RichText::new(&step.label).heading().color(color::TEXT));
            if let Some(hint) = &step.hint {
                ui.label(RichText::new(hint).color(color::TEXT_MUTED));
            }
            ui.add_space(space::S);
            action = match &step.kind {
                StepKind::Choice(options) => self.choice_ui(ui, options),
                StepKind::YesNo => yes_no_ui(ui, step.default.as_deref()),
                _ => self.text_ui(ui, &step, cx, label.id),
            };
            if let Some(error) = &self.error {
                ui.label(RichText::new(format!("⚠ {error}")).color(color::ERROR));
            }
            ui.add_space(space::S);
            ui.horizontal(|ui| {
                if ui.add(PrimaryButton::new(text::CONTINUE)).clicked() {
                    action = Some(Action::Submit(self.current_raw(&step)));
                }
                if ui.add(SecondaryButton::new(text::BACK)).clicked() {
                    action = Some(Action::Back);
                }
                if step.searchable() && ui.add(SecondaryButton::new(text::SEARCH_ICON)).clicked() {
                    action = Some(Action::OpenLookup);
                }
            });
        });
        let mut hints = vec![("Enter", text::CONTINUE), ("Esc", text::BACK)];
        if step.searchable() {
            hints.push(("↑ ↓", text::CHOOSE_SUGGESTION));
            hints.push(("F1", text::SEARCH_ALL));
        }
        if matches!(step.kind, StepKind::Choice(_)) {
            hints.push(("↑ ↓ / 1–9", text::CHOOSE));
        }
        if matches!(step.kind, StepKind::YesNo) {
            hints.push(("S / N", text::ANSWER));
        }
        key_hints(ui, &hints);
        action
    }

    /// Valores con la sugerencia seleccionada (o el valor escrito, si coincide
    /// exactamente) en el paso actual, para que el panel de contexto muestre su
    /// ficha antes de confirmarla.
    fn preview_values(&self, steps: &[StepDef], cx: &FlowCx<'_>) -> Option<Values> {
        if self.phase != Phase::Editing {
            return None;
        }
        let step = steps.get(self.current)?;
        let (_, items) = step.items(cx.store)?;
        let typed = self.buffer.trim();
        let value = self
            .highlight
            .and_then(|index| {
                filter_items(&items, &self.buffer)
                    .get(index)
                    .map(|item| item.value.clone())
            })
            .or_else(|| {
                items
                    .iter()
                    .find(|item| item.value == typed)
                    .map(|item| item.value.clone())
            })?;
        let mut values = self.values.clone();
        values.set(step.id, value);
        Some(values)
    }

    fn current_raw(&self, step: &StepDef) -> String {
        match &step.kind {
            StepKind::Choice(options) => options
                .get(self.choice)
                .map(|(v, _)| (*v).to_owned())
                .unwrap_or_default(),
            StepKind::YesNo => step.default.clone().unwrap_or_else(|| "N".into()),
            _ => self.buffer.clone(),
        }
    }

    fn text_ui(
        &mut self,
        ui: &mut Ui,
        step: &StepDef,
        cx: &FlowCx<'_>,
        label: egui::Id,
    ) -> Option<Action> {
        let items = step.items(cx.store);
        let money = step.kind == StepKind::Money;
        let response = ui
            .horizontal(|ui| {
                if money {
                    ui.label(RichText::new("$").heading().color(color::TEXT_MUTED));
                }
                let edit =
                    single_line(&mut self.buffer, ("flow_input", step.id)).desired_width(320.0);
                ui.add(edit).labelled_by(label)
            })
            .inner;
        if self.focus {
            response.request_focus();
            self.focus = false;
        }
        if response.changed() {
            self.highlight = None;
            if money {
                regroup_thousands(ui.ctx(), response.id, &mut self.buffer);
            }
        }
        if let StepKind::Lookup { source, .. } = step.kind
            && let Some(name) = source.name(cx.store, self.buffer.trim())
        {
            ui.label(RichText::new(name).color(color::TEXT_MUTED));
        }
        let (enter, escape, f1) = ui.input(|i| {
            (
                i.key_pressed(Key::Enter),
                i.key_pressed(Key::Escape),
                i.key_pressed(Key::F1),
            )
        });
        if let Some((_, items)) = &items
            && let Some(value) = suggestion_list(
                ui,
                items,
                &self.buffer,
                &mut self.highlight,
                response.has_focus(),
            )
        {
            self.focus = true;
            return Some(Action::Submit(value));
        }
        // Enter y Esc cuentan tanto si el campo acaba de soltar el foco (egui lo
        // suelta con esas teclas) como si el foco estaba en otra parte.
        let free = response.lost_focus() || !response.has_focus();
        if enter && free {
            self.focus = true;
            let value = items.as_ref().map_or_else(
                || self.buffer.clone(),
                |(_, items)| enter_value(items, &self.buffer, self.highlight),
            );
            return Some(Action::Submit(value));
        }
        if escape && free {
            return Some(Action::Back);
        }
        if response.has_focus() && f1 && items.is_some() {
            return Some(Action::OpenLookup);
        }
        None
    }

    fn choice_ui(
        &mut self,
        ui: &mut Ui,
        options: &[(&'static str, &'static str)],
    ) -> Option<Action> {
        let mut action = None;
        for (index, (value, label)) in options.iter().enumerate() {
            let selected = index == self.choice;
            let text = format!("{}  {label}", index + 1);
            if ui
                .selectable_label(selected, RichText::new(text).size(font_size::BODY + 1.0))
                .clicked()
            {
                self.choice = index;
                action = Some(Action::Submit((*value).to_owned()));
            }
        }
        let (down, up, enter, escape, digit) = ui.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::ArrowDown),
                i.consume_key(Modifiers::NONE, Key::ArrowUp),
                i.consume_key(Modifiers::NONE, Key::Enter),
                i.consume_key(Modifiers::NONE, Key::Escape),
                pressed_digit(i),
            )
        });
        if down && self.choice + 1 < options.len() {
            self.choice += 1;
        }
        if up {
            self.choice = self.choice.saturating_sub(1);
        }
        // Un número sólo selecciona la opción; Enter la confirma.
        if let Some(digit) = digit
            && digit <= options.len()
        {
            self.choice = digit - 1;
        }
        if enter && let Some((value, _)) = options.get(self.choice) {
            action = Some(Action::Submit((*value).to_owned()));
        }
        if escape {
            action = Some(Action::Back);
        }
        action
    }
}

/// Lo que el usuario pidió durante un frame.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Submit(String),
    Back,
    OpenLookup,
    Save,
    Correct,
    Again,
    Finish,
}

fn yes_no_ui(ui: &mut Ui, default: Option<&str>) -> Option<Action> {
    let mut action = None;
    ui.horizontal(|ui| {
        if ui.add(SecondaryButton::new(text::YES)).clicked() {
            action = Some(Action::Submit("S".into()));
        }
        if ui.add(SecondaryButton::new(text::NO)).clicked() {
            action = Some(Action::Submit("N".into()));
        }
    });
    let (yes, no, enter, escape) = ui.input_mut(|i| {
        (
            i.consume_key(Modifiers::NONE, Key::S),
            i.consume_key(Modifiers::NONE, Key::N),
            i.consume_key(Modifiers::NONE, Key::Enter),
            i.consume_key(Modifiers::NONE, Key::Escape),
        )
    });
    if yes {
        action = Some(Action::Submit("S".into()));
    } else if no {
        action = Some(Action::Submit("N".into()));
    } else if enter {
        action = Some(Action::Submit(default.unwrap_or("N").to_owned()));
    } else if escape {
        action = Some(Action::Back);
    }
    action
}

fn pressed_digit(input: &mut egui::InputState) -> Option<usize> {
    const DIGITS: [Key; 9] = [
        Key::Num1,
        Key::Num2,
        Key::Num3,
        Key::Num4,
        Key::Num5,
        Key::Num6,
        Key::Num7,
        Key::Num8,
        Key::Num9,
    ];
    DIGITS
        .iter()
        .position(|key| input.consume_key(Modifiers::NONE, *key))
        .map(|index| index + 1)
}

impl<S: FlowSpec> Screen for FlowScreen<S> {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        let mut action = None;
        let mut outcome = Outcome::Stay;
        {
            let cx = FlowCx { store, today };
            let steps = self.spec.steps(&cx, &self.values);
            let wide = ui.available_width() >= SIDE_BY_SIDE_MIN_WIDTH;
            let main = |ui: &mut Ui, this: &mut Self| -> Option<Action> {
                match this.phase.clone() {
                    Phase::Editing => {
                        this.completed_ui(ui, &steps, &cx, this.current);
                        if this.lookup.is_some() {
                            None
                        } else {
                            this.editing_ui(ui, &steps, &cx)
                        }
                    }
                    Phase::Review => {
                        this.completed_ui(ui, &steps, &cx, steps.len());
                        this.review_ui(ui, &cx)
                    }
                    Phase::Done(message) => done_ui(ui, &message, this.spec.again_question()),
                }
            };
            if wide {
                ui.columns(2, |columns| {
                    action = main(&mut columns[0], self);
                    let preview = self.preview_values(&steps, &cx);
                    self.spec.context(
                        &mut columns[1],
                        &cx,
                        preview.as_ref().unwrap_or(&self.values),
                    );
                });
            } else {
                action = main(ui, self);
                ui.add_space(space::L);
                let preview = self.preview_values(&steps, &cx);
                self.spec
                    .context(ui, &cx, preview.as_ref().unwrap_or(&self.values));
            }

            if let (Some(state), Some(step)) = (self.lookup.as_mut(), steps.get(self.current))
                && let Some((title, items)) = step.items(cx.store)
            {
                match lookup_dialog(ui.ctx(), title, state, &items) {
                    LookupOutcome::Selected(value) => {
                        self.lookup = None;
                        self.buffer.clone_from(&value);
                        action = Some(Action::Submit(value));
                    }
                    LookupOutcome::Cancelled => {
                        self.lookup = None;
                        self.focus = true;
                    }
                    LookupOutcome::Open => {}
                }
            }

            match action.take() {
                Some(Action::Submit(raw)) => self.submit(&raw, &steps, &cx),
                Some(Action::Back) => match self.phase {
                    Phase::Review => {
                        self.phase = Phase::Editing;
                        let last = steps.len().saturating_sub(1);
                        self.enter_step(last, &steps);
                    }
                    _ => outcome = self.step_back(&steps),
                },
                Some(Action::OpenLookup) => {
                    self.lookup = Some(LookupState {
                        query: String::new(),
                        highlighted: 0,
                    });
                }
                Some(Action::Correct) => {
                    self.phase = Phase::Editing;
                    self.enter_step(0, &steps);
                }
                other => action = other,
            }
        }
        match action {
            Some(Action::Save) => match self.spec.commit(store, today, &self.values) {
                Ok(message) => self.phase = Phase::Done(message),
                Err(error) => self.error = Some(error.to_string()),
            },
            Some(Action::Again) => self.restart(),
            Some(Action::Finish) => outcome = Outcome::Back,
            _ => {}
        }
        outcome
    }
}

impl<S: FlowSpec> FlowScreen<S> {
    fn review_ui(&mut self, ui: &mut Ui, cx: &FlowCx<'_>) -> Option<Action> {
        let mut action = None;
        Card::new().title(text::REVIEW_TITLE).show(ui, |ui| {
            let extra = self.spec.review_extra(cx, &self.values);
            if !extra.is_empty() {
                egui::Grid::new("flow_review")
                    .num_columns(2)
                    .spacing([space::L, space::XS])
                    .show(ui, |ui| {
                        for (label, value) in extra {
                            ui.label(RichText::new(label).color(color::TEXT_MUTED));
                            ui.label(RichText::new(value).heading().color(color::TEXT));
                            ui.end_row();
                        }
                    });
                ui.add_space(space::S);
            }
            ui.label(RichText::new(text::REVIEW_QUESTION).color(color::TEXT));
            if let Some(error) = &self.error {
                ui.label(RichText::new(format!("⚠ {error}")).color(color::ERROR));
            }
            ui.add_space(space::S);
            ui.horizontal(|ui| {
                if ui.add(PrimaryButton::new(text::SAVE)).clicked() {
                    action = Some(Action::Save);
                }
                if ui.add(SecondaryButton::new(text::CORRECT)).clicked() {
                    action = Some(Action::Correct);
                }
            });
        });
        key_hints(
            ui,
            &[("G", text::SAVE), ("C", text::CORRECT), ("Esc", text::BACK)],
        );
        let (save, correct, escape) = ui.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::G),
                i.consume_key(Modifiers::NONE, Key::C),
                i.consume_key(Modifiers::NONE, Key::Escape),
            )
        });
        if save {
            action = Some(Action::Save);
        } else if correct {
            action = Some(Action::Correct);
        } else if escape {
            action = Some(Action::Back);
        }
        action
    }
}

fn done_ui(ui: &mut Ui, message: &str, question: &str) -> Option<Action> {
    let mut action = None;
    Card::new().show(ui, |ui| {
        ui.add(StatusBadge::new(BadgeTone::Success, text::SAVED));
        ui.add_space(space::XS);
        ui.label(RichText::new(message).heading().color(color::TEXT));
        ui.add_space(space::S);
        ui.label(RichText::new(question).color(color::TEXT));
        ui.horizontal(|ui| {
            if ui.add(PrimaryButton::new(text::YES)).clicked() {
                action = Some(Action::Again);
            }
            if ui.add(SecondaryButton::new(text::NO)).clicked() {
                action = Some(Action::Finish);
            }
        });
    });
    key_hints(ui, &[("S", text::YES), ("N / Esc", text::NO)]);
    let (yes, no) = ui.input_mut(|i| {
        (
            i.consume_key(Modifiers::NONE, Key::S) || i.consume_key(Modifiers::NONE, Key::Enter),
            i.consume_key(Modifiers::NONE, Key::N) || i.consume_key(Modifiers::NONE, Key::Escape),
        )
    });
    if yes {
        action = Some(Action::Again);
    } else if no {
        action = Some(Action::Finish);
    }
    action
}

/// Convierte lo escrito en la forma canónica del tipo de paso.
fn normalize(kind: &StepKind, raw: &str, cx: &FlowCx<'_>) -> Result<String, String> {
    let raw = raw.trim();
    match kind {
        StepKind::Text { optional } => {
            if raw.is_empty() && !optional {
                Err(text::REQUIRED.into())
            } else {
                Ok(raw.to_owned())
            }
        }
        StepKind::Integer { min, max } => {
            let value: u32 = raw.parse().map_err(|_| text::NOT_A_NUMBER.to_owned())?;
            if value < *min || value > *max {
                Err(format!("Escriba un número entre {min} y {max}."))
            } else {
                Ok(value.to_string())
            }
        }
        StepKind::Money => {
            let value = Money::parse(raw).map_err(|e| e.to_string())?;
            if value.is_positive() {
                Ok(value.as_pesos().to_string())
            } else {
                Err(text::MUST_BE_POSITIVE.into())
            }
        }
        StepKind::Rate => {
            let value = Rate::parse(raw).map_err(|e| e.to_string())?;
            if value.hundredths() > 0 {
                Ok(value.hundredths().to_string())
            } else {
                Err(text::MUST_BE_POSITIVE.into())
            }
        }
        StepKind::Weight { optional } => {
            if raw.is_empty() && *optional {
                return Ok(String::new());
            }
            Weight::parse(raw)
                .map(|w| w.as_milligrams().to_string())
                .map_err(|e| e.to_string())
        }
        StepKind::Date => parse_date(raw, cx.today).map(|d| d.format("%Y-%m-%d").to_string()),
        StepKind::Choice(options) => options
            .iter()
            .find(|(value, _)| *value == raw)
            .map(|(value, _)| (*value).to_owned())
            .ok_or_else(|| text::CHOOSE_OPTION.to_owned()),
        StepKind::YesNo => match raw {
            "S" | "N" => Ok(raw.to_owned()),
            _ => Err(text::ANSWER_YES_NO.into()),
        },
        StepKind::Lookup { source, optional } => {
            if raw.is_empty() && *optional {
                return Ok(String::new());
            }
            if raw.is_empty() {
                return Err(text::REQUIRED_LOOKUP.into());
            }
            source
                .name(cx.store, raw)
                .map(|_| raw.to_owned())
                .ok_or_else(|| format!("No existe «{raw}». {}", text::PRESS_F1))
        }
    }
}

/// Fecha escrita como dd/mm/aaaa (también con guiones) o «hoy».
fn parse_date(raw: &str, today: NaiveDate) -> Result<NaiveDate, String> {
    if raw.is_empty() || raw.eq_ignore_ascii_case("hoy") {
        return Ok(today);
    }
    let normalized = raw.replace('-', "/");
    NaiveDate::parse_from_str(&normalized, "%d/%m/%Y").map_err(|_| text::DATE_FORMAT.to_owned())
}

/// Texto que aparece en el campo para un valor canónico.
fn input_text(kind: &StepKind, canonical: &str) -> String {
    if canonical.is_empty() {
        return String::new();
    }
    match kind {
        StepKind::Money => canonical
            .parse()
            .map(|v| Money::pesos(v).to_string().replace('$', ""))
            .unwrap_or_default(),
        StepKind::Rate => canonical
            .parse()
            .map(|v| {
                Rate::from_hundredths(v)
                    .to_string()
                    .trim_end_matches(" %")
                    .to_owned()
            })
            .unwrap_or_default(),
        StepKind::Weight { .. } => canonical
            .parse()
            .map(|v| {
                Weight::milligrams(v)
                    .to_string()
                    .trim_end_matches(" g")
                    .to_owned()
            })
            .unwrap_or_default(),
        StepKind::Date => NaiveDate::parse_from_str(canonical, "%Y-%m-%d")
            .map(|d| d.format("%d/%m/%Y").to_string())
            .unwrap_or_default(),
        _ => canonical.to_owned(),
    }
}

/// Texto legible de un valor canónico.
fn display(kind: &StepKind, canonical: &str, cx: &FlowCx<'_>) -> String {
    if canonical.is_empty() {
        return String::from("—");
    }
    match kind {
        StepKind::Money => canonical
            .parse()
            .map(|v| Money::pesos(v).to_string())
            .unwrap_or_default(),
        StepKind::Rate => canonical
            .parse()
            .map(|v| Rate::from_hundredths(v).to_string())
            .unwrap_or_default(),
        StepKind::Weight { .. } => canonical
            .parse()
            .map(|v| Weight::milligrams(v).to_string())
            .unwrap_or_default(),
        StepKind::Date => input_text(kind, canonical),
        StepKind::Choice(options) => options
            .iter()
            .find(|(v, _)| *v == canonical)
            .map(|(_, l)| (*l).to_owned())
            .unwrap_or_default(),
        StepKind::YesNo => if canonical == "S" {
            text::YES
        } else {
            text::NO
        }
        .to_owned(),
        StepKind::Lookup { source, .. } => source.name(cx.store, canonical).map_or_else(
            || canonical.to_owned(),
            |name| format!("{canonical} · {name}"),
        ),
        StepKind::Text { .. } | StepKind::Integer { .. } => canonical.to_owned(),
    }
}

/// Agrupa los miles mientras se escribe un importe («1210000» → «1.210.000»)
/// sin mover el cursor respecto de las cifras que tiene a su derecha.
fn regroup_thousands(ctx: &egui::Context, id: egui::Id, buffer: &mut String) {
    let digits: String = buffer.chars().filter(char::is_ascii_digit).collect();
    let trimmed = digits.trim_start_matches('0');
    let grouped = if digits.is_empty() {
        String::new()
    } else if trimmed.is_empty() {
        String::from("0")
    } else {
        trimmed.parse::<i64>().map_or_else(
            |_| digits.clone(),
            |value| Money::pesos(value).to_string().replace('$', ""),
        )
    };
    if grouped == *buffer {
        return;
    }
    let Some(mut state) = TextEdit::load_state(ctx, id) else {
        *buffer = grouped;
        return;
    };
    let cursor = state
        .cursor
        .char_range()
        .map_or(buffer.chars().count(), |range| range.primary.index.0);
    let digits_after = buffer
        .chars()
        .skip(cursor)
        .filter(char::is_ascii_digit)
        .count();
    *buffer = grouped;
    let chars: Vec<char> = buffer.chars().collect();
    let mut position = chars.len();
    let mut remaining = digits_after;
    while remaining > 0 && position > 0 {
        position -= 1;
        if chars[position].is_ascii_digit() {
            remaining -= 1;
        }
    }
    state
        .cursor
        .set_char_range(Some(CCursorRange::one(CCursor::new(position))));
    state.store(ctx, id);
}
