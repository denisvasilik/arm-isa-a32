//! API for Arm Base ISA A32
//!
//! This crate provides:
//!
//! - Wrappers around assembly instructions
//!

#![no_std]
#![cfg_attr(feature = "inline-asm", feature(llvm_asm))]
#![feature(core_intrinsics)]

pub mod instructions;
