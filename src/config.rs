use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_iterations: u32,
    pub max_iterations: u32,
    pub click_change_amount: u32,

    pub width: u32,
    pub height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_iterations: 10,
            max_iterations: 1000,
            click_change_amount: 10,
            width: 1200,
            height: 900,
        }
    }
}
