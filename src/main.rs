use std::thread::sleep;
use std::time::{Duration, Instant};

use windows_sys::Win32::{
    UI::Input::{
        KeyboardAndMouse::{
            SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
            KEYEVENTF_KEYUP, VK_X,
        },
        XboxController::{
            XInputGetState, XINPUT_STATE, XINPUT_GAMEPAD_B,
        },
    },
    System::Threading::Sleep,
};

/// ボタンの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Released,
    Pressed,
}

const POLL_INTERVAL: Duration = Duration::from_millis(5);

fn main() {
    println!("XInput to Keyboard converter started");
    println!("Press Xbox Controller B button to send X key");
    println!("Press Ctrl+C to exit");

    let mut prev_button_state = ButtonState::Released;
    let mut last_poll = Instant::now();
    let mut error_shown = false;

    loop {
        // ポーリング間隔制御（Instantベース）
        let now = Instant::now();
        if now.duration_since(last_poll) < POLL_INTERVAL {
            sleep(Duration::from_millis(1));
            continue;
        }
        last_poll = now;

        // XInput 状態取得
        let mut xinput_state: XINPUT_STATE = unsafe { std::mem::zeroed() };
        let result = unsafe { XInputGetState(0, &mut xinput_state) };

        if result == 0 {
            error_shown = false;

            let pressed =
                (xinput_state.Gamepad.wButtons & XINPUT_GAMEPAD_B) != 0;

            let current_button_state = if pressed {
                ButtonState::Pressed
            } else {
                ButtonState::Released
            };

            // 状態遷移 FSM
            match (prev_button_state, current_button_state) {
                (ButtonState::Released, ButtonState::Pressed) => {
                    send_key_event(VK_X as u16, false);
                    println!("Button pressed -> KeyDown (X)");
                }
                (ButtonState::Pressed, ButtonState::Released) => {
                    send_key_event(VK_X as u16, true);
                    println!("Button released -> KeyUp (X)");
                }
                _ => {}
            }

            prev_button_state = current_button_state;
        } else if !error_shown {
            println!("XInput controller not found (error code: {})", result);
            error_shown = true;
        }
    }
}

/// キーイベント送信
fn send_key_event(virtual_key: u16, is_key_up: bool) {
    let mut input: INPUT = unsafe { std::mem::zeroed() };
    input.r#type = INPUT_KEYBOARD;

    unsafe {
        input.Anonymous.ki = KEYBDINPUT {
            wVk: virtual_key,
            wScan: 0,
            dwFlags: if is_key_up { KEYEVENTF_KEYUP } else { 0 },
            time: 0,
            dwExtraInfo: 0,
        };

        let sent = SendInput(
            1,
            &input,
            std::mem::size_of::<INPUT>() as i32,
        );

        if sent != 1 {
            eprintln!("SendInput failed");
        }
    }
}
