mod ascii_board;
mod ascii_group;

pub mod ascii {
    pub use crate::visualization::ascii_board::AsciiBoardPrinter;
    pub use crate::visualization::ascii_group::AsciiGroupPrinter;
}