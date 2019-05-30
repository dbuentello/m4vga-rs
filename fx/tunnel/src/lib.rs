#![cfg_attr(feature = "bare_metal", no_std)]

pub mod table;
pub mod render;

const NATIVE_WIDTH: usize = 800;
const NATIVE_HEIGHT: usize = 600;
const SCALE: usize = 2;

const WIDTH: usize = NATIVE_WIDTH / SCALE;
const HEIGHT: usize = NATIVE_HEIGHT / SCALE;
const HALF_WIDTH: usize = WIDTH / 2;
const HALF_HEIGHT: usize = HEIGHT / 2;

const BUFFER_SIZE: usize = WIDTH * HALF_HEIGHT;
const BUFFER_WORDS: usize = BUFFER_SIZE / 4;
const BUFFER_STRIDE: usize = WIDTH / 4;

#[cfg(feature = "bare_metal")]
mod bare;

#[cfg(feature = "bare_metal")]
pub use bare::*;