use crate::status_enum::*;
use crate::clock_out_status::*;
use chrono::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calc_do_laundry() {
        assert_eq!(true, calc_do_laundry(1.5));
    }

    #[test]
    fn test_calc_do_laundry_false() {
        assert_eq!(false, calc_do_laundry(1.4));
    }
}

pub fn calc_actions(status: ClockOutStatus) -> Vec<String> {
    let dinner_options = calc_dinner(status.stress, status.time);
    let do_laundry = calc_do_laundry(status.laundry_amount);
    let actions = calc_order_of_actions(dinner_options, do_laundry, status.hunger);
    actions
}

fn calc_order_of_actions(dinner_options: Dinner, do_laundry: bool, hunger: Hunger) -> Vec<String> {
    let mut actions = Vec::<String>::new();
    if do_laundry {
        actions.push(String::from("Do Laundry"));
    };
    if hunger == Hunger::ALittle {
        actions.push(String::from("Training"));
        actions.push(String::from("Dinner"));
    } else {
        actions.push(String::from("Dinner"));
        actions.push(String::from("Training"));
    }
    actions
}

fn calc_do_laundry(laundry_amount: f32) -> bool {
    if laundry_amount < 1.5 {
        false
    } else {
        true
    }
}

fn calc_dinner(stress: Stress, time: DateTime<Local>) -> Dinner {
    let dinner = match stress {
        Stress::High => match time.hour() {
            0..=19 => Dinner::Rice,
            _ => Dinner::NabeSet,
        },
        Stress::Low => match time.hour() {
            0..=19 => Dinner::Rice,
            _ => Dinner::Convinience
        }
    };
    println!("Dinner Options: {:?}", dinner);
    dinner
}