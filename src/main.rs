mod myapp;
mod snake;
mod utils;

fn main() {
    let mut my_app = myapp::MyApp::new(400, 300);
    my_app.run_snake();

    // my_app.run();
}
