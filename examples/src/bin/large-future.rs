//! The lint `large_futures` needs to be enabled mannually as it's in the group
//! `clippy::pedantic`.
//!
//! To examine the warning for this file, run:
//! `cargo clippy -- -W clippy::pedantic`

#![allow(clippy::large_types_passed_by_value)]
#![allow(clippy::unused_async)]

async fn large_future(_x: [u8; 16 * 1024]) {}

pub async fn trigger() {
    large_future([0u8; 16 * 1024]).await;
}

fn main() {}