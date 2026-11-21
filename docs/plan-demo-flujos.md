# Plan de trabajo — Demo de flujos completos

Estado: **aprobado y ejecutado** (2026-11-21). Fases A–H completas; ver
«Resultado» al final. Decisiones: D1 sí (datos en memoria), D2 **fórmulas
implementadas** como provisionales (ADR-0008), D3 sí, D4 sí.

Fuentes (en `docs/levantamiento/`):

- `2026-11-18-transcripcion-reunion.md`: transcripción de la reunión con el
  cliente (fuente primaria).
- `2026-11-18-analisis-reunion.md`: análisis de esa transcripción (fuente
  secundaria; cuando difiere de la transcripción, manda la transcripción).

Este plan **no** inicia el desarrollo funcional definitivo: sigue siendo una
demo. Su objetivo es que el cliente pueda recorrer, con teclado, cada flujo que
describió en la reunión, con datos simulados, para validar pantallas, pasos y
nombres antes de analizar el código y la base de datos del sistema anterior.

---

## 1. Hallazgos al contrastar análisis y transcripción

### 1.1 Corrección del menú de Operaciones (afecta a la demo actual)

La transcripción dice expresamente que la consulta de contratos está en «consultas
varias […] la opción 9» (min. 10:00–10:29). La demo actual tiene la opción 09
como «Modificación de contratos». La distribución coherente con la reunión es:

| N.º | Opción | Demo actual | Propuesta |
|---|---|---|---|
| 01 | Ingresar contratos al inventario de contratos vigentes | Igual | Igual |
| 02 | Prorrogar plazo a contratos vigentes | Igual | Igual |
| 03 | Retiro de contrato – Devolución a dueño original | Igual | Igual |
| 04 | Contratos vencidos – Pasar al inventario de mercancía propia | Igual | Igual |
| 05 | Ingresar mercancía al inventario de mercancía propia | Igual | Igual (contenido pendiente) |
| 06 | Salida del inventario de mercancías propias | Igual | Igual |
| 07 | Ingresos y/o egresos de caja | Igual | Igual |
| 08 | Ingresos y/o egresos de banco | Igual | Igual (sin uso según el cliente) |
| 09 | — | Modificación de contratos | **Consultas varias** |
| 10 | — | Otras opciones | **Modificar, reversar, anular y borrar** |
| 11 | — | Pendiente de identificar | **Otras opciones** |

La numeración de la 10 y la 11 sigue siendo una hipótesis: la transcripción no
lee el menú completo. Se registrará en un ADR nuevo que reemplace esa parte del
ADR-0007.

### 1.2 Diferencias o datos que el análisis no recoge

| Tema | Transcripción | Consecuencia |
|---|---|---|
| Ejemplo de venta (min. 27:05) | «a la fecha de hoy vale 1.000.2 80» (1.280.000), se vende en 1.000.000, «la pérdida […] fueron 280.000» | El análisis dice «valor de referencia 720.000» y duda entre utilidad y pérdida. Con 1.280.000 la pérdida es coherente. Confirmar, pero la lectura más probable es **pérdida**. |
| Numeración de contratos | Los comunes los digita el usuario desde su libreta; los no comunes (hipotecarios) los numera **el sistema** y no se pueden cambiar (min. 22:51) | El paso «Número de contrato» depende del tipo de contrato. |
| Vocabulario | «Sobrecosto» se usa como sinónimo de intereses (min. 21:03, 38:51, 45:48) | Va al glosario; los informes del legado usan esa palabra. |
| Abonos a capital | Se registran desde «modificar contrato» (min. 13:41–13:50, 30:35) | El abono a capital vive en la opción 10, no en prórroga ni retiro. |
| Abono a mercancía separada | «por acá voy a pedir ingresos» (min. 28:09–28:31) | No queda claro si es la opción 05, la 07 u otra. Pendiente. |
| Contratos vencidos (04) | Código de artículo «0» = inventario general; filtros por intereses pagados / no pagados / plazo diferente / con y sin abonos; comunes o ambos tipos; salida por pantalla o impresora | Detalle suficiente para la pantalla de filtros. |
| «Acumulados y tarjetas» | «estos 3 últimos […] no lo necesito» (min. 34:46) y una última opción de balances, cierres y acumulados de año (min. 35:10) | Probable contenido de 11 «Otras opciones», o de Informes. Pendiente. |
| Salida de informes | Muchos informes preguntan «por pantalla o impresora» | La demo muestra todo en pantalla; imprimir queda deshabilitado con su motivo. |
| Perfiles | El cliente propone dejar sólo **administrativo** y **propietario**; el propietario además saca vencidos, modifica contratos y ve acumulados (min. 54:29–55:14) | Dos perfiles en la demo. |
| Varios negocios | Usa el programa en otros dos negocios, cada uno con su instalación (min. 52:30) | El nombre del negocio debe ser configurable, no fijo en el código. |
| Copia de seguridad | Es un programa aparte («otro icono», min. 4:54) | Se representará como pantalla de Mantenimiento; pendiente confirmar. |
| Impresión de contratos | El cliente no imprime contratos (min. 9:15) | El paso «¿Imprimir?» se conserva por fidelidad, pero sin impresión real. |

### 1.3 Restricciones que se mantienen

- No se implementan fórmulas financieras (intereses, retiros, saldos): no están
  documentadas y el plan maestro prohíbe lógica financiera en la demo. Los
  importes que se muestren serán **datos de ejemplo fijos**, rotulados como tales.
- No hay base de datos: los datos de la demo viven en memoria.
- No se crea el esquema definitivo de entidades hasta analizar la base anterior.

---

## 2. Alcance de la demo

### Incluye

- Todos los flujos de Operaciones descritos en la reunión, paso a paso y
  operables **sólo con teclado** (Enter avanza, F1 consulta catálogos, Esc
  vuelve, C corrige, letras para confirmar).
- Consultas de contratos con ficha completa e historial.
- Las pantallas de Mantenimiento e Informes que el cliente mostró o mencionó.
- Dos perfiles simulados (administrativo y propietario) para mostrar qué opciones
  quedan restringidas.
- Datos de ejemplo coherentes con los casos de la reunión (contratos 26613,
  26247, 26148, 26320, la venta 23668, códigos 8 bicicletas, 25 motos, 28 sueldos,
  33 seguridad social, 42 cafetería, quilatajes 18/16/14…), rotulados como datos
  de demostración.

### No incluye

- Cálculos de intereses, retiros, saldos, utilidades o acumulados.
- Persistencia, migración, respaldo real, impresión.
- Inicio de sesión real con contraseñas.

---

## 3. Fases de trabajo

Cada fase termina compilando, con `cargo fmt`, `cargo clippy` sin avisos,
pruebas en verde, capturas revisadas y **mensajes de commit preparados** (los
commits los hace el usuario).

### Fase A — Especificación de la demo (sin código)

1. **Fichas por opción** en `docs/levantamiento/opciones/`: una por cada opción
   de Operaciones, Mantenimiento e Informes. Cada ficha separa **confirmado**
   (con minuto de la transcripción), **inferido** y **pendiente**, y lista los
   pasos, los campos, las teclas y los mensajes.
2. **Glosario** (`docs/levantamiento/glosario.md`): retroventa, sobrecosto,
   propios, separado, abono, prórroga, artículo esporádico, contrato común e
   hipotecario, número de movimiento, inventario general.
3. **Preguntas para el cliente** (`docs/levantamiento/preguntas-cliente.md`),
   ordenadas por impacto.
4. **ADR-0008**: corrección del menú (09 Consultas, 10 Modificaciones, 11 Otras
   opciones), patrón de captura por teclado y datos de demostración en memoria.

Entregable: documentación revisable por el usuario. **Punto de aprobación.**

### Fase B — Base para flujos por teclado

Componentes nuevos, reutilizables por todas las operaciones:

| Componente | Función |
|---|---|
| `StepFlow` | Captura secuencial: un paso activo, pasos anteriores resumidos, Enter valida y avanza, Esc retrocede |
| `LookupDialog` | Ventana F1: lista filtrable con teclado (flechas, Enter, Esc) para clientes, códigos de mercancía, quilatajes y conceptos |
| `ReviewPanel` | Resumen antes de grabar: «G Grabar / C Corregir» |
| `PromptBar` | Preguntas de una tecla: «¿Imprimir? S/N», «¿Continuar? S/N» |
| `RecordCard` | Ficha de solo lectura (contrato, cliente, artículo) |
| `KeyHintBar` | Barra inferior con las teclas válidas en cada momento |
| `MoneyText` | Muestra importes de ejemplo con formato colombiano ($1.210.000) a partir de enteros; sin aritmética |

Además: atajos numéricos para abrir cada opción desde la pantalla de sección
(1–9, 0 = 10, la 11 por definir), y menú de secciones con Alt+letra.

**Datos de demostración** en `pos_core::demo` (feature `demo-data`): clientes,
contratos en todos los estados, movimientos, inventario propio, separados,
catálogos y usuarios. Se exponen mediante puertos de `pos_core::application`,
como prevé el ADR-0001, para que la interfaz no sepa de dónde vienen.

Entregable: un flujo de ejemplo navegable con todos los componentes y sus pruebas.

### Fase C — Operaciones principales (lo que el cliente usa a diario)

En este orden, porque la reunión los mostró en detalle y dependen entre sí:

1. **09 Consultas varias** → consulta de contrato: ficha completa, historial de
   intereses y abonos, valor de retiro de ejemplo, estado (vigente, vencido,
   retirado, propio). Va primero porque los demás flujos muestran la misma ficha.
2. **01 Ingresar contratos**: los 10 pasos de la reunión, con F1 para cliente,
   código de mercancía y quilataje; artículo esporádico; historial del cliente;
   numeración manual (común) o automática (hipotecario); fecha anterior permitida.
3. **03 Retiro de contrato**: buscar, ficha, total de ejemplo, verificación
   física del artículo, confirmación y cambio de estado.
4. **02 Prórroga**: plazo actual, ¿modificar plazo?, meses a pagar, fecha,
   importe de ejemplo, grabar.

En la demo, grabar cambia el estado del contrato **en memoria** (por ejemplo, un
retiro lo deja «retirado» y la consulta lo muestra así) hasta cerrar la app.
Pendiente de aprobación: ver decisión D1.

### Fase D — Inventario y caja

1. **04 Contratos vencidos**: individual o masivo; filtros (código de artículo,
   0 = general; intereses pagados / no pagados / plazo diferente / con y sin
   abonos; comunes o ambos tipos); listado con totales de ejemplo; traslado a
   propios con confirmación.
2. **06 Salida de inventario**: venta directa o separado; validación de que el
   artículo es propio («no está en venta»); valor de referencia y diferencia de
   ejemplo; fecha estimada de entrega en separados.
3. **Abono a mercancía separada**: ubicación pendiente (ver pregunta P3).
4. **07 Caja**: ingreso o egreso, concepto con F1 (gastos, utilidades,
   pérdidas, otros), descripción, fecha y valor.
5. **08 Banco**: misma pantalla que caja, rotulada «Sin uso en el sistema
   actual» hasta que el cliente decida.

### Fase E — Modificaciones y anulaciones (opción 10)

Submenú con las siete acciones descritas: modificar contrato (incluye abono a
capital), revertir traslado a propios, reversar retiro, anular movimiento (por
número de movimiento), anular contrato, anular venta, anular abono. Cada una:
buscar, ver el efecto que se deshará, confirmar. Restringida al perfil
propietario donde corresponda.

### Fase F — Mantenimiento

Usuarios (código, nombre, perfil), matriz de permisos (sólo lectura en la demo),
códigos de mercancía, códigos de calidad de oro, códigos de gastos, códigos de
caja/banco, datos del negocio (nombre configurable) y copia de seguridad
(pantalla informativa). Altas y ediciones en memoria.

### Fase G — Informes

En pantalla, con datos de ejemplo y «Imprimir» deshabilitado: caja diaria por
fecha y su resumen, gastos por período (por código o por fecha, detallado o no),
acumulados mensuales por operación (comunes e hipotecarios por separado),
inventario de contratos vigentes (por código, oro por quilataje, plata),
inventario de mercancía propia y auditoría interna.

### Fase H — Perfiles y cierre de la demo

Selector de perfil simulado al iniciar (Administrativo / Propietario). Las
opciones restringidas se muestran deshabilitadas con el motivo. Recorrido
completo sólo con teclado, capturas, `egui-review`, consumo en reposo con
`egui-profiler` y guion de presentación para el cliente.

---

## 4. Decisiones que necesito del usuario

| N.º | Decisión | Recomendación |
|---|---|---|
| D1 | ¿Las operaciones de la demo modifican los datos en memoria (un retiro cambia el estado en la consulta) o todo es estático? | **En memoria**: el recorrido se siente real y sigue sin haber persistencia ni fórmulas. |
| D2 | Importes: ¿sólo valores de ejemplo fijos, o aceptas una fórmula provisional de interés simple mensual rotulada como provisional? | **Valores fijos**: respeta la regla de no implementar lógica financiera sin documentarla. |
| D3 | Aplicar ya la corrección del menú (09 Consultas, 10 Modificaciones, 11 Otras opciones). | **Sí**, en la Fase A, con su ADR. |
| D4 | Teclas: replicar el estilo del legado (números para opciones, F1, letras para confirmar) además de ratón. | **Sí**, es el requisito más explícito de la reunión. |

## 5. Preguntas para el cliente (no bloquean la demo)

| N.º | Pregunta | Afecta a |
|---|---|---|
| P1 | ¿Qué hace la opción 05? ¿Compras directas de artículos no empeñados? | Fase D |
| P2 | ¿Confirmas 09 Consultas varias, 10 Modificaciones y 11 Otras opciones? ¿Qué contiene la 11? | Menú |
| P3 | ¿Desde dónde se registra el abono a una mercancía separada? | Fase D |
| P4 | Venta del 23668: ¿el valor a la fecha era 1.280.000 y hubo pérdida de 280.000? | Fase D |
| P5 | ¿Qué son «acumulados y tarjetas» y las tres opciones que no usa? | Opción 11 / Informes |
| P6 | ¿Qué campos distintos tiene un contrato hipotecario? | Fase C |
| P7 | ¿Dónde y cómo se hace el cierre de mes y el ajuste de caja? | Fases D y G |
| P8 | ¿Basta con dos perfiles (administrativo y propietario)? | Fase H |
| P9 | ¿La copia de seguridad debe integrarse al programa nuevo? | Fase F |

## 6. Fuera del alcance de la demo (siguientes etapas)

Inspección del código y la base de datos del sistema anterior, documentación de
fórmulas, redondeos y estados, modelo de datos y persistencia, movimientos
auditables, migración de 20 años de histórico, respaldo y restauración reales.

## 7. Riesgos

- **Rehacer pantallas** si el cliente corrige el menú o los pasos: se mitiga con
  la Fase A (fichas revisables) y con flujos construidos sobre componentes
  comunes.
- **Que el cliente tome los valores de ejemplo como cálculos reales**: todos los
  importes llevan la marca «Ejemplo».
- **Alcance**: la demo cubre unas 30 pantallas. Si hace falta mostrar algo
  antes, se puede presentar al cliente al terminar la Fase C.

---

## Resultado (2026-11-21)

| Fase | Resultado |
|---|---|
| A | Glosario, fichas de Operaciones, Mantenimiento e Informes, preguntas para el cliente y ADR-0008 en `docs/levantamiento/` y `docs/decisions/` |
| B | Motor de captura por pasos (`views/flow.rs`), consulta F1, barra de teclas; dominio, almacén en memoria, fórmulas e informes en `pos_core` con datos de demostración |
| C | 01 Ingresar contratos, 02 Prórroga, 03 Retiro, 09 Consultas (contrato, cliente, movimiento) |
| D | 04 Vencidos (filtros, individual y masivo), 06 Venta, separado y abono a separado, 07 Caja, 08 Banco |
| E | 10 con ocho acciones: modificar, abono a capital, revertir paso a propios, reversar retiro, anular movimiento, anular venta o separado, anular abono, anular contrato |
| F | Clientes, usuarios y permisos, códigos de mercancía, calidades de oro, conceptos de caja, datos del negocio, copia de seguridad (informativa) |
| G | Caja diaria, gastos del período, acumulados por operación, inventario de vigentes, mercancía propia, auditor interno |
| H | Inicio de sesión con perfil simulado (Administrativo / Propietario); opciones restringidas deshabilitadas con su motivo |

Validación: 99 pruebas (44 del núcleo con los casos de la reunión, 15 flujos
completos recorridos sólo con teclado, navegación y componentes), clippy sin
avisos, capturas revisadas y ejecutable release sin wgpu.

Opciones que siguen provisionales por falta de información: 05 (P1) y 11 (P2, P5).
