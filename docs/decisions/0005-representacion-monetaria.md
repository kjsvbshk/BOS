# ADR-0005 — Prohibición de coma flotante para importes

- Estado: Aceptada (2026-11-21)
- Fecha: 2026-11-21

## Contexto

`f32` y `f64` no pueden representar exactamente valores como 0.10; en cálculos
de préstamos, intereses y saldos los errores se acumulan. El plan maestro lo
prohíbe expresamente (regla 15).

## Decisión

1. Ningún importe se representa con `f32` ni `f64`.
2. En `pos_core` se activa el lint `clippy::float_arithmetic` como error, para
   que el compilador rechace aritmética en coma flotante en la lógica de negocio.
   La interfaz sí usa `f32` para medidas de pantalla, lo cual es correcto.
3. La representación concreta se decide en la Fase 6, tras revisar los tipos y
   redondeos de la base de datos anterior. Opciones:
   - `rust_decimal::Decimal`: decimal exacto, cómodo para tasas e intereses.
   - Enteros (`i64`) en la unidad mínima (centavos), con un tipo `Money` propio.
4. Durante la demo no hay cálculos monetarios. Si una tabla de demostración
   muestra importes, serán textos fijos de ejemplo, no resultados calculados.

## Consecuencias

- Cuando se elija la representación, todos los importes usarán un único tipo
  `Money` del dominio; las reglas de redondeo quedarán documentadas en él.
