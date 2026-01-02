use windows_sys::Win32::{
    UI::Input::{
        KeyboardAndMouse::{
            SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
            KEYEVENTF_KEYUP,
        },
        XboxController::{
            XInputGetState, XINPUT_STATE, XINPUT_GAMEPAD_B,
        },
    },
    System::Threading::Sleep,
};

/// ボタンの状態
#[derive(Debug, Clone, Copy, PartialEq)]
enum ButtonState {
    Released,
    Pressed,
}

fn main() {
    println!("XInput to Keyboard converter started");
    println!("Press Xbox Controller B button to send X key");
    println!("Press Ctrl+C to exit");

    let mut prev_button_state = ButtonState::Released;

    loop {
        // XInputからコントローラーの状態を取得
        let mut xinput_state: XINPUT_STATE = unsafe { std::mem::zeroed() };
        let result = unsafe { XInputGetState(0, &mut xinput_state) };

        if result == 0 {  // ERROR_SUCCESS
            // Bボタンの状態を確認
            let b_button_pressed = (xinput_state.Gamepad.wButtons & XINPUT_GAMEPAD_B) != 0;
            
            let current_button_state = if b_button_pressed {
                ButtonState::Pressed
            } else {
                ButtonState::Released
            };

            // 状態遷移とキーイベント送信
            match (prev_button_state, current_button_state) {
                (ButtonState::Released, ButtonState::Pressed) => {
                    // ボタン押下開始 -> KeyDown送信
                    send_key_event(0x58, false);  // 0x58 = 'X' key
                    println!("Button pressed - Sending KeyDown (X)");
                }
                (ButtonState::Pressed, ButtonState::Released) => {
                    // ボタン解放 -> KeyUp送信
                    send_key_event(0x58, true);  // 0x58 = 'X' key
                    println!("Button released - Sending KeyUp (X)");
                }
                _ => {
                    // 状態変化なし -> 何もしない
                }
            }

            prev_button_state = current_button_state;
        } else {
            // コントローラーが接続されていない場合
            // エラーメッセージは最初の1回だけ表示
            static mut ERROR_SHOWN: bool = false;
            unsafe {
                if !ERROR_SHOWN {
                    println!("XInput controller not found (error code: {})", result);
                    ERROR_SHOWN = true;
                }
            }
        }

        // 5ms待機
        unsafe { Sleep(5) };
    }
}

/// キーイベントを送信
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
        
        let result = SendInput(1, &input, std::mem::size_of::<INPUT>() as i32);
        if result != 1 {
            eprintln!("Failed to send input: {}", result);
        }
    }
}