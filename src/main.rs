mod consts;
mod food;
mod myapp;
mod snake;
mod utils;

fn main() {
    let mut my_app = myapp::MyApp::new(0, 0, 400, 300);
    my_app.run();

    // my_app.run();
}
