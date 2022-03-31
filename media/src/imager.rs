pub mod prelude {
    pub use image::{
        ImageBuffer, Rgb, Luma,
        io::Reader as ImageReader,
    };
    // pub extern crate imageproc as proc;
    pub extern crate dssim_core as dssim;
    // pub use proc::{haar};
    pub use crate::imager::{self, *};
}

use prelude::*;

pub type RgbBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;