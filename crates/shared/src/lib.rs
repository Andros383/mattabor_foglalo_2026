use std::thread::panicking;

use rhai::Engine;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TextPayload {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Ruleset {
    pub symbol_values: Vec<(char, f32)>,
    pub concat_value: f32,
    // Mennyi pontot veszít a távolság miatt
    pub distance_penalty: f32,
}

impl Default for Ruleset {
    fn default() -> Self {
        Self {
            symbol_values: Vec::from([
                ('0', 1.0),
                ('1', 1.0),
                ('2', 1.0),
                ('3', 1.0),
                ('4', 1.0),
                ('5', 1.0),
                ('6', 1.0),
                ('7', 1.0),
                ('8', 1.0),
                ('9', 1.0),
                ('+', 5.0),
                ('-', 5.0),
                ('*', 10.0),
            ]),
            concat_value: 1000.,
            distance_penalty: 1.0,
        }
    }
}

pub fn calculate_score(inp: &str, rules: Ruleset) -> f32 {
    let mut sum = 0.0;
    let chars = inp.chars();
    for char in chars {
        for (symbol, value) in &rules.symbol_values {
            if *symbol == char {
                sum += value;
            }
        }
    }

    // Concat számolás
    let x: Vec<char> = inp.chars().collect();
    for pair in x.windows(2) {
        if pair[0].is_numeric() && pair[1].is_numeric() {
            sum += rules.concat_value;
        }
    }

    sum
}

pub fn calculate_number(inp: &str) -> Result<i64, String> {
    let engine = Engine::new();
    engine.eval::<i64>(inp).map_err(|e| e.to_string())
}
