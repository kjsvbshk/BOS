# ADR-0002 — Versión de egui/eframe y toolchain de Rust

- Estado: Aceptada (2026-11-21)
- Fecha: 2026-11-21

## Contexto

- La última versión publicada de egui/eframe es la **0.37.0**. Las skills de
  apoyo instaladas describen la **0.36** y piden verificar cada API en el código
  fuente cuando se usa otra versión.
- Ambas versiones exigen **Rust 1.95 o superior**. El equipo tiene 1.92.0; la
  versión estable actual es 1.100.0.
- egui publica versiones con cambios incompatibles con frecuencia; todos los
  crates de egui deben ir en la misma versión.

## Decisión

1. Actualizar a Rust estable actual (`rustup update`) e instalar `clippy` y
   `rustfmt`. Fijar el canal en `rust-toolchain.toml` para que todos compilen igual.
2. Usar **egui/eframe 0.37** declarados una sola vez en
   `[workspace.dependencies]` (`egui`, `eframe`, `egui_extras`, `egui_kittest`).
3. Antes de usar una API que las skills describen para 0.36, comprobarla en el
   código fuente de 0.37 (`~/.cargo/registry/src/*/egui-0.37.*/`) o en docs.rs.
4. Comprobar con `cargo tree --duplicates` que no hay dos copias de egui.

## Alternativas descartadas

- **egui 0.36**: coincide exactamente con las skills, pero empezar un proyecto
  de larga duración una versión por detrás obliga a migrar casi de inmediato.
  Queda como plan de respaldo si 0.37 presenta un problema bloqueante.

## Diferencias encontradas entre 0.36 y 0.37 (Fase 1)

Las skills describen 0.36. Al implementar se encontró en el código fuente de 0.37:

| Tema | 0.36 (skills) | 0.37 |
|---|---|---|
| Identificadores | `Id::new(..)` | Obsoleto. `IdSalt`/`impl AsIdSalt` para ids locales; `Id::unique(..)` para ids globales |
| Tipo de widget accesible | `WidgetType` | `Role` (de AccessKit) |
| Alineación de `TextEdit` | `vertical_align` | `align(Align2)` |
| Fuentes por defecto | Incluían emoji | Sólo Ubuntu Light e iconos de egui; emoji con la opción `monochrome_emoji_fonts` o `system_fonts` |
| Glifo inexistente | Cuadro vacío | Cuadro vacío en la app; las pruebas de kittest fallan (útil para detectarlo) |
| Pruebas kittest | — | Comprueban que todo control tenga nombre accesible |

Se activó `monochrome_emoji_fonts` en egui: las fuentes van dentro del
ejecutable y el resultado no depende de las fuentes instaladas en cada equipo.

## Consecuencias

- Hay que verificar en el código fuente las APIs que cambiaron entre 0.36 y 0.37.
- Las actualizaciones futuras de egui se hacen de forma deliberada, una versión
  por vez, con su propio ADR si rompen algo visible.
