# Arquitectura inicial — POS Pawnshop

Estado: **propuesta (Fase 0)**, pendiente de validación. Fecha: 2026-11-21.

Este documento describe la arquitectura con la que se construirá la demo y que
debe sostener después los módulos reales. Las decisiones que la justifican están
en [`decisions/`](decisions/README.md); el plan de trabajo, en
[`roadmap.md`](roadmap.md).

## 1. Diagnóstico del entorno (2026-11-21)

| Elemento | Estado | Acción necesaria |
|---|---|---|
| Carpeta del proyecto | Repositorio git creado (rama `main`), sin commits todavía | Hecho |
| Ubicación | Dentro de OneDrive | `target/` movido a `C:/Users/Kjsvb/.cargo-targets/bos` (ver §7) |
| Sistema | Windows 11, `x86_64-pc-windows-msvc` | — |
| Rust | Actualizado de 1.92.0 a **1.100.0** (egui 0.37 exige ≥ 1.95) | Hecho |
| clippy / rustfmt | Instalados | Hecho |
| Compilador C++ (MSVC) | Visual Studio 2022 Community con herramientas VC | — |
| Windows SDK | 10.0.22621, 10.0.26100, 10.0.28000 | — |
| egui / eframe | Última versión publicada: **0.37.0** | Ver [ADR-0002](decisions/0002-version-egui.md) |
| Código y base de datos del POS anterior | No disponibles todavía | No bloquea la demo; necesarios en la Fase 6 |

## 2. Principios

1. **La interfaz no contiene reglas de negocio ni acceso a datos.** Se garantiza
   con crates separados: el compilador impide que el dominio dependa de egui.
2. **Nada se inventa.** Los nombres de módulos, campos, procesos e indicadores
   que no estén especificados aparecen como *pendientes de definición*.
3. **La navegación es datos, no condicionales.** Agregar, quitar o reordenar una
   opción se hace en un único catálogo.
4. **Los datos simulados están aislados** y se ven como tales en pantalla.
5. **Sin dinero en coma flotante.** `f32`/`f64` están prohibidos para importes
   (ver [ADR-0005](decisions/0005-representacion-monetaria.md)).

## 3. Capas y crates

Se usa un *workspace* de Cargo. Las cuatro capas conceptuales del plan maestro se
mapean así:

| Capa | Dónde vive | Depende de | Contenido |
|---|---|---|---|
| Presentation | crate `pos_desktop` (binario) | `pos_core`, `eframe` | Tema, layout, componentes, navegación, vistas |
| Application | crate `pos_core`, módulo `application` | `domain` | Casos de uso y *puertos* (traits) hacia los datos |
| Domain | crate `pos_core`, módulo `domain` | nada | Entidades, reglas y validaciones (vacío durante la demo) |
| Infrastructure | crate `pos_infrastructure` (**se crea en la Fase 6**) | `pos_core` | Persistencia, migración, archivos |

Reglas de dependencia (de arriba abajo, nunca al revés):

```
pos_desktop ──► pos_core::application ──► pos_core::domain
     │                    ▲
     └──► (Fase 6) pos_infrastructure implementa los puertos de application
```

- `pos_core` **no** depende de `egui`, `eframe` ni de ninguna base de datos.
- `pos_core` contiene `domain` (valores sin coma flotante, entidades, fórmulas
  provisionales), `application` (almacén en memoria, operaciones e informes) y
  `demo` (datos de ejemplo, feature `demo-data`). Ver ADR-0008.
- `pos_desktop` sólo llama a `pos_core::application`; nunca toca la
  infraestructura directamente salvo para construir las implementaciones en `main`.
- Domain y application comparten crate mientras sean pequeños; se separarán en
  crates propios cuando el dominio crezca (decisión revisable, ver
  [ADR-0001](decisions/0001-workspace-y-capas.md)).

### Datos simulados

La demo necesita datos para tablas y paneles. Se modelan como una implementación
de un puerto de `application` dentro de `pos_core::demo`, detrás de la *feature*
de Cargo `demo-data`:

- Ningún código de producción puede usar esos datos sin activar la *feature*.
- Toda vista que los muestre lleva una marca visible **«Datos de demostración»**.
- Cuando exista la infraestructura real, se sustituye la implementación sin
  tocar la interfaz. Esto valida la arquitectura desde la demo.

## 4. Estructura del proyecto

Sólo se crean los archivos que la fase en curso necesita; esta es la forma
esperada al terminar la demo (Fases 1–5):

```
bos/
├── Cargo.toml                 # [workspace], dependencias y lints compartidos
├── Cargo.lock
├── rust-toolchain.toml        # fija el canal estable y los componentes
├── rustfmt.toml
├── .gitignore
├── README.md
├── docs/
│   ├── architecture.md        # este documento
│   ├── design-system.md       # Fase 1
│   ├── roadmap.md
│   └── decisions/             # ADRs
├── assets/                    # sólo si se incorpora una fuente o iconos propios
└── crates/
    ├── pos_core/
    │   └── src/
    │       ├── lib.rs
    │       ├── domain/mod.rs
    │       ├── application/   # puertos y servicios
    │       └── demo/          # feature "demo-data"
    └── pos_desktop/
        ├── src/
        │   ├── main.rs        # sólo arranque: opciones de ventana y run_native
        │   ├── app.rs         # struct de la app, eframe::App
        │   ├── state.rs       # estado de UI: ruta actual, modal abierto…
        │   ├── navigation/    # catálogo, rutas, registro de vistas
        │   ├── theme/         # tokens de color, tipografía, espaciado
        │   ├── components/    # sidebar, header, tabla, botones, modal…
        │   ├── strings.rs     # textos visibles centralizados (español)
        │   └── views/         # dashboard, placeholder, vista de demostración
        └── tests/             # pruebas de navegación con egui_kittest
```

## 5. Modelo de navegación

El catálogo es la única fuente de verdad de secciones y opciones:

```rust
pub enum SectionId { Operaciones, Mantenimiento, Informes }

pub struct OptionId(&'static str);          // "operaciones.01" … "operaciones.11"

pub enum OptionStatus { PendienteDeDefinicion, Disponible }

pub struct NavOption  { id: OptionId, label: &'static str, status: OptionStatus }
pub struct NavSection { id: SectionId, label: &'static str, options: &'static [NavOption] }

pub enum Route { Inicio, Seccion(SectionId), Opcion(OptionId) }
```

- **Operaciones**: 11 entradas con identificadores estables y etiquetas
  provisionales («Operación 1» … «Operación 11») marcadas *Pendiente de definición*.
  Al recibir los nombres reales sólo cambian las etiquetas.
- **Mantenimiento** e **Informes**: sin opciones. Al seleccionarlas se muestra un
  estado vacío: «Opciones pendientes de levantamiento».
- Las vistas se registran por `OptionId` en un registro
  (`OptionId → Box<dyn ModuleView>`). Una opción sin vista registrada muestra la
  vista provisional genérica. Se usa `dyn` porque el registro guarda vistas de
  tipos distintos.
- La ruta actual vive en el estado de la app; el sidebar y el header la leen,
  nunca la deducen de condicionales.

## 6. Interfaz

- **egui inmediato**: el estado vive en structs; las vistas lo toman prestado.
  Nada bloqueante dentro del frame; trabajo lento en hilos con
  `request_repaint` al terminar.
- **Layout**: `Panel::left` (sidebar oscuro, redimensionable con mínimo y
  máximo), `Panel::top` (header con título, contexto y acciones), y
  `CentralPanel` al final. Contenido largo dentro de `ScrollArea`; tablas
  largas con filas virtualizadas.
- **Tema**: un único esquema claro con la paleta oficial y el sidebar oscuro
  mediante un estilo acotado al panel (ver [ADR-0004](decisions/0004-tema-y-contraste.md)).
  Los colores salen de tokens centralizados en `theme/`, nunca de valores
  sueltos en las vistas.
- **Textos**: en español, centralizados en `strings.rs`; sin tamaños fijos en
  widgets de texto.
- **Accesibilidad**: navegación completa con teclado y foco visible, contraste
  WCAG 2.2 AA, botones con texto (no sólo icono), AccessKit activo.

## 7. Convenciones

- **Código** en inglés (tipos, funciones, módulos); **interfaz y documentación**
  en español.
- **Toolchain** estable fijada en `rust-toolchain.toml`; edición 2024.
- **Lints** de workspace: `clippy::all` como error, `clippy::pedantic` como
  aviso, `unwrap_used` y `expect_used` prohibidos fuera de pruebas, y
  `float_arithmetic` prohibido en `pos_core`. Excepciones sólo con
  `#[expect(...)]` y su justificación.
- **Errores**: `thiserror` en `pos_core` cuando haga falta; nunca `panic!` ante
  datos del usuario.
- **Formato**: `cargo fmt`; imports agrupados std → externos → workspace → crate.
- **Pruebas**: unitarias junto al código; navegación y componentes con
  `egui_kittest`.
- **Carpeta `target/`**: fuera de OneDrive (por ejemplo con
  `CARGO_TARGET_DIR` o un `.cargo/config.toml` local), porque la sincronización
  de miles de archivos de compilación bloquea archivos y ralentiza el equipo.

## 8. Decisiones pendientes

| Tema | Bloquea | Cuándo |
|---|---|---|
| Actualizar Rust e instalar clippy/rustfmt | Fase 1 | Antes de empezar |
| Renderizador (glow u OpenGL frente a wgpu) según el hardware de las sucursales | Fase 1 (sólo la configuración) | Antes de la Fase 1 |
| Resolución mínima de pantalla de los equipos | Fase 2 | Antes de la Fase 2 |
| Colores semánticos (error, advertencia, éxito) | Indicadores de estado (Fase 1) | Antes de la Fase 1 |
| Nombres reales de las 11 operaciones (captura del menú actual) | Sólo etiquetas | Cuando estén disponibles |
| Motor de base de datos y estrategia de persistencia | Fase 6 | Tras analizar el sistema anterior |
| Representación decimal (`rust_decimal` o enteros en centavos) | Fase 6 | Tras ver los tipos de la base anterior |
| Instalador y distribución (incluido enlazar el runtime de C estático) | Después de la demo | Fase 6 o posterior |
