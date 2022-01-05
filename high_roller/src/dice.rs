use actix_web::{get, web, Responder};
use serde::{Serialize};
use rand::Rng;

#[derive(Copy, Clone, Eq, PartialEq, Serialize)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[derive(Serialize)]
pub struct Dice {
    rolled_value: u32,
    type_: DiceType,
}

#[derive(Debug, Serialize)]
struct MyError {
    name: &'static str,
}

#[get("/dice/{dice}/roll")]
async fn roll_dice(web::Path(dice_type): web::Path<String>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let dice_roll: Dice;

    match &*dice_type {
        "D4" => dice_roll = Dice{rolled_value: rng.gen_range(1..4), type_: DiceType::D4},
        "D6" => dice_roll = Dice{rolled_value: rng.gen_range(1..6), type_: DiceType::D6},
        "D8" => dice_roll = Dice{rolled_value: rng.gen_range(1..8), type_: DiceType::D8},
        "D10" => dice_roll = Dice{rolled_value: rng.gen_range(0..9), type_: DiceType::D10},
        "D12" => dice_roll = Dice{rolled_value: rng.gen_range(1..12), type_: DiceType::D12},
        "D20" => dice_roll = Dice{rolled_value: rng.gen_range(1..20), type_: DiceType::D20},
        _ => return web::Json(Err(MyError{name: "unrecognised dice type"}))
    }

    return web::Json(Ok(dice_roll));
}