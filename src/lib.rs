//! Typed bindings to the Screeps in-game API for WASM Rust AIs.
//!
//! # Cargo Features
//!
//! ## `sim`
//!
//! Enables special-case handling of the unique room name present in the
//! simulator - must be enabled to build code that is compatible with that
//! environment. If this is enabled, the top-left valid room coordinate has the
//! name `sim`, otherwise it's named `W127N127`.
//!
//! ## `generate-pixel`
//!
//! Enables the function to generate pixels, which is only present on the
//! Screeps: World official servers.
//!
//! ## `inter-shard-memory`
//!
//! Enables interacting with `IntershardMemory`, which is not present in most
//! private server environments.
//!
//! ## `score`
//!
//! Enables the score resource and entities, introduced for Screeps Seasonal's
//! first season.
//!
//! ## `symbols`
//!
//! Enables the symbol resources and entities, introduced for Screeps Seasonal's
//! second season.
//!
//! ## `thorium`
//!
//! Enables the thorium resource and reactor object, introduced for Screeps
//! Seasonal's fifth season.
//!
//! ## `unsafe-return-conversion`
//!
//! Enables return code conversion from game functions that presumes all return
//! code values are in the expected ranges skipping checks, and risks undefined
//! behavior if they are not.
//!
//! ## `mmo`
//!
//! Enables the `generate-pixel` and `inter-shard-memory` features, which are
//! present on the Screeps: World official servers but not on private servers.
//!
//! ## `seasonal-season-1`
//!
//! Enables the `score` feature, a mechanic introduced for Screeps Seasonal's
//! first season, as well as enabling constants relevant to season 1.
//!
//! ## `seasonal-season-2`
//!
//! Enables the `symbols` feature, a mechanic introduced for Screeps Seasonal's
//! second season, as well as enabling constants relevant to season 2.
//!
//! ## `seasonal-season-5`
//!
//! Enables the `thorium` feature, a mechanic introduced for Screeps Seasonal's
//! fifth season, as well as enabling constants relevant to season 5.
#![recursion_limit = "128"]
// to build locally with doc_cfg enabled, run:
// `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// warn when functions can safely be given the const keyword, see
// https://rust-lang.github.io/rust-clippy/master/index.html#/missing_const_for_fn
// unfortunately this warns for bindgen-attached functions so we can't leave it
// enabled

// #![warn(clippy::missing_const_for_fn)]

// Make clippy really suck. But it has a lot of useful things like docs, API improvements, and can
// catch more bugs.
#![warn(clippy::pedantic)]
// ===================
// RESTRICTIONS
// ===================
// Help with `unsafe`.
#![warn(
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc
)]
// Types are important.
#![warn(clippy::as_underscore)]
// Make it more obvious when Rc/Arc are used.
#![warn(clippy::clone_on_ref_ptr)]
// Binary size.
#![warn(clippy::multiple_crate_versions)]
// Can cause panics in multibyte chars.
#![warn(clippy::string_slice)]
// Sort your tests.
#![warn(clippy::tests_outside_test_module)]
// You probaly forgot to do something.
#![warn(clippy::todo)]
// =================
// CLIPPY EXCLUSIONS
// =================

// Inlining format args can cause formatting issues in many cases, notably when `target_indent +
// str_lit.len() > wrap_width`. This formatting breaking can break more than just the target string
// though. Minimizing string literal lengths helps to avoid this.
#![allow(clippy::uninlined_format_args)]
// This is not that helpful most of the time.
#![allow(clippy::similar_names)]
// Annoyingly lints on `_` arguments in cases where we need the type, but want to pass a value.
#![allow(clippy::needless_pass_by_value)]
// FIXME: add these docs.
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
// FIXME: we only target one specific target, issues with floats, etc
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
// TODO: idk
#![allow(clippy::module_name_repetitions)]

pub mod console;
pub mod constants;
pub mod enums;
pub mod game;
#[cfg(feature = "inter-shard-memory")]
pub mod inter_shard_memory;
pub mod js_collections;
pub mod local;
pub mod memory;
pub mod objects;
pub mod pathfinder;
pub(crate) mod prototypes;
pub mod raw_memory;
pub mod traits;

pub use crate::{constants::*, enums::*, js_collections::*, local::*, objects::*, traits::*};

/// Traits which implement base functionalities for Screeps types.
///
/// # Example
///
/// ```no_run
/// use js_sys::{JsString, Reflect};
/// use screeps::{game, prelude::*, Creep};
///
/// let c = game::creeps().get(String::from("Bob")).unwrap();
///
/// // `HasId` trait brought in from prelude
/// let id = c.try_id().unwrap();
/// ```
///
/// This module contains all base functionality traits, and no structures.
pub mod prelude {
    pub use crate::{js_collections::*, traits::*};
}
