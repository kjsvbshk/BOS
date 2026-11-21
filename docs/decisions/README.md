# Registro de decisiones arquitectónicas (ADR)

Cada decisión relevante se registra en un archivo propio con: contexto, decisión,
alternativas descartadas y consecuencias. Una decisión aceptada no se edita; si
cambia, se escribe un ADR nuevo que la reemplaza y se actualiza su estado aquí.

| N.º | Decisión | Estado |
|---|---|---|
| [0001](0001-workspace-y-capas.md) | Workspace de Cargo con capas separadas en crates | Aceptada |
| [0002](0002-version-egui.md) | Versión de egui/eframe y toolchain de Rust | Aceptada |
| [0003](0003-renderizador.md) | Renderizador gráfico (glow o wgpu) | Aceptada (revisable con datos del hardware) |
| [0004](0004-tema-y-contraste.md) | Tema único claro y reglas de contraste de la paleta | Aceptada (colores semánticos pendientes) |
| [0005](0005-representacion-monetaria.md) | Prohibición de coma flotante para importes | Aceptada |
| [0006](0006-navegacion-como-catalogo.md) | Navegación definida como catálogo de datos | Aceptada |
| [0007](0007-referencia-visual-y-navegacion-superior.md) | Referencia visual aprobada y navegación por pestañas superiores (sin sidebar) | Aceptada |
| [0008](0008-menu-teclado-y-formulas-provisionales.md) | Menú corregido, captura por teclado, datos en memoria y fórmulas provisionales | Aceptada |

Estados posibles: *Propuesta*, *Aceptada*, *Reemplazada por NNNN*, *Descartada*.
