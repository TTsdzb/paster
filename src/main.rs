// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use slint::{EventLoopError, SharedString, Weak};

const ONE_SECOND: Duration = Duration::from_secs(1);

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_input(move || {
        let ui = ui_handle.upgrade().unwrap();

        let input_text = ui.get_input_text();
        let secs: u64 = ui.get_secs().parse().unwrap();
        println!("Secs: {}", &secs);
        println!("Text: {}", &input_text);

        // 释放一个新线程，避免界面无响应
        thread::spawn(thread_callback(ui_handle.clone(), input_text, secs));
    });

    ui.run()
}

fn thread_callback(ui_handle: Weak<AppWindow>, input_text: SharedString, secs: u64) -> impl Fn() {
    move || {
        let (tx, rx) = mpsc::channel();
        let mut countdown = secs;
        while countdown > 0 {
            thread::sleep(ONE_SECOND);
            countdown -= 1;
            send_canceled(&ui_handle, tx.clone()).unwrap();
            let canceled = rx.recv().unwrap();
            if canceled {
                reset_ui(&ui_handle, secs).unwrap();
                return;
            }
            display_secs(&ui_handle, countdown).unwrap();
        }
        println!("{}", &input_text);
        reset_ui(&ui_handle, secs).unwrap();
    }
}

fn send_canceled(ui_handle: &Weak<AppWindow>, tx: Sender<bool>) -> Result<(), EventLoopError> {
    ui_handle.upgrade_in_event_loop(move |ui| {
        tx.send(ui.get_canceled()).unwrap();
    })?;
    Ok(())
}

fn reset_ui(ui_handle: &Weak<AppWindow>, secs: u64) -> Result<(), EventLoopError> {
    ui_handle.upgrade_in_event_loop(move |ui| {
        ui.set_secs(format!("{}", secs).into());
        ui.set_started(false);
    })?;
    Ok(())
}

fn display_secs(ui_handle: &Weak<AppWindow>, secs: u64) -> Result<(), EventLoopError> {
    ui_handle.upgrade_in_event_loop(move |ui| {
        ui.set_secs(format!("{}", secs).into());
    })?;
    Ok(())
}
