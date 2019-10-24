//! Processor core registers

mod arithmetic;
mod branch;
mod coprocessor;
mod data;
mod logic;
mod misc;

pub use self::misc::nop;
pub use self::misc::svc;