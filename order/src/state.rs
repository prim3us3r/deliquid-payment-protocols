// src/state.rs
use anchor_lang::prelude::*;

#[account]
pub struct Order {
    pub name: String,
    pub description: String,
    pub quantity: i8,
}

impl Order {
    const LEN: usize = 1;
}
