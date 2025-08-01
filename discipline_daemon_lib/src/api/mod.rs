pub mod internal {
  pub use super::implementations::*;
}

mod api;
pub use api::IntoPublic;

mod implementations;