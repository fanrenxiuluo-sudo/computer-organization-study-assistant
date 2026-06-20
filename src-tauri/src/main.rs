// 只在发布模式下禁用控制台窗口，开发模式保留控制台用于调试
// 注意：禁用控制台后 panic/log 信息不可见，已在下方设置 panic hook 写入日志
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR};

/// 获取崩溃日志文件路径
fn crash_log_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("计组备考助手"));
    base.join("计组备考助手").join("crash.log")
}

/// 在 Windows 上弹出错误消息框（UTF-16）
fn show_error_message(msg: &str) {
    let wide_msg: Vec<u16> = msg.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_title: Vec<u16> = "计组备考助手 - 错误"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide_msg.as_ptr(),
            wide_title.as_ptr(),
            MB_ICONERROR,
        );
    }
}

fn main() {
    let log_path = crash_log_path();
    if let Some(parent) = log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    std::panic::set_hook(Box::new(move |info| {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| format!("{}", d.as_secs()))
            .unwrap_or_else(|_| "unknown".into());
        let msg = format!(
            "[{}] 应用崩溃\n信息: {}\n位置: {:?}\n",
            timestamp,
            info.to_string(),
            info.location(),
        );
        let _ = fs::File::create(&log_path).and_then(|mut f| f.write_all(msg.as_bytes()));
        show_error_message(&format!(
            "应用遇到问题需要关闭。\n\n错误详情已保存至：\n{}\n\n请将此文件发送给开发者。",
            log_path.display()
        ));
    }));

    computer_organization_study_assistant_lib::run()
}

