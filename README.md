# POS Pawnshop

Aplicación de escritorio para la gestión de una casa de empeños, en Rust con
egui/eframe. Estado actual: **demo de flujos completos** (ADR-0008): todas las
operaciones descritas por el cliente se recorren con teclado, con datos de
demostración en memoria y fórmulas provisionales. Ver `docs/plan-demo-flujos.md`.

Al abrirla, elija un usuario: «Propietario» ve todo; «Recepción» (perfil
administrativo) no puede pasar vencidos a propios, modificar ni ver acumulados.

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

## Generar el instalador

```powershell
# Una sola vez: instalar Inno Setup 6
winget install --id JRSoftware.InnoSetup -e

# Cada vez que se quiera publicar una versión
powershell -ExecutionPolicy Bypass -File packaging\build-package.ps1
```

Resultado en `dist\` (no se versiona):

| Archivo | Uso |
|---|---|
| `Compraventa3Diamantes-<versión>-instalador.exe` | Instalador: menú Inicio, acceso directo opcional en el escritorio y desinstalador |
| `Compraventa3Diamantes-<versión>-portable.zip` | Copiar, descomprimir y ejecutar `Compraventa3Diamantes.exe`, sin instalar |

Con `-PortableOnly` sólo se genera el `.zip`. La versión sale de
`[workspace.package] version` en `Cargo.toml`.

El ejecutable incluye el runtime de C (no requiere Visual C++ Redistributable)
y sólo usa bibliotecas propias de Windows. Requisitos del equipo: Windows 10 u
11 de 64 bits con controlador de video compatible con OpenGL 2 (ADR-0003).

### En GitHub Actions

El workflow `.github/workflows/build.yml` corre en Windows:

- **Push a `main` y pull requests:** formato, clippy, pruebas y paquetes. El
  instalador y el portable se descargan desde la ejecución (Actions ›
  ejecución › *Artifacts*), durante 30 días.
- **Release publicada:** la etiqueta debe ser `v` + la versión de
  `Cargo.toml` (por ejemplo `v0.1.0`); si no coincide, la ejecución falla. El
  instalador y el portable se adjuntan a la release.

Para publicar una versión: subir el número en `[workspace.package] version`,
hacer push, y crear la release con la etiqueta correspondiente.

## Estructura

| Ruta | Contenido |
|---|---|
| `crates/pos_core` | Dominio y casos de uso. Sin interfaz ni persistencia |
| `crates/pos_desktop` | Aplicación egui: tema, componentes, vistas |
| `docs/architecture.md` | Arquitectura y convenciones |
| `docs/design-system.md` | Tokens, componentes y reglas visuales |
| `docs/roadmap.md` | Plan por fases |
| `docs/decisions/` | Registro de decisiones (ADR) |
| `packaging/` | Script de empaquetado e instalador (Inno Setup) |
| `docs/levantamiento/` | Reunión con el cliente, glosario, fichas y preguntas |
