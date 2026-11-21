//! Dominio: valores, entidades y fórmulas del negocio. Sin interfaz ni
//! persistencia.

pub mod catalog;
pub mod contract;
pub mod interest;
pub mod people;
pub mod values;

pub use catalog::{
    CashConcept, CashDirection, ConceptCategory, GoldQuality, MerchandiseCode, Metal,
};
pub use contract::{Contract, ContractKind, ContractStatus, Item, Movement, MovementKind};
pub use people::{Customer, Permission, Profile, User};
pub use values::{Money, ParseValueError, Rate, Weight};
