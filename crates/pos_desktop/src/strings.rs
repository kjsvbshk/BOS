//! Textos visibles de la aplicación, en un único lugar.
//!
//! Los textos también son el nombre accesible de cada control y lo que buscan
//! las pruebas de interfaz: al cambiar uno, hay que actualizar sus pruebas.

/// Nombre visible de la aplicación (título de la ventana).
pub const APP_TITLE: &str = "Compraventa 3 Diamantes";

/// Encabezado de la aplicación.
pub mod header {
    pub const SYSTEM_NAME: &str = "Sistema de Gestión de Empeños";
    pub const SIGN_OUT: &str = "Cambiar de usuario";
    pub const DEMO_SESSION: &str = "Sesión de demostración: sin contraseñas.";
    pub const COMPONENT_GALLERY: &str = "Catálogo de componentes (desarrollo)";
    /// Abreviaturas de los meses para la fecha del encabezado.
    pub const MONTHS: [&str; 12] = [
        "Ene.", "Feb.", "Mar.", "Abr.", "May.", "Jun.", "Jul.", "Ago.", "Sep.", "Oct.", "Nov.",
        "Dic.",
    ];
}

/// Inicio de sesión simulado.
pub mod sign_in {
    pub const TITLE: &str = "Iniciar sesión";
    pub const SUBTITLE: &str = "Elija su usuario. En la demo no se piden contraseñas.";
    pub const MOVE: &str = "Moverse";
    pub const ENTER: &str = "Entrar";
    pub const PICK: &str = "Elegir por número";
}

/// Navegación y pantallas provisionales.
pub mod navigation {
    pub const EXIT: &str = "Salir";
    pub const EXIT_TITLE: &str = "Salir de la aplicación";
    pub const EXIT_MESSAGE: &str = "Se cerrará Compraventa 3 Diamantes.";
    pub const SELECT_OPTION_HINT: &str = "Seleccione una opción del menú para continuar.";
    pub const CHOOSE_ACTION: &str = "Seleccione la acción que desea realizar.";
    pub const AWAITING_SPECIFICATION: &str = "Pendiente de especificación";
    pub const UNIDENTIFIED: &str = "Opción por identificar";
    pub const SCREEN_PENDING_TITLE: &str = "Pantalla en preparación";
    pub const SCREEN_PENDING_DESCRIPTION: &str = "Esta pantalla se construirá cuando se documente \
        su proceso a partir del sistema anterior.";
    pub const UNIDENTIFIED_DESCRIPTION: &str = "Falta identificar qué opción del sistema anterior \
        ocupa este lugar del menú.";
    pub const SECTION_PENDING_TITLE: &str = "Opciones pendientes de levantamiento";
    pub const RESTRICTED_TITLE: &str = "Acceso restringido";
    pub const SELECT_BY_NUMBER: &str = "Seleccionar por número";
    pub const MOVE_BETWEEN: &str = "Moverse";
    pub const OPEN: &str = "Abrir";
    /// Separador de la ruta de navegación («Operaciones › Contratos»).
    pub const BREADCRUMB_SEPARATOR: &str = "  ›  ";

    /// Explicación del estado vacío de una sección sin opciones.
    #[must_use]
    pub fn section_pending_description(section: &str) -> String {
        format!(
            "Las opciones de {section} se agregarán cuando se identifiquen en el sistema anterior."
        )
    }

    /// Texto del botón para volver a la pantalla anterior.
    #[must_use]
    pub fn back_to(destination: &str) -> String {
        format!("Volver a {destination}")
    }
}

/// Marca de campo obligatorio.
pub const REQUIRED_MARK: &str = "*";
/// Explicación de la marca de campo obligatorio.
pub const REQUIRED_HINT: &str = "Campo obligatorio";

/// Componentes genéricos.
pub mod common {
    pub const SEARCH_HINT: &str = "Buscar…";
    pub const SEARCH_LABEL: &str = "Escriba para filtrar";
    pub const SUGGESTIONS: &str = "Mostrando";
    pub const SUGGESTION_KEYS: &str = "↑ ↓ elegir · F1 ver todos";
    pub const SEARCH_CLEAR: &str = "Limpiar";
    pub const SEARCH_CLEAR_DISABLED: &str = "No hay texto que limpiar";
    pub const CANCEL: &str = "Cancelar";
    pub const LOADING: &str = "Cargando…";
    pub const NO_MATCHES: &str = "Ningún elemento coincide con la búsqueda.";
    pub const LOOKUP_KEYS: &str = "↑ ↓ para moverse · Enter para elegir · Esc para cerrar";
    pub const PRINT: &str = "Imprimir";
    pub const PRINT_UNAVAILABLE: &str = "La impresión todavía no está disponible en la demo.";
    pub const DEMO_DATA: &str = "Datos de demostración";
}

/// Motor de captura por pasos.
pub mod flow {
    pub const STEP: &str = "Paso";
    pub const OF: &str = "de";
    pub const CONTINUE: &str = "Continuar";
    pub const BACK: &str = "Atrás";
    pub const SEARCH: &str = "Buscar";
    /// Botón de búsqueda con lupa.
    pub const SEARCH_ICON: &str = "\u{E30C}  Buscar (F1)";
    pub const SEARCH_ALL: &str = "Ver todos";
    pub const CHOOSE_SUGGESTION: &str = "Elegir sugerencia";
    pub const CHOOSE: &str = "Elegir";
    pub const ANSWER: &str = "Responder";
    pub const YES: &str = "Sí";
    pub const NO: &str = "No";
    pub const SAVE: &str = "Grabar";
    pub const CORRECT: &str = "Corregir";
    pub const SAVED: &str = "Operación grabada";
    pub const REVIEW_TITLE: &str = "Revise la información";
    pub const REVIEW_QUESTION: &str = "¿Graba la operación o corrige algún dato?";
    pub const REQUIRED: &str = "Escriba un valor.";
    pub const REQUIRED_LOOKUP: &str = "Escriba un código o pulse F1 para consultar.";
    pub const PRESS_F1: &str = "Pulse F1 para consultar.";
    pub const NOT_A_NUMBER: &str = "Escriba un número entero.";
    pub const MUST_BE_POSITIVE: &str = "El valor debe ser mayor que cero.";
    pub const CHOOSE_OPTION: &str = "Elija una de las opciones.";
    pub const ANSWER_YES_NO: &str = "Responda S (sí) o N (no).";
    pub const DATE_FORMAT: &str = "Escriba la fecha como dd/mm/aaaa, por ejemplo 18/11/2026.";
    pub const LOOKUP_CUSTOMERS: &str = "Clientes";
    pub const LOOKUP_MERCHANDISE: &str = "Códigos de mercancía";
    pub const LOOKUP_GOLD: &str = "Calidades de oro";
    pub const LOOKUP_CONCEPTS: &str = "Conceptos de caja y banco";
}

/// Catálogo de componentes (vista interna de la Fase 1).
pub mod gallery {
    pub const SUBTITLE: &str = "Catálogo de componentes";
    pub const INTERNAL_BADGE: &str = "Uso interno";
    pub const INTRO: &str = "Referencia visual de los componentes base en todos sus estados. \
        Los textos y registros son ejemplos y no representan operaciones del negocio.";

    pub const TOGGLE_ERRORS: &str = "Mostrar errores de validación";
    pub const TOGGLE_DISABLED: &str = "Deshabilitar controles";
    pub const TOGGLE_LOADING: &str = "Mostrar indicador de carga";

    pub const SECTION_BUTTONS: &str = "Botones";
    pub const PRIMARY_ACTION: &str = "Guardar ejemplo";
    pub const SECONDARY_ACTION: &str = "Acción secundaria";
    pub const DISABLED_REASON: &str = "Controles deshabilitados desde el panel de opciones";
    pub const SAVED_MESSAGE: &str = "Ejemplo guardado (simulación).";
    pub const SECONDARY_MESSAGE: &str = "Acción secundaria ejecutada (simulación).";

    pub const SECTION_FORM: &str = "Campos de formulario";
    pub const FIELD_NAME: &str = "Nombre de ejemplo";
    pub const FIELD_NAME_HINT: &str = "Escriba un nombre";
    pub const FIELD_NAME_HELP: &str = "Texto libre de hasta 60 caracteres.";
    pub const FIELD_NAME_ERROR: &str = "Escriba un nombre para continuar.";
    pub const FIELD_NOTES: &str = "Notas";
    pub const FIELD_NOTES_HELP: &str = "Opcional.";
    pub const FIELD_CATEGORY: &str = "Categoría de ejemplo";
    pub const CATEGORY_A: &str = "Categoría A";
    pub const CATEGORY_B: &str = "Categoría B";
    pub const CATEGORY_C: &str = "Categoría C";

    pub const SECTION_STATUS: &str = "Indicadores de estado";
    pub const STATUS_NEUTRAL: &str = "Neutral";
    pub const STATUS_PENDING: &str = "Pendiente de definición";
    pub const STATUS_SUCCESS: &str = "Completado";
    pub const STATUS_WARNING: &str = "Requiere revisión";
    pub const STATUS_ERROR: &str = "Con error";
    pub const SEMANTIC_NOTE: &str = "Los colores de éxito, advertencia y error son una propuesta \
        pendiente de aprobación.";

    pub const SECTION_TABLE: &str = "Tabla";
    pub const COLUMN_CODE: &str = "Código";
    pub const COLUMN_DESCRIPTION: &str = "Descripción";
    pub const COLUMN_STATUS: &str = "Estado";
    pub const DELETE_SELECTED: &str = "Eliminar seleccionado";
    pub const DELETE_DISABLED: &str = "Seleccione una fila de la tabla";
    pub const RESTORE_ROWS: &str = "Restablecer ejemplos";
    pub const NO_MATCHES_TITLE: &str = "Sin resultados";
    pub const NO_MATCHES_DESCRIPTION: &str = "Ningún elemento coincide con la búsqueda.";
    pub const NO_ROWS_TITLE: &str = "No quedan elementos";
    pub const NO_ROWS_DESCRIPTION: &str = "Se eliminaron todos los elementos de ejemplo.";

    pub const SECTION_EMPTY: &str = "Estado vacío y carga";
    pub const EMPTY_TITLE: &str = "Todavía no hay elementos";
    pub const EMPTY_DESCRIPTION: &str = "Cuando se registre el primer elemento aparecerá aquí.";
    pub const EMPTY_ACTION: &str = "Crear elemento de ejemplo";
    pub const EMPTY_ACTION_MESSAGE: &str = "Se pulsó la acción del estado vacío (simulación).";

    pub const SECTION_DIALOGS: &str = "Diálogos";
    pub const OPEN_DIALOG: &str = "Abrir diálogo de confirmación";
    pub const CONFIRM_TITLE: &str = "Eliminar elemento de ejemplo";
    pub const CONFIRM_DELETE: &str = "Eliminar elemento";
    pub const DELETED_MESSAGE: &str = "Elemento de ejemplo eliminado (simulación).";
    pub const CANCELLED_MESSAGE: &str = "Eliminación cancelada.";

    /// Mensaje del diálogo de confirmación para un elemento concreto.
    #[must_use]
    pub fn confirm_message(code: &str) -> String {
        format!(
            "Se eliminará el elemento {code} de la lista de ejemplos. Puede restablecerlos después."
        )
    }

    /// Nombre de un elemento de ejemplo.
    #[must_use]
    pub fn sample_description(number: usize) -> String {
        format!("Elemento de ejemplo {number}")
    }
}
