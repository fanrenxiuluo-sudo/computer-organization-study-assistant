// 只在发布模式下禁用控制台窗口，开发模式保留控制台用于调试
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    computer_organization_study_assistant_lib::run()
}

