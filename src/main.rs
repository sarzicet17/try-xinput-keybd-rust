//! XInput コントローラーのボタン入力をキーボード入力に変換するツール
//!
//! Xbox 互換コントローラーの B ボタンを押すと X キーの入力として動作します。
//! 状態遷移に基づいた適切な KeyDown / KeyUp イベントを送信することで、
//! 「本物のキー長押し」として OS やアプリケーションから認識されます。

use std::thread::sleep;
use std::time::{Duration, Instant};

use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_X;

mod win32_input;
mod xinput;

use xinput::ButtonState;

/// XInput の状態をポーリングする間隔（5ミリ秒）
///
/// XInput は非同期イベントではなくポーリング方式で状態を取得するため、
/// 適切な間隔で定期的に状態を確認する必要があります。
const POLL_INTERVAL: Duration = Duration::from_millis(5);

/// XInput コントローラーの B ボタンを監視し、対応するキーボードイベントを送信する
fn main() {
    println!("XInput → Keyboard FSM started");

    // 前回のボタン状態（状態遷移の判定に使用）
    let mut prev = ButtonState::Released;
    // 最後にポーリングした時刻
    let mut last_poll = Instant::now();
    // エラーメッセージを一度だけ表示するためのフラグ
    let mut error_shown = false;

    loop {
        // ポーリング間隔の制御
        let now = Instant::now();
        if now.duration_since(last_poll) < POLL_INTERVAL {
            sleep(Duration::from_millis(1));
            continue;
        }
        last_poll = now;

        // XInput からボタン状態を取得
        match xinput::read_b_button() {
            Ok(current) => {
                error_shown = false;

                // 状態遷移に応じてキーイベントを送信（FSM）
                match (prev, current) {
                    (ButtonState::Released, ButtonState::Pressed) => {
                        // ボタン押下開始 → KeyDown 送信（1回のみ）
                        win32_input::send_key_down(VK_X as u16);
                        println!("KeyDown");
                    }
                    (ButtonState::Pressed, ButtonState::Released) => {
                        // ボタン解放 → KeyUp 送信（1回のみ）
                        win32_input::send_key_up(VK_X as u16);
                        println!("KeyUp");
                    }
                    _ => {
                        // 状態変化なし → 何も送信しない
                        // (Released, Released) または (Pressed, Pressed)
                    }
                }

                prev = current;
            }
            Err(err) if !error_shown => {
                // コントローラー未接続時などのエラーは1回だけ表示
                println!("XInput error: {}", err);
                error_shown = true;
            }
            Err(_) => {
                // エラーが継続している場合は何もしない
            }
        }
    }
}
