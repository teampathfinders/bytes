#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod read;
mod read_buffer;
mod write;
mod write_buffer;
mod error;

pub use read_buffer::ReadBuffer;
pub use write_buffer::WriteBuffer;
pub use error::{Result, Error, ErrorKind};