# Sistema de diseño — POS Pawnshop

Estado: **Fase 1 completada**, pendiente de revisión visual del usuario.
Fecha: 2026-11-21. Código: `crates/pos_desktop/src/theme/` y
`crates/pos_desktop/src/components/`.

La referencia visual viva es la pantalla **Catálogo de componentes**, que muestra
cada componente en sus estados. Las capturas se regeneran con:

```
cargo test -p pos_desktop --test review -- --ignored --nocapture
```

## 1. Tokens

Todos los valores viven en `theme/tokens.rs`. Las vistas y los componentes usan
siempre los tokens, nunca valores sueltos.

### Color

| Token | Valor | Uso |
|---|---|---|
| `OLIVE_BLACK` | `#11120d` | Texto principal, acción principal, futuro sidebar |
| `OLIVE_GRAY` | `#565449` | Texto secundario, bordes de controles, foco |
| `BEIGE` | `#D8CFBC` | Selección, botón secundario, separadores decorativos |
| `IVORY` | `#FFFBF4` | Fondo de contenido, campos, texto sobre oscuro |
| `BEIGE_SOFT` | `#ECE5D8` | Mezcla 50 % beige y 50 % marfil: tarjetas y filas alternas |
| `BEIGE_DEEP` | `#B8B09F` | Mezcla 75 % beige y 25 % gris: botón secundario bajo el puntero |

Colores semánticos (**propuesta pendiente de aprobación**):

| Token | Texto | Fondo | Contraste texto/marfil | Texto/fondo |
|---|---|---|---|---|
| Error | `#9A3324` | `#F3DED7` | 7.09:1 | 5.65:1 |
| Advertencia | `#7A5A0C` | `#F1E6C6` | 6.18:1 | 5.12:1 |
| Éxito | `#3F6430` | `#DFE8D3` | 6.62:1 | 5.40:1 |

Reglas de contraste (ADR-0004), comprobadas por pruebas en `tokens.rs`:

- Nunca texto gris oliva sobre negro oliva (2.47:1), ni beige sobre marfil (1.50:1).
- El beige no puede ser el único borde de un control sobre marfil.
- Los colores semánticos no se usan como texto sobre beige: van sobre marfil o
  sobre su propio fondo.

### Tipografía

Fuente proporcional Ubuntu Light (incluida en egui). Base 14 pt con proporción 1.125.

| Estilo | Tamaño | Uso |
|---|---|---|
| `Title` (propio) | 22 pt | Título de pantalla |
| `Heading` | 18 pt | Título de tarjeta o sección |
| `Body`, `Button` | 14 pt | Texto general y botones |
| `Monospace` | 13 pt | Códigos e identificadores |
| `Small` | 12 pt | Ayudas, errores de campo, indicadores. Nunca contenido principal |

Como máximo tres o cuatro tamaños por pantalla.

Iconos: egui 0.37 ya no incluye fuentes de emoji por defecto; se activó la
opción `monochrome_emoji_fonts`. **Antes de usar un icono nuevo, comprobar que
existe en las fuentes incluidas**: las pruebas fallan si falta un glifo y la
aplicación mostraría un cuadro vacío.

### Espaciado, medidas y radios

| Token | Valores |
|---|---|
| `space` | XS 4 · S 8 · M 12 · L 16 · XL 24 · XXL 32 |
| `margin` | Los mismos pasos en enteros, para marcos y paneles |
| `size::CONTROL_HEIGHT` | 32: alto de botones, campos y selectores |
| `size::TABLE_ROW_HEIGHT` / `TABLE_HEADER_HEIGHT` | 30 / 32 |
| `radius` | Control 4 · Tarjeta 6 · Diálogo 8 · Indicador 10 |

## 2. Estados de interacción

| Estado | Aspecto |
|---|---|
| Normal | Botón secundario beige con borde gris; campo marfil con borde gris |
| Puntero encima | Botón secundario `BEIGE_DEEP` con borde negro; principal pasa a gris oliva |
| Foco de teclado | Botón secundario gris oliva con texto marfil; principal con anillo negro exterior; campo con borde negro de 2 px |
| Deshabilitado | Atenuado por egui, con texto que explica el motivo al pasar el puntero |
| Error | Mensaje bajo el campo con «⚠» y color de error |
| Carga | Indicador animado con texto; sólo visible mientras hay trabajo |

egui usa el estilo `active` también para el foco. Por eso no hay que usar
`RichText::strong()` ni `ui.strong()`: toman ese color, que en este tema es marfil.

## 3. Componentes

| Componente | Propósito | Propiedades | Devuelve |
|---|---|---|---|
| `PrimaryButton` | Única acción principal de un grupo | texto | `Response` |
| `SecondaryButton` | Acciones secundarias | texto | `Response` |
| `FormField` | Etiqueta, control, ayuda y error con disposición fija | etiqueta, obligatorio, ayuda, error | `Response` del control |
| `single_line` | Campo de texto de una línea con alto y alineación del sistema | texto, `id_salt` | `TextEdit` configurable |
| `SearchField` | Búsqueda con botón «Limpiar» siempre presente | consulta, `id_salt`, ayuda, ancho | `Response` (`changed` al escribir o limpiar) |
| `selector` | Lista desplegable de valores fijos | `id_salt`, valor, opciones, ancho | `Response` (`changed` al elegir) |
| `Card` | Agrupa contenido relacionado | título opcional | `InnerResponse` |
| `DataTable` | Tabla con filas virtualizadas, selección y estado vacío | columnas, filas, seleccionada, alto máximo, estado vacío | `TableOutcome` |
| `StatusBadge` | Estado con icono, texto y color | tono, texto | `Response` |
| `EmptyState` | Explica la ausencia de contenido y ofrece el siguiente paso | título, icono, descripción, acción | `bool` (acción pulsada) |
| `LoadingIndicator` | Trabajo en curso | texto | `Response` |
| `AppModal` | Diálogo modal con título | id global, título, ancho | `ModalOutcome` |
| `ConfirmDialog` | Confirmación de una acción | id global, título, mensaje, texto de la acción | `Option<ConfirmChoice>` |

Reglas de uso:

- **Accesibilidad**: todo control necesita nombre accesible. `selector` y los
  campos de texto van dentro de un `FormField`, que los asocia a su etiqueta;
  las pruebas de kittest fallan si un control queda sin nombre.
- **Botones de diálogo**: orden de Windows, con la acción a la izquierda y
  «Cancelar» a la derecha. El botón nombra la acción («Eliminar elemento»),
  nunca «Aceptar» ni «Sí».
- **Tablas**: la selección se marca con fondo beige y «▶»; dentro de las celdas
  la selección de texto está desactivada para que el clic llegue a la fila.
- **Identificadores**: los componentes con estado reciben un `id_salt` único en
  su pantalla; los diálogos, un id único en toda la aplicación.
- **Textos**: todos en `strings.rs`. Al cambiar un texto, actualizar las pruebas
  que lo buscan.

## 4. Pendiente

- Aprobar o ajustar los colores semánticos.
- Componentes de la Fase 2: `AppSidebar` (con el estilo oscuro acotado al
  panel) y `AppHeader`.

## 5. Actualización: referencia visual aprobada (2026-11-21, ADR-0007)

- **Tipografía**: Segoe UI de Windows (regular, semibold, negrita) reemplaza a
  Ubuntu Light, que queda como respaldo. Tamaños: cuerpo 15, encabezado 18,
  título de página 26 en negrita, nombre del negocio 24 en negrita.
- **Iconos**: Phosphor «Fill», en `theme/icons.rs`. Se escriben dentro del texto.
- **Colores nuevos** (mezclas de la paleta): `OLIVE_DARK` `#26261F` para la barra
  de pestañas. Roles nuevos: `CHROME`, `CHROME_RAISED`, `CHROME_HOVER`,
  `TEXT_ON_DARK_MUTED` e `ICON_BG`, todos con su contraste comprobado en pruebas.
- **Componentes nuevos**:

| Componente | Propósito |
|---|---|
| `OptionCard` | Tarjeta-botón con icono en círculo, título y descripción; atenuada si la opción está sin identificar |
| `SectionTab` | Pestaña de la barra oscura; la seleccionada toma el color del contenido |
| `PageHeader` | Icono en círculo negro, título, subtítulo y línea separadora |
| `info_banner` | Aviso de una línea con icono de información |
| `brand_mark` | Diamante vectorial del logotipo |

- `theme::use_chrome_style(ui)` adapta los controles a las zonas oscuras.
- `paint::focus_ring` y `paint::icon_circle` son el dibujo compartido de los
  widgets propios.
