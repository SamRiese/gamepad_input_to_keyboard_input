use std::any::Any;
use std::collections::HashMap;
use gilrs::{Gilrs, Event, EventType, Button, GamepadId};

fn match_id(id: u32){
    print!("{}", id);
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut controller: Vec<GamepadId> = Vec::new();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("id: {}, name: {}, status: {:?}", gamepad.id(), gamepad.name(), gamepad.power_info());
        controller.push(_id);
    }

    loop {
        while let Some(Event{ id, event, time}) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);

            match_id(1);
        }
    }
}
