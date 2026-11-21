//! Núcleo del POS: dominio y casos de uso.
//!
//! Este crate no depende de la interfaz gráfica ni de la persistencia
//! (ADR-0001). Las fórmulas financieras son provisionales (ADR-0008).

// Ningún importe puede calcularse en coma flotante (ADR-0005).
#![deny(clippy::float_arithmetic)]

pub mod application;
#[cfg(feature = "demo-data")]
pub mod demo;
pub mod domain;
