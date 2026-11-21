# ADR-0007 — Referencia visual aprobada y navegación por pestañas superiores

- Estado: Aceptada (2026-11-21)
- Fecha: 2026-11-21
- Reemplaza: el sidebar del ADR-0004 (punto 2) y del plan de la Fase 2.

## Contexto

El usuario entregó `referencia visual.png` como diseño a seguir, con cinco
ajustes:

1. No duplicar la navegación: las tres secciones van en una barra superior,
   como en el sistema original, sin sidebar.
2. La opción 06 es «Salida del inventario de mercancías propias».
3. La opción 09 es «Modificación de contratos» y al abrirse muestra Reversar,
   Anular, Modificar y Borrar.
4. La opción 10 se mantiene como «Otras opciones», sin inventar su contenido.
5. «Salir» es una acción global, no la opción 11; la 11 queda «Pendiente de
   identificar».

Además, las tarjetas se agrupan visualmente en Contratos, Inventario y
Movimientos y consultas, sin cambiar la numeración ni el orden.

## Decisión

- **Marco**: encabezado negro oliva (diamante, nombre del negocio, fecha y hora,
  usuario de demostración), barra de pestañas (Operaciones, Mantenimiento,
  Informes) y «Salir» a la derecha de esa barra, con confirmación.
- **Contenido**: título de página con icono en círculo, tarjetas de opción en
  hasta tres columnas (dos o una en ventanas estrechas o con zoom) y aviso
  «Seleccione una opción del menú para continuar».
- **Navegación**: cada tarjeta abre una pantalla provisional con «Volver» y ruta
  de navegación; Escape vuelve a la pantalla anterior.
- **Tipografía**: Segoe UI de Windows (regular, semibold y negrita), leída de
  `%WINDIR%\Fonts` y no redistribuida; Ubuntu Light de egui como respaldo.
- **Iconos**: fuente Phosphor «Fill» (MIT) incluida en
  `crates/pos_desktop/assets/fonts/`. El crate `egui-phosphor` sólo es compatible
  con egui 0.36 y su bifurcación para 0.37 no es oficial, así que se incluye
  sólo el archivo de fuente.
- **Fecha y hora**: crate `chrono` (sólo la función `clock`). La pantalla se
  redibuja una vez por segundo para el reloj.
- **Catálogo de componentes**: queda como pantalla interna, accesible desde el
  menú del usuario sólo en compilaciones de desarrollo.

## Pendiente

- La opción 09 pertenece al tema de contratos, pero el orden original la deja
  en el bloque «Movimientos y consultas». Confirmar si se acepta así.
- Opción 11: identificar cuál es en el sistema anterior.
- Contenido de «Otras opciones» y de Mantenimiento e Informes.

## Consecuencias

- El reloj impide que la app quede totalmente en reposo: redibuja una vez por
  segundo, un costo mínimo.
- Sin Segoe UI (otro sistema operativo) el texto se ve con Ubuntu Light y sin
  negrita real.
