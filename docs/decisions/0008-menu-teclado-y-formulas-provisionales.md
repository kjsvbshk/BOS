# ADR-0008 — Menú corregido, captura por teclado, datos en memoria y fórmulas provisionales

- Estado: Aceptada (2026-11-21)
- Fecha: 2026-11-21
- Reemplaza: la numeración de las opciones 09–11 del ADR-0007 y la regla «no
  implementar lógica financiera en la demo» del plan maestro (sólo para las
  fórmulas listadas aquí).

## Contexto

La reunión del 2026-11-18 (`docs/levantamiento/`) aportó el detalle de los
flujos. El usuario aprobó (2026-11-21) el plan `docs/plan-demo-flujos.md` con
todas sus decisiones y pidió implementar ya las fórmulas, aunque no estén
documentadas por el sistema anterior.

## Decisiones

1. **Menú**: 09 Consultas varias, 10 Modificar, reversar, anular y borrar,
   11 Otras opciones (la transcripción ubica las consultas en la 9).
2. **Teclado**: Enter avanza y confirma, F1 abre catálogos, Esc vuelve, letras
   S/N y G/C para preguntas. Todas las operaciones se pueden completar sin
   ratón.
   - **Actualización (2026-11-21, a pedido del usuario)**: un número **sólo
     selecciona** la tarjeta y Enter la abre, para evitar aperturas por
     accidente. Dos cifras seguidas forman un número (1, 1 = opción 11). Si lo
     escrito no lleva a una opción permitida, se quita la selección. Igual en
     submenús, inicio de sesión y listas de elección de los flujos.
   - Los campos que piden contratos, clientes, movimientos o códigos muestran
     **sugerencias** filtradas mientras se escribe (↑ ↓ y Enter, o clic) y la
     **ficha de la sugerencia seleccionada** en el panel de contexto; F1 abre la
     lista completa. Las sugerencias sólo incluyen registros válidos para la
     operación (por ejemplo, contratos vigentes o vencidos en un retiro).
   - Los importes muestran separadores de miles mientras se escriben.
   - **Máscaras de entrada**: cada campo sólo admite los caracteres de su tipo,
     también al pegar (cédula: dígitos, 6–10; teléfono: dígitos y espacios, 7 o
     10 dígitos; nombres: letras; gramos y porcentajes: un separador decimal;
     números de contrato, movimiento y códigos: dígitos; fechas: dígitos y `/`).
     Los campos numéricos filtran las sugerencias por número; la búsqueda por
     nombre o artículo se hace con F1. La cédula también se valida en `pos_core`.
3. **Datos de demostración en memoria**: `pos_core` mantiene un almacén en
   memoria con datos de ejemplo (feature `demo-data`). Las operaciones lo
   modifican; nada se guarda al cerrar.
4. **Estado derivado de movimientos**: el estado de un contrato se calcula a
   partir de sus movimientos no anulados. Anular o reversar es marcar un
   movimiento como anulado y dejar constancia en la auditoría; nunca se borran
   registros.
5. **Fórmulas provisionales** (`pos_core::domain::interest`), deducidas de los
   ejemplos de la reunión y cubiertas por pruebas con esos mismos casos:

| Fórmula | Definición | Ejemplo de la reunión |
|---|---|---|
| Interés mensual | capital × porcentaje, redondeado al peso | 700.000 × 7 % = 49.000 (11:00) |
| Meses causados | meses calendario desde la fecha del contrato; un mes empezado cuenta completo; mínimo 1 | 28-ago → 18-nov = 3 meses (15:19) |
| Meses pendientes | meses causados − meses pagados, mínimo 0 | 3 causados − 2 pagados = 1 (18:16) |
| Valor de retiro | capital vigente + interés mensual × meses pendientes | 1.000.000 + 3 × 70.000 = 1.210.000 (15:40) |
| Prórroga | interés mensual × meses a pagar | 2 × 28.000 = 56.000 (17:26) |
| Abono a capital | reduce el capital; el interés se calcula sobre el nuevo capital | 600.000 → 42.000 al mes (13:50) |
| Vencimiento | fecha del contrato + máx(plazo, meses pagados) meses | 28-ago + 3 = 28-nov (12:34) |
| Diferencia de venta | valor de venta − valor de retiro a la fecha; negativa = pérdida | 1.000.000 − 1.280.000 = −280.000 (27:26) |
| Saldo de separado | valor del separado − abonos | — |

Los importes son enteros en pesos (`Money(i64)`); el porcentaje se guarda en
centésimas de punto (`Rate`), y el peso en miligramos. No se usa coma flotante
(ADR-0005).

## Consecuencias

- Las fórmulas son **provisionales**: se validarán contra el sistema anterior
  (código y base de datos) antes del desarrollo funcional definitivo. Las dudas
  abiertas están en `preguntas-cliente.md` (P10–P12).
- Al encontrar diferencias, sólo cambia `domain::interest` y sus pruebas.
