use actix_web::{get, web, Responder};
use serde::{Serialize};
use rand::Rng;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Debug)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[derive(Serialize, Debug)]
pub struct Dice {
    rolled_value: u32,
    type_: DiceType,
}

#[get("/dice/{dice}/roll")]
async fn roll_dice_controller(web::Path(dice_type): web::Path<String>) -> impl Responder {
    return web::Json(roll_dice(dice_type));
}

fn roll_dice(dice_type: String) -> Result<Dice, String> {
    let mut rng = rand::thread_rng();
    let dice_roll: Dice;

    match &*dice_type {
        "D4" => dice_roll = Dice{rolled_value: rng.gen_range(1..4), type_: DiceType::D4},
        "D6" => dice_roll = Dice{rolled_value: rng.gen_range(1..6), type_: DiceType::D6},
        "D8" => dice_roll = Dice{rolled_value: rng.gen_range(1..8), type_: DiceType::D8},
        "D10" => dice_roll = Dice{rolled_value: rng.gen_range(0..9), type_: DiceType::D10},
        "D12" => dice_roll = Dice{rolled_value: rng.gen_range(1..12), type_: DiceType::D12},
        "D20" => dice_roll = Dice{rolled_value: rng.gen_range(1..20), type_: DiceType::D20},
        _ => return Err(format!("unrecognised dice type {}", dice_type))
    }
    Ok(dice_roll)
}

#[cfg(test)]
mod dice_tests {
    use super::*;

    #[test]
    fn roll_dice_when_invalid_dicetype_then_return_error() {
        // given
        let dice_type: &str = "D100";

        // when
        let response = roll_dice(dice_type.to_string());

        // then
        assert_eq!(false, response.is_ok());
        let expected_error = format!("unrecognised dice type {}", dice_type.to_string());
        assert_eq!(expected_error, response.err().unwrap());
    }

    #[test]
    fn roll_dice_when_valid_d6_dicetype_then_return_dice_roll() {
        // given
        let dice_type: String = String::from("D6");

        // when
        let response = roll_dice(dice_type);

        // then
        assert_eq!(true, response.is_ok());
        let dice_roll = response.unwrap();
        assert_eq!(DiceType::D6, dice_roll.type_);
        let dice_roll_range = 1..6;
        assert!(dice_roll_range.contains(&dice_roll.rolled_value));
    }
}