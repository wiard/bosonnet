pub mod ui_layer;
pub mod observer;
pub mod api;

pub fn init_ui() {
    println!("User Interface initialized.");
    ui_layer::start_ui();
}

