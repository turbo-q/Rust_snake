use std::{cell::RefCell, rc::Rc};

use fltk::{enums::*, prelude::*, window::DoubleWindow, *};

// 移动步伐大小
const MOVE_STEP: i32 = 1;

use crate::{
    food::Food,
    snake::{self, Point, BODY_SIZE},
    utils,
};
pub struct MyApp {
    // 不知道为什么直接从_window.x()/_window.y()获取数据不对，所以只用冗余
    _window_x: i32,
    _window_y: i32,
    _app: app::App,
    _snake: Rc<RefCell<snake::Snake>>, // 多所有者
    _window: DoubleWindow,
    _food: Food,
}

impl MyApp {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> MyApp {
        // init app style
        let a = app::App::default().with_scheme(app::Scheme::Plastic);

        // 渲染窗口
        let wind = MyApp::draw_window(x, y, w, h);

        // init snake
        let min_x = x;
        let max_x = x + w - BODY_SIZE;
        let min_y = y;
        let max_y = y + h - BODY_SIZE;
        let rand_x: i32 = utils::rand_range(min_x, max_x);
        let rand_y = utils::rand_range(min_y, max_y);
        let _snake = snake::Snake::new(x, y, rand_x, rand_y, wind.clone()); // 初始化snake

        MyApp {
            _window_x: x,
            _window_y: y,
            _app: a,
            _snake: Rc::new(RefCell::new(_snake)),
            _window: wind,
            _food: Food::new(0, 0),
        }
    }

    // 绘画统一在这里处理
    fn draw(&mut self) {
        // 获取snake 点位
        let points = (*self._snake).borrow().get_occupied_points().to_vec();

        // 获取food点位
        let food = self._food.clone();

        // draw
        app::awake(); // 唤醒ui线程
        self._window.draw(move |f| {
            // 在 draw 中实现绘制逻辑，此处是根据缓存绘制
            for point in &points {
                draw::set_draw_color(Color::Red);
                draw::draw_rectf(point.x(), point.y(), snake::BODY_SIZE, snake::BODY_SIZE);
                draw::draw_circle_fill(food.x(), food.y(), snake::BODY_SIZE, Color::DarkYellow)
            }
        });
        self._window.redraw();
        app::wait();
    }

    fn draw_window(x: i32, y: i32, w: i32, h: i32) -> window::DoubleWindow {
        // init
        let mut wind: window::DoubleWindow = window::Window::new(x, y, w, h, "Rust_snake");
        wind.set_color(Color::Black); // 设置颜色
        wind.set_border(false); // 无边框
                                // wind.fullscreen(true); // 全屏

        wind.end();
        wind.show();
        wind
    }

    pub fn run_snake(&mut self) {
        self.init_food(); // 初始化food
        self.watch_key(); // 监听key
        loop {
            app::sleep(0.12 - self._snake.borrow_mut().len() as f64 * 0.02); // sleep 时间决定了speed，长度越长，speed越快
            self._snake
                .borrow_mut()
                .move_direction(MOVE_STEP, false /*is_direction*/)
                .unwrap();
            self.draw();
        }
    }

    // 初始化食物
    fn init_food(&mut self) {
        let occupied_points = (*self._snake).borrow().get_occupied_points().to_vec();
        let min_x = self._window_x;
        let max_x = self._window_x + self._window.w() - BODY_SIZE;
        let min_y = self._window_y;
        let max_y = self._window_y + self._window.h() - BODY_SIZE;

        // 剩下的坐标点
        let all_points: Vec<Point> = (min_x..max_x)
            .flat_map(|x| (min_y..max_y).map(move |y| snake::Point::new(x, y)))
            .filter(|point| !occupied_points.contains(point))
            .collect();
        let food_point = all_points
            .get(utils::rand_range(0, all_points.len()))
            .unwrap();
        self._food = Food::new(food_point.x(), food_point.y());
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
