mod consts;
mod food;
mod myapp;
mod snake;
mod utils;

fn main() {
    let mut my_app = myapp::MyApp::new(0, 0, 50, 50);
    loop {
        // 结束后仍然可以重启
        my_app.run();
    }
}
