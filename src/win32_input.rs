use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP,
};

/// 指定された仮想キーコードのキーダウンイベントを送信する
///
/// # 引数
///
/// - `vk`: Win32の仮想キーコード（例: 0x58 = 'X'キー）
pub fn send_key_down(vk: u16) {
    send(vk, false);
}

/// 指定された仮想キーコードのキーアップイベントを送信する
///
/// # 引数
///
/// - `vk`: Win32の仮想キーコード（例: 0x58 = 'X'キー）
pub fn send_key_up(vk: u16) {
    send(vk, true);
}

fn send(vk: u16, key_up: bool) {
    let mut input: INPUT = unsafe { std::mem::zeroed() };
    input.r#type = INPUT_KEYBOARD;

    unsafe {
        input.Anonymous.ki = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: if key_up { KEYEVENTF_KEYUP } else { 0 },
            time: 0,
            dwExtraInfo: 0,
        };

        SendInput(
            1,
            &input,
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}
