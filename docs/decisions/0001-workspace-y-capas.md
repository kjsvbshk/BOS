# ADR-0001 — Workspace de Cargo con capas separadas en crates

- Estado: Aceptada (2026-11-21)
- Fecha: 2026-11-21

## Contexto

El plan maestro exige que la interfaz no contenga reglas financieras ni lógica de
persistencia, y define cuatro capas: presentación, aplicación, dominio e
infraestructura. Dentro de un único crate esa separación depende sólo de la
disciplina: cualquier vista puede importar cualquier módulo.

## Decisión

Usar un workspace de Cargo:

- `pos_core` (biblioteca): módulos `domain` y `application`. Sin dependencia de
  egui, eframe ni bases de datos.
- `pos_desktop` (binario): presentación egui/eframe y arranque.
- `pos_infrastructure`: se crea en la Fase 6, cuando se decida la persistencia.

Domain y application comparten crate mientras sean pequeños. Se separarán cuando
el dominio crezca lo suficiente para justificarlo.

## Alternativas descartadas

- **Un único crate con carpetas por capa**: más simple, pero el compilador no
  impide que una vista importe infraestructura o que el dominio use tipos de egui.
- **Cuatro crates desde el inicio**: dominio e infraestructura estarían vacíos
  durante la demo; sería sobreingeniería.

## Consecuencias

- El compilador hace cumplir la regla «la interfaz no contiene lógica de negocio».
- Las pruebas de `pos_core` compilan sin egui y son rápidas.
- Las versiones de dependencias y los lints se declaran una sola vez en el
  `Cargo.toml` del workspace.
- Cuesta un poco más de configuración inicial que un crate único.
