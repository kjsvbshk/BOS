# ADR-0004 — Tema único claro y reglas de contraste de la paleta

- Estado: Aceptada (2026-11-21); los colores semánticos siguen pendientes de aprobación
- Fecha: 2026-11-21

## Contexto

La paleta oficial define un único esquema: sidebar oscuro (negro oliva) y
contenido claro (marfil). egui maneja un estilo global por tema (claro u oscuro),
de modo que el sidebar oscuro no puede salir sólo del tema global.

Contraste medido entre los colores oficiales (WCAG 2.2; el texto necesita
4.5:1 y los bordes o controles 3:1):

| Par | Contraste | Texto normal | Controles y bordes |
|---|---|---|---|
| Negro oliva `#11120d` / Marfil `#FFFBF4` | 18.24:1 | Sí | Sí |
| Negro oliva / Beige `#D8CFBC` | 12.16:1 | Sí | Sí |
| Gris oliva `#565449` / Marfil | 7.38:1 | Sí | Sí |
| Gris oliva / Beige | 4.92:1 | Sí | Sí |
| Negro oliva / Gris oliva | 2.47:1 | **No** | **No** |
| Beige / Marfil | 1.50:1 | **No** | **No** |

## Decisión

1. **Un solo tema claro** fijo (`ThemePreference::Light`), construido con la
   paleta oficial. No se ofrece tema oscuro en la demo. El mismo estilo se
   aplica a los dos temas internos de egui, para que la app se vea igual aunque
   el sistema operativo o el arnés de pruebas activen el oscuro.
2. El **sidebar** usa un `Frame` con fondo negro oliva y un estilo acotado a ese
   panel: texto marfil, opción seleccionada con fondo beige y texto negro oliva.
3. Reglas derivadas del contraste:
   - Nunca texto gris oliva sobre negro oliva, ni texto beige sobre marfil.
   - Los bordes de campos y tablas sobre marfil son **gris oliva**; el beige
     sobre marfil sólo sirve como superficie decorativa, no como único borde.
   - El hover del sidebar puede usar gris oliva como fondo, pero la selección
     se marca con beige **y** con un indicador adicional (barra o peso de letra),
     porque el color no puede ser la única señal.
   - Las filas seleccionadas de una tabla (fondo beige) llevan también otra
     señal visible.
4. Los colores se definen como **tokens** en `theme/`; las vistas nunca usan
   valores `Color32` sueltos.
5. Los **colores semánticos** (error, advertencia, éxito) quedan pendientes de
   aprobación. Hasta entonces, los estados se muestran con texto e icono en
   colores de la paleta.

## Consecuencias

- La interfaz cumple WCAG AA con la paleta tal como está.
- Si en el futuro se pide modo oscuro, habrá que definir una segunda paleta.
- Los colores semánticos deben aprobarse antes de cerrar los indicadores de
  estado de la Fase 1.
