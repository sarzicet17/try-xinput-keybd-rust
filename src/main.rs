use std::thread::sleep;
use std::time::{Duration, Instant};

use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_X;

mod win32_input;
mod xinput;

use xinput::ButtonState;

const POLL_INTERVAL: Duration = Duration::from_millis(5);

fn main() {
    println!("XInput → Keyboard FSM started");

    let mut prev = ButtonState::Released;
    let mut last_poll = Instant::now();
    let mut error_shown = false;

    loop {
        let now = Instant::now();
        if now.duration_since(last_poll) < POLL_INTERVAL {
            sleep(Duration::from_millis(1));
            continue;
        }
        last_poll = now;

        match xinput::read_b_button() {
            Ok(current) => {
                error_shown = false;

                match (prev, current) {
                    (ButtonState::Released, ButtonState::Pressed) => {
                        win32_input::send_key_down(VK_X as u16);
                        println!("KeyDown");
                    }
                    (ButtonState::Pressed, ButtonState::Released) => {
                        win32_input::send_key_up(VK_X as u16);
                        println!("KeyUp");
                    }
                    _ => {}
                }

                prev = current;
            }
            Err(err) if !error_shown => {
                println!("XInput error: {}", err);
                error_shown = true;
            }
            Err(_) => {}
        }
    }
}
