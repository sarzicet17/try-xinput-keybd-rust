use windows_sys::Win32::UI::Input::XboxController::{
    XInputGetState, XINPUT_STATE, XINPUT_GAMEPAD_B,
};

/// XInputコントローラーのボタンの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// ボタンが押されていない状態
    Released,
    /// ボタンが押されている状態
    Pressed,
}

/// XInputコントローラー（UserIndex 0）のBボタンの現在の状態を取得する
///
/// # 戻り値
///
/// - `Ok(ButtonState)`: ボタンの状態（`Pressed` または `Released`）
/// - `Err(u32)`: XInput APIのエラーコード（例: コントローラーが接続されていない場合）
pub fn read_b_button() -> Result<ButtonState, u32> {
    let mut state: XINPUT_STATE = unsafe { std::mem::zeroed() };
    let result = unsafe { XInputGetState(0, &mut state) };

    if result != 0 {
        return Err(result);
    }

    let pressed =
        (state.Gamepad.wButtons & XINPUT_GAMEPAD_B) != 0;

    Ok(if pressed {
        ButtonState::Pressed
    } else {
        ButtonState::Released
    })
}
