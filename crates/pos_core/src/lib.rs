//! Núcleo del POS: dominio y casos de uso.
//!
//! Este crate no depende de la interfaz gráfica ni de la persistencia
//! (ADR-0001). Durante la demo todavía no contiene reglas de negocio: se irá
//! llenando cuando se documenten las operaciones reales.

// Ningún importe puede calcularse en coma flotante (ADR-0005).
#![deny(clippy::float_arithmetic)]
