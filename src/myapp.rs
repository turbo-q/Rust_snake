use std::{borrow::Borrow, cell::RefCell, ops::Deref, rc::Rc};

use fltk::{enums::*, prelude::*, window::DoubleWindow, *};
use rand::seq::SliceRandom;

// 移动步伐大小
const MOVE_STEP: i32 = 2;

use crate::{
    snake::{self, Point},
    utils,
};
pub struct MyApp {
    _app: app::App,
    _snake: Rc<RefCell<snake::Snake>>, // 多所有者
    _window: DoubleWindow,
}

impl MyApp {
    pub fn new(w: i32, h: i32) -> MyApp {
        // init app style
        let a = app::App::default().with_scheme(app::Scheme::Plastic);

        // 渲染窗口
        let wind = MyApp::draw_window(w, h);

        // init snake
        let min_x = wind.x() + MOVE_STEP;
        let max_x = wind.x() + w;
        let min_y = wind.y() + MOVE_STEP;
        let max_y = wind.y() + h;

        let rand_x = utils::rand_range(min_x, max_x);
        let rand_y = utils::rand_range(min_y, max_y);
        let _snake = snake::Snake::new(rand_x, rand_y, wind.clone()); // 初始化snake

        // init food

        MyApp {
            _app: a,
            _snake: Rc::new(RefCell::new(_snake)),
            _window: wind,
        }
    }

    // fn get_available_point(&self) -> Point {
    //     let min_x = self._window.x() + MOVE_STEP;
    //     let max_x = self._window.x() + self._window.w();
    //     let min_y = self._window.y() + MOVE_STEP;
    //     let max_y = self._window.y() + self._window.h();

    //     let snake = self._snake.borrow(); // study 必须先声明，创建一个临时变量，确保引用结束之前值不会被释放
    //     let points = snake.get_occupied_points(); //
    //                                               // 生成未占用坐标点的列表初始化食物
    //     let available_points: Vec<snake::Point> = (min_x..max_x)
    //         .flat_map(|x| (min_y..max_y).map(move |y| snake::Point::new(x, y)))
    //         .filter(|point| !points.contains(point))
    //         .collect();
    //     available_points
    //         .choose(&mut rand::thread_rng())
    //         .unwrap()
    //         .to_point()
    // }

    fn draw(&mut self) {
        // 获取snake 点位
        let points = (*self._snake).borrow().get_occupied_points().to_vec();

        // todo 获取food点位
        let x = 100;
        let y = 100;

        // draw
        self._window.draw(move |f| {
            // 在 draw 中实现绘制逻辑，此处是根据缓存绘制
            for point in &points {
                draw::set_draw_color(Color::Red);
                draw::draw_rectf(point.x(), point.y(), snake::BODY_SIZE, snake::BODY_SIZE);
                draw::draw_circle_fill(x, y, snake::BODY_SIZE, Color::DarkYellow)
            }
        });
        self._window.redraw();
    }

    fn draw_window(w: i32, h: i32) -> window::DoubleWindow {
        // init
        let mut wind: window::DoubleWindow = window::Window::new(0, 0, w, h, "Rust_snake");
        wind.set_color(Color::Black); // 设置颜色
                                      // wind.set_border(false); // 无边框
                                      // wind.fullscreen(true); // 全屏
        wind.end();
        wind.show();
        wind
    }

    pub fn run_snake(&mut self) {
        self.watch_key();
        loop {
            app::sleep(0.22 - self._snake.borrow_mut().len() as f64 * 0.02); // sleep 时间决定了speed，长度越长，speed越快
            app::awake(); // 唤醒ui线程
                          // self.draw_food();
            self._snake
                .borrow_mut()
                .move_direction(MOVE_STEP, false /*is_direction*/)
                .unwrap();

            self.draw();
            println!("{}", Rc::strong_count(&self._snake));
            app::wait();
        }
    }

    fn watch_key(&mut self) {
        let _snake = Rc::clone(&self._snake);
        self._window.handle(move |_, ev| {
            match ev {
                Event::KeyDown => {
                    let key = app::event_key();
                    match key {
                        Key::Up => _snake.borrow_mut().set_direction(snake::Direction::Up),
                        Key::Down => _snake.borrow_mut().set_direction(snake::Direction::Down),
                        Key::Left => _snake.borrow_mut().set_direction(snake::Direction::Left),
                        Key::Right => _snake.borrow_mut().set_direction(snake::Direction::Right),
                        _ => (),
                    }
                    // 移动完马上渲染一次
                    app::awake(); // 唤醒ui线程
                    _snake
                        .borrow_mut()
                        .move_direction(MOVE_STEP, true /*is_direction*/)
                        .unwrap();
                    app::wait();
                    true
                }
                _ => false, // 返回 false 表示未处理其他事件
            }
        })
    }

    // 主线程run，不需要？会主动调用？
    // pub fn run(&self) {
    // self.app.run().unwrap()
    // }
}
