use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum LaunchPadError {
    
}

#[derive(Clone, CandidType, Deserialize)]
pub struct RuneInfo {
    pub decimals: u128,
    pub symbol: String,
    pub amount: u128
}

pub struct Converter {
    pub base_offset: u32,
}

impl Converter {
    pub fn symbol_to_int(&self, symbol: &str) -> Result<u128, String> {
        if !symbol.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(format!("Invalid symbol: {}. Only uppercase letters A-Z are allowed", symbol));
        }

        let mut value = 0;
        for (i, c) in symbol.chars().rev().enumerate() {
            let char_value = (c as u128) - ('A' as u128) + self.base_offset as u128;
            value += char_value * 26u128.pow(i as u32);
        }
        Ok(value)
    }
}
