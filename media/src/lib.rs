#[cfg(test)]
mod tests;

pub mod video;
pub mod imager;

pub mod prelude {
    pub use crate::{
        imager::{self, prelude::*}, 
        video::{self, prelude::*}
    };
}