# POS Pawnshop

Aplicación de escritorio para la gestión de una casa de empeños, en Rust con
egui/eframe. Estado actual: **demo en construcción** (Fase 1 completada: sistema
visual y componentes base).

## Requisitos

- Windows 10/11 de 64 bits.
- Rust estable 1.95 o superior (`rustup update`) con `clippy` y `rustfmt`.
- Visual Studio Build Tools con las herramientas de C++ (MSVC).

## Uso

```sh
cargo run -p pos_desktop                 # ejecutar en modo desarrollo
cargo build --release -p pos_desktop     # ejecutable optimizado
cargo test --workspace                   # pruebas
cargo fmt --all --check                  # formato
cargo clippy --all-targets -- -D warnings
cargo test -p pos_desktop --test review -- --ignored --nocapture   # capturas de revisión
```

## Estructura

| Ruta | Contenido |
|---|---|
| `crates/pos_core` | Dominio y casos de uso. Sin interfaz ni persistencia |
| `crates/pos_desktop` | Aplicación egui: tema, componentes, vistas |
| `docs/architecture.md` | Arquitectura y convenciones |
| `docs/design-system.md` | Tokens, componentes y reglas visuales |
| `docs/roadmap.md` | Plan por fases |
| `docs/decisions/` | Registro de decisiones (ADR) |
