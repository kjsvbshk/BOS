# Plan de implementación de la demo

Estado: **Fase 0 aprobada y Fase 1 completada** (2026-11-21). Siguiente: Fase 2.

Cada fase termina con el proyecto compilando, `cargo fmt --check` y
`cargo clippy` sin avisos, y las pruebas en verde. No se avanza a la siguiente
fase si la anterior tiene problemas estructurales.

## Requisitos previos (antes de la Fase 1) — completados

Rust actualizado a 1.100.0 con clippy y rustfmt; repositorio git creado; carpeta
de compilación en `C:/Users/Kjsvb/.cargo-targets/bos` mediante
`.cargo/config.toml` (local, no versionado); ADR 0001–0006 aceptados. Sin datos
del hardware se aplicaron los valores por defecto.


| Requisito | Responsable |
|---|---|
| `rustup update` y `rustup component add clippy rustfmt` | Aprobación del usuario |
| `git init` y primer commit con `docs/` | Aprobación del usuario |
| Ubicar `target/` fuera de OneDrive | Aprobación del usuario |
| Validar los ADR 0001–0006 | Usuario |
| Datos del hardware de sucursales y resolución mínima | Usuario (si no se conocen, se usan los valores por defecto indicados abajo) |

Valores por defecto si no se reciben datos: renderizador **glow**; ventana
mínima **1280 × 720**; tamaño inicial **1366 × 768**.

## Skills de apoyo por fase

| Skill | Uso |
|---|---|
| `eframe-project` | Fase 1: workspace, `Cargo.toml`, `main.rs`, renderizador |
| `egui` | Todas: APIs, IDs, frame loop, sin bloqueos |
| `egui-ui-design` | Fases 1–4: layout, botones, textos, contraste, revisión con capturas |
| `rust-best-practices` | Todas: errores, lints, préstamos, pruebas |
| `egui-kittest` | Fases 2–5: pruebas de navegación y de componentes |
| `egui-review` | Cierre de cada fase |
| `egui-profiler` | Fase 5: consumo en reposo y fluidez |
| `sql-optimization-patterns` | Fase 6: persistencia y migración |

## Fase 1 — Esqueleto y sistema visual ✔

Resultado: workspace con `pos_core` y `pos_desktop`; tema, 13 componentes y el
catálogo de componentes; `docs/design-system.md`; 21 pruebas (contraste,
componentes y flujo completo de eliminación con confirmación); capturas
revisadas en 1280 × 720, 1366 × 768 y al 150 %; release compilado sin wgpu.
Pendiente de aprobar: colores semánticos.

**Objetivo:** proyecto compilable con el tema oficial y los componentes base.

**Actividades**

1. Crear el workspace (`Cargo.toml`, `rust-toolchain.toml`, `rustfmt.toml`,
   `.gitignore`, lints compartidos) y los crates `pos_core` y `pos_desktop`.
2. `main.rs` mínimo: título, tamaño inicial y mínimo, consola oculta en release.
3. Tokens de tema: colores, escala tipográfica (`TextStyle`), espaciados,
   radios; aplicados una sola vez en el constructor.
4. Componentes reutilizables, sin dependencia de ningún módulo de negocio:
   `PrimaryButton`, `SecondaryButton`, `FormField`, `SearchField`, selector,
   `Card`, `DataTable`, `StatusBadge`, `AppModal`, `EmptyState`.
5. Una vista interna de **catálogo de componentes** que muestra cada componente
   en todos sus estados (sirve de referencia visual y de base para las pruebas).
6. `docs/design-system.md`: tokens, componentes, propiedades y estados.

**Archivos:** raíz del workspace, `crates/pos_core/src/lib.rs`,
`crates/pos_desktop/src/{main.rs, app.rs, theme/, components/, strings.rs}`,
`docs/design-system.md`.

**Criterios de aceptación**

- La app abre en Windows 64 bits y muestra el catálogo de componentes.
- Todos los colores salen de tokens; contraste conforme al ADR-0004.
- Cada componente tiene estados normal, hover, foco, deshabilitado y, si
  aplica, error y carga.
- Prueba con `egui_kittest` por componente interactivo (clic, foco, `changed`).

## Fase 2 — Layout principal y navegación ✔ (con la referencia visual, ADR-0007)

Resultado: encabezado, pestañas de sección, «Salir» con confirmación, las 11
operaciones con sus nombres reales en tarjetas agrupadas, pantallas provisionales
por opción y por acción (opción 09), Escape para volver, y estados vacíos en
Mantenimiento e Informes. 39 pruebas. Se adelantó parte de la Fase 3 (la pantalla
de Operaciones funciona como pantalla de inicio). Lo que sigue describe el plan
original; el sidebar quedó descartado.

**Objetivo:** estructura completa de la aplicación, dirigida por el catálogo.

**Actividades**

1. Catálogo de navegación (ADR-0006) con las 3 secciones y las 11 opciones
   provisionales de Operaciones.
2. `AppSidebar`: secciones; Operaciones se despliega con sus 11 opciones;
   opción activa resaltada; colapsable en ventanas pequeñas.
3. `AppHeader`: título de la opción, ruta (sección › opción) y zona de acciones
   contextuales.
4. Registro de vistas y vista provisional genérica.
5. Estado vacío para Mantenimiento e Informes.
6. Atajos de teclado básicos (navegación y cierre de diálogos con Escape).

**Archivos:** `crates/pos_desktop/src/{navigation/, components/sidebar.rs,
components/header.rs, views/placeholder.rs, state.rs}`, `tests/navigation.rs`.

**Criterios de aceptación**

- Seleccionar cualquier opción actualiza el contenido central y el header.
- Agregar o reordenar una opción sólo requiere editar el catálogo
  (comprobado por una prueba que recorre el catálogo).
- Navegación completa con teclado y foco visible.

## Fase 3 — Pantalla de inicio

**Objetivo:** punto de entrada con accesos rápidos y espacios para indicadores.

**Actividades**

1. Encabezado de bienvenida.
2. Accesos rápidos a las secciones, generados desde el catálogo.
3. Tarjetas para indicadores con el texto «Indicador pendiente de definición»;
   **no** se inventan métricas financieras.
4. Lista de actividad reciente con datos simulados (`pos_core::demo`, feature
   `demo-data`) y la marca «Datos de demostración».

**Criterios de aceptación:** ningún dato simulado puede confundirse con real;
la pantalla funciona en la resolución mínima y al 150 % de zoom.

## Fase 4 — Vista de demostración

**Objetivo:** comprobar que el sistema visual soporta una pantalla típica de POS.

**Actividades**

1. Una vista con encabezado, búsqueda, filtros, tabla con acciones por fila,
   formulario en panel o modal, confirmación de acción destructiva, estado
   vacío y retorno a la navegación.
2. Datos a través de un puerto de `pos_core::application` implementado por
   `pos_core::demo`; la vista no sabe de dónde vienen.
3. La entidad mostrada es **genérica y está rotulada como ejemplo**; no
   representa ningún proceso real del negocio (ver pregunta abierta 3).

**Criterios de aceptación:** sin cálculos monetarios; sin reglas de negocio;
todos los componentes de la Fase 1 aparecen en uso real.

## Fase 5 — Validación técnica

1. Compilación completa en release para `x86_64-pc-windows-msvc`.
2. `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
3. Pruebas unitarias y de navegación.
4. Capturas de pantalla automáticas en resolución mínima, típica y al 150 %
   (revisión visual según `egui-ui-design`).
5. Consumo en reposo: la app no debe redibujar sin interacción (`egui-profiler`).
6. Revisión completa con `egui-review` y corrección de lo encontrado.

**Entregable:** informe breve de validación en `docs/` con resultados.

## Fase 6 — Preparación del desarrollo funcional

Después de aprobar la demo: nombres reales de módulos, análisis del POS
anterior, documentación de reglas de negocio y estructuras de datos, estrategia
de persistencia y migración, crate `pos_infrastructure`, representación
monetaria (ADR-0005) y orden de implementación de los módulos.

## Preguntas abiertas

1. **Menú actual:** una captura del menú principal del POS anterior, aunque sea
   de baja calidad. Puede revelar submenús que cambien la forma del catálogo.
2. **Equipos de sucursal:** PC, GPU, versión de Windows, resolución de pantalla y
   si se usa escritorio remoto.
3. **Vista de demostración:** ¿se acepta una entidad genérica rotulada como
   ejemplo, o se prefiere esperar a conocer una operación real para la Fase 4?
4. **Colores semánticos:** propuesta de tres tonos (error, advertencia, éxito)
   compatibles con la paleta, para aprobar antes de cerrar la Fase 1.
5. **Sistema anterior:** lenguaje, framework y motor de base de datos (sólo para
   planificar la Fase 6; no bloquea la demo).
