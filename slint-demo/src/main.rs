slint::include_modules!();

fn main() {
    let app = App::new().unwrap();
    let weak_app = app.as_weak().unwrap();
    app.on_create_qr_code(move||{
        weak_app.set_plate("test".into());
    });
   let _= app.run();
}
