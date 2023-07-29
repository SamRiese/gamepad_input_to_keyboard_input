use enigo::*;
use gilrs::{Axis, Button, Event, Gamepad, GamepadId, Gilrs};

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

    if gamepad.value(Axis::LeftStickX) < -0.2 {
        enigo.key_click(left_key);
    } else if gamepad.value(Axis::LeftStickX) > 0.2 {
        enigo.key_click(right_key);
    }
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();

    let mut gamepads: Vec<GamepadId> = Vec::new();
    let gamepad_keys = vec![
        Keys::new(Key::Layout('w'), Key::Layout('a'),Key::Layout('d')),
        Keys::new(Key::UpArrow, Key::LeftArrow, Key::RightArrow),
        Keys::new(Key::Layout('i'), Key::Layout('j'), Key::Layout('l')),
        Keys::new(Key::Layout('8'), Key::Layout('4'), Key::Layout('6')),
    ];

    for (id, gamepad) in gilrs.gamepads() {
        eprintln!("id: {}, name: {}, status: {:?}", gamepad.id(), gamepad.name(), gamepad.power_info());
        gamepads.push(id);
    }

    loop {
        while let Some(Event{ id, event, time}) = gilrs.next_event() {
            eprintln!("{:?} New event from {}: {:?}", time, id, event);

            let gamepad = gilrs.gamepad(id);

            for (i, gamepad_key) in gamepad_keys.iter().enumerate() {
                if let Some(gamepad_id) = gamepads.get(i) {
                    if *gamepad_id == gamepad.id() {
                        process_gamepad_input(
                            gamepad,
                            &mut enigo,
                            gamepad_key.jump,
                            gamepad_key.left,
                            gamepad_key.right
                        );
                    }
                }
            }
        }
    }
}