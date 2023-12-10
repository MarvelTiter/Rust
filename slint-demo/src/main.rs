#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let app = App::new().unwrap();
    let weak_app = app.as_weak().unwrap();
    app.on_create_qr_code(move|plate, pclass|{
        println!("{} - {}", plate, pclass)
    });
   app.run()
}
