# ADR-0006 — Navegación definida como catálogo de datos

- Estado: Aceptada (2026-11-21)
- Fecha: 2026-11-21

## Contexto

Operaciones tendrá 11 opciones cuyos nombres aún no se conocen; Mantenimiento e
Informes tienen opciones pendientes de levantamiento. El plan exige que agregar,
quitar o reordenar opciones no obligue a modificar toda la interfaz.

## Decisión

1. Un **catálogo** único (`navigation/catalog.rs`) declara secciones y opciones
   con identificador estable, etiqueta y estado (`PendienteDeDefinicion` o
   `Disponible`).
2. Los identificadores (`operaciones.01` … `operaciones.11`) no cambian cuando
   llegan los nombres reales; sólo cambia la etiqueta.
3. Las vistas se registran por identificador. Una opción sin vista registrada
   muestra la vista provisional genérica, que indica que la opción está
   pendiente de especificación.
4. El sidebar, el header y las pruebas recorren el catálogo; ninguna vista usa
   `match` sobre nombres de opciones.
5. Las secciones sin opciones muestran un estado vacío explicativo.

## Consecuencias

- Incorporar un módulo real consiste en: actualizar su etiqueta y estado en el
  catálogo y registrar su vista.
- Las pruebas de navegación pueden recorrer el catálogo completo y comprobar que
  cada opción abre una vista, sin escribir una prueba por opción.
- Si las opciones reales resultan tener submenús, el catálogo admite añadir un
  nivel de anidamiento; por eso conviene ver primero el menú actual.
