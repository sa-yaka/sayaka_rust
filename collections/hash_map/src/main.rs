#![allow(unused)]

use std::collections::HashMap;
fn main() {
    let team_name = vec![
        "fire".to_string(),
        "water".to_string(),
        "elec".to_string(),
        "elec".to_string(),
        "elec".to_string(),
    ];
    let team_id = vec![1, 2, 3];
    let map: HashMap<_, _> = team_name.iter().zip(team_id.iter()).collect();
}
