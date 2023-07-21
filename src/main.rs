use std::any::Any;
use std::collections::HashMap;
use gilrs::{Gilrs, Event, EventType, Button, GamepadId};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn process_input_of_gamepad_one() {

}

fn process_input_of_gamepad_two() {

}

fn process_input_of_gamepad_three() {

}

fn process_input_of_gamepad_four() {

}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut gamepads: Vec<GamepadId> = Vec::new();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("id: {}, name: {}, status: {:?}", gamepad.id(), gamepad.name(), gamepad.power_info());
        gamepads.push(_id);
    }

    let gamepad_one_id = gamepads.get(0);
    let gamepad_two_id = gamepads.get(1);
    let gamepad_three_id = gamepads.get(2);
    let gamepad_four_id = gamepads.get(3);

    loop {
        while let Some(Event{ id, event, time}) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);

            if gamepad_one_id.is_some() && *gamepad_one_id.unwrap() == id {
                process_input_of_gamepad_one();
            }

            if gamepad_two_id.is_some() && *gamepad_two_id.unwrap() == id {
                process_input_of_gamepad_two()
            }

            if gamepad_three_id.is_some() && *gamepad_three_id.unwrap() == id {
                process_input_of_gamepad_three()
            }

            if gamepad_four_id.is_some() && *gamepad_four_id.unwrap() == id {
                process_input_of_gamepad_four()
            }
        }
    }
}
