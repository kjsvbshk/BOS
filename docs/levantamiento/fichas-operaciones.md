# Fichas de Operaciones

Cada ficha separa lo **confirmado** en la transcripción (con minuto), lo
**inferido** y lo **pendiente**. Las teclas siguen el sistema anterior: Enter
avanza, F1 consulta, Esc vuelve, letras para confirmar.

Convención de la demo: los importes se calculan con las fórmulas provisionales
del ADR-0008, deducidas de los ejemplos de la reunión.

---

## 01 · Ingresar contratos al inventario de contratos vigentes

**Confirmado** (01:38–10:00)

1. Número de contrato: el usuario lo digita desde su libreta.
2. Plazo en meses: propone 6; se puede cambiar (1, 2, 3, 6).
3. Porcentaje: propone 10 %; se puede cambiar.
4. ¿Contrato común? (Sí/No).
5. Cédula del cliente; F1 abre el listado de clientes. Muestra nombre,
   dirección, teléfono e historial de contratos con su estado.
6. Valor del empeño (capital).
7. Código de mercancía (F1). Si nunca se ha empeñado, «artículo esporádico» y se
   describe a mano.
8. Descripción del artículo y estado físico (torcido, reventado…).
9. Peso en gramos.
10. Quilataje (F1), cuando es oro.
11. Fecha del contrato: hoy o una anterior.
12. ¿Imprimir contrato? (el cliente nunca imprime).
13. Grabar o corregir (C).
14. ¿Continuar ingresando contratos?

**Inferido**: los contratos no comunes reciben número automático (22:51), por eso
en la demo se pregunta el tipo antes del número y se salta ese paso para ellos.
El quilataje sólo se pide para códigos de oro o plata.

**Pendiente**: campos propios de los contratos hipotecarios (P6); opción de
contratos antiguos con pagos previos (06:43) — no se incluye.

## 02 · Prorrogar plazo a contratos vigentes

**Confirmado** (16:33–18:25): número de contrato → muestra plazo actual → ¿modificar
el plazo? → importe del plazo completo (3 meses = 84.000) → meses a pagar (2 =
56.000) → fecha de pago → grabar. La consulta muestra el pago y el saldo (428.000).

## 03 · Retiro de contrato – Devolución a dueño original

**Confirmado** (14:50–16:33): número → fecha de empeño, meses de interés pendientes,
pagos realizados → total (1.000.000 al 7 %, 3 meses = 1.210.000) → descripción y
peso del artículo → ¿es el mismo artículo? → grabar → ¿continuar? El contrato
queda retirado con fecha y valor.

## 04 · Contratos vencidos – Pasar al inventario de mercancía propia

**Confirmado** (18:52–24:06): individual o masivo; pantalla o impresora; filtros
por intereses pagados, no pagados, plazo diferente, con y sin abonos (el cliente
siempre usa «con y sin»); código de artículo (0 = general); comunes o ambos tipos.
Listado con valor prestado, valor adeudado y totales. Individual: número →
¿pasar a propios? → deja de figurar en vigentes.

**Restricción**: sólo el propietario (54:14).

## 05 · Ingresar mercancía al inventario de mercancía propia

**Pendiente** (P1): la reunión no la muestra. En la demo queda como pantalla
provisional; no se inventa el proceso.

## 06 · Salida del inventario de mercancías propias

**Confirmado** (25:21–28:44): número de contrato; valida que sea propio («no está
en venta»); modalidad venta o separado.

- Venta: fecha, cédula del comprador (opcional), valor de venta. Muestra el valor
  a la fecha (1.280.000) y la diferencia (vendido en 1.000.000 → pérdida 280.000).
- Separado: valor (500.000), fecha estimada de entrega (31-dic-2026).
- Abono a separado: número de contrato, fecha, valor; muestra lo que falta.

**Inferido**: el abono a separados se incluye aquí como tercera modalidad (P3).

## 07 · Ingresos y/o egresos de caja

**Confirmado** (28:47–30:14): ingreso o egreso; concepto por código con F1
(gastos, utilidades, pérdidas, otros); descripción; fecha; valor.

## 08 · Ingresos y/o egresos de banco

**Confirmado** (30:14–30:26): existe, el cliente nunca lo usó y cree que no es
necesario. En la demo funciona igual que caja, rotulado «Sin uso».

## 09 · Consultas varias

**Confirmado** (10:00–11:44, 12:10–14:19): consulta de contrato por número: cliente,
condiciones, fechas de empeño y vencimiento, interés mensual, valor de retiro a
hoy, artículo con peso y quilataje, estado, historial de intereses y abonos.

**Inferido**: tiene más consultas internas («ingresos de mercancía», 10:03). La
demo incluye consulta de contrato y consulta por cliente.

## 10 · Modificar, reversar, anular y borrar

**Confirmado** (30:35–34:42):

| Acción | Efecto |
|---|---|
| Modificar contrato | Cambia valor, porcentaje u otros datos |
| Abono a capital | Reduce el capital (se hace «modificando» el contrato, 13:41) |
| Revertir traslado | Un contrato pasado a propios vuelve a vigente si no se ha vendido |
| Reversar retiro | Un retiro hecho por error vuelve a vigente |
| Anular movimiento | Por número de movimiento; el contrato queda como si no hubiera ocurrido |
| Anular venta | La mercancía vuelve a propios |
| Anular abono | Un abono a separado mal registrado se deshace |
| Anular contrato | Un contrato mal ingresado queda anulado |

**Restricción**: modificar contratos sólo el propietario (54:14).

## 11 · Otras opciones

**Pendiente** (P2, P5): posible ubicación de balances, cierres de fin de año y
acumulados (35:10). Pantalla provisional.
