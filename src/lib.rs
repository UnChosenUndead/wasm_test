use wasm_bindgen::prelude::*;

extern crate wee_alloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    pub width: usize
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> Self {
        Self {
            width,
        }
    }

    pub fn default() -> Self {
        Self {
            width: 8
        }
    }
}