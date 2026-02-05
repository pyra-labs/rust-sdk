#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(unused_must_use)]

pub mod constants;

pub mod admin;
pub mod drift;
pub mod operations;
pub mod user;

pub use admin::*;
pub use drift::*;
pub use operations::*;
pub use user::*;

#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::arithmetic_side_effects,
    unused_must_use,
    unused_imports,
    clippy::too_many_arguments,
    ambiguous_glob_reexports,
    clippy::new_without_default,
    missing_docs,
    unused_qualifications
)]
mod generated {
    use anchor_lang::declare_program;
    declare_program!(pyra);
}

pub use generated::pyra as pyra_program;
