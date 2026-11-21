# ADR-0003 — Renderizador gráfico (glow o wgpu)

- Estado: Aceptada (2026-11-21); revisable cuando se conozca el hardware de las sucursales
- Fecha: 2026-11-21

## Contexto

eframe puede dibujar con dos renderizadores:

- **wgpu** (predeterminado): DirectX 12 o Vulkan en Windows. Moderno, pero más
  pesado y más exigente con controladores y GPU.
- **glow**: OpenGL. Binario más pequeño, compila más rápido y suele funcionar
  mejor en equipos antiguos, máquinas virtuales y escritorio remoto.

Un POS de sucursal suele correr en equipos modestos o mediante escritorio
remoto. Todavía no conocemos el hardware real.

## Decisión

Usar **glow** como renderizador de la aplicación, porque la interfaz no necesita
gráficos avanzados y prioriza la compatibilidad.

Implementación (Fase 1): eframe 0.37 trae por defecto `accesskit`, `clipboard`,
`default_fonts`, `links`, `system_fonts`, `wayland`, `web_screen_reader`, `wgpu`,
`winit/default` y `x11`. Se desactivan y se declaran sólo `glow`, `accesskit`,
`clipboard`, `default_fonts` y `links`. Las de Linux y la web no aplican a
Windows; `system_fonts` no hace falta porque las fuentes necesarias van
incluidas (ADR-0002). Si algún día se compila para Linux, hay que volver a
añadir `wayland`, `x11` y las *features* de `winit` (ver skill `eframe-project`).
Verificado: la compilación release no incluye wgpu.

Las capturas de pantalla de las pruebas (`egui_kittest` con `snapshot`) usan
wgpu sólo como dependencia de desarrollo; no afecta al ejecutable final.

## Información que puede cambiar la decisión

- Modelo de PC, GPU y versión de Windows de las sucursales.
- Si se usa escritorio remoto (RDP) o máquinas virtuales.

## Consecuencias

- Cambiar después de renderizador cuesta poco: afecta a `Cargo.toml` y a una
  opción de arranque, no a las vistas.
- Si se necesitara pintar con wgpu (gráficos personalizados), habría que
  revisar esta decisión.
