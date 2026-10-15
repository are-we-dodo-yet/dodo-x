// Write good code.
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::style)]
#![warn(clippy::absolute_paths)]
#![warn(clippy::empty_docs)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::needless_pass_by_ref_mut)]
// Never panic.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::string_slice)]
#![deny(clippy::arithmetic_side_effects)]
// Finish your code
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]

pub mod components;
pub mod javascript;
