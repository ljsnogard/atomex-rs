// Declaring our library as `no-std` unconditionally lets us be consistent
// in how we `use` items from `std` or `core`
#![no_std]

// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(test)]
extern crate std;

mod atomex_ptr_;
mod atomic_cell_;
mod atomic_count_;
mod atomic_flags_;
mod cmpxch_result_;
pub mod fetch;

pub use atomex_ptr_::*;
pub use atomic_cell_::*;
pub use atomic_count_::*;
pub use atomic_flags_::*;
pub use cmpxch_result_::*;

pub mod x_deps {
    pub use funty;
}
