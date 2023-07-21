use std::any::Any;
use std::collections::HashMap;
use gilrs::{Gilrs, Event, EventType, Button, GamepadId, Gamepad, Axis};
use enigo::*;

struct Keys {
    jump: Key,
    left: Key,
    right: Key
}

impl Keys {
    fn new(jump_key: Key, left_key: Key, right_key: Key) -> Self {
        Self {
            jump: jump_key,
            left: left_key,
            right: right_key,
        }
    }
}

fn process_gamepad_input(gamepad: Gamepad, enigo: &mut Enigo, jump_key: Key,
                                left_key: Key, right_key: Key) {
    if gamepad.is_pressed(Button::South) {
        enigo.key_click(jump_key);
    }

    if gamepad.value(Axis::LeftStickX) < -0.2{
        enigo.key_click(left_key);
    } else if gamepad.value(Axis::LeftStickX) > 0.2 {
        enigo.key_click(right_key);
    }
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();
    let mut gamepads: Vec<GamepadId> = Vec::new();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("id: {}, name: {}, status: {:?}", gamepad.id(), gamepad.name(), gamepad.power_info());
        gamepads.push(_id);
    }

    let gamepad_one_id = gamepads.get(0);
    let gamepad_one_keys = Keys::new(Key::Layout('w'), Key::Layout('a'),Key::Layout('d'));

    let gamepad_two_id = gamepads.get(1);
    let gamepad_two_keys = Keys::new(Key::UpArrow, Key::LeftArrow, Key::RightArrow);

    let gamepad_three_id = gamepads.get(2);
    let gamepad_three_keys = Keys::new(Key::Layout('i'), Key::Layout('j'), Key::Layout('l'));

    let gamepad_four_id = gamepads.get(3);
    let gamepad_four_keys = Keys::new(Key::Layout('8'), Key::Layout('4'), Key::Layout('6'));

    loop {
        while let Some(Event{ id, event, time}) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);

            let gamepad = gilrs.gamepad(id);

            if gamepad_one_id.is_some() && *gamepad_one_id.unwrap() == gamepad.id() {
                process_gamepad_input(gamepad, &mut enigo, gamepad_one_keys.jump,
                                      gamepad_one_keys.left, gamepad_one_keys.right);
            }

            if gamepad_two_id.is_some() && *gamepad_two_id.unwrap() == gamepad.id() {
                process_gamepad_input(gamepad, &mut enigo, gamepad_two_keys.jump,
                                      gamepad_two_keys.left, gamepad_two_keys.right);
            }

            if gamepad_three_id.is_some() && *gamepad_three_id.unwrap() == gamepad.id() {
                process_gamepad_input(gamepad, &mut enigo, gamepad_three_keys.jump,
                                      gamepad_three_keys.left, gamepad_three_keys.right);
            }

            if gamepad_four_id.is_some() && *gamepad_four_id.unwrap() == gamepad.id() {
                process_gamepad_input(gamepad, &mut enigo, gamepad_four_keys.jump,
                                      gamepad_four_keys.left, gamepad_four_keys.right);
            }
        }
    }
}
