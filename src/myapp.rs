use std::{cell::RefCell, cmp::max, rc::Rc};

use fltk::{enums::*, prelude::*, window::DoubleWindow, *};

use crate::{
    consts,
    food::Food,
    snake::{self, Point},
    utils,
};
pub struct MyApp {
    _app: app::App,
    _snake: Rc<RefCell<snake::Snake>>, // 多所有者
    _window: DoubleWindow,
    _food: Food,
    _is_display: Rc<RefCell<bool>>,
    _is_watch: bool,
}

impl MyApp {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> MyApp {
        // init app style
        let a = app::App::default().with_scheme(app::Scheme::Plastic);

        // 渲染窗口
        let wind = MyApp::draw_window(x, y, w, h);

        // init snake/根据consts::BODY_SIZE 分为相应的份数
        let max_x = (w - consts::BODY_SIZE) / consts::BODY_SIZE;
        let max_y = (h - consts::BODY_SIZE) / consts::BODY_SIZE;
        let rand_x: i32 = utils::rand_range(0, max_x) * consts::BODY_SIZE;
        let rand_y = utils::rand_range(0, max_y) * consts::BODY_SIZE;
        let _snake = snake::Snake::new(rand_x, rand_y, wind.clone()); // 初始化snake

        MyApp {
            _app: a,
            _snake: Rc::new(RefCell::new(_snake)),
            _window: wind,
            _food: Food::new(0, 0),
            _is_display: Rc::new(RefCell::new(true)),
            _is_watch: false,
        }
    }

    pub fn run(&mut self) {
        if !self._is_watch {
            self.watch_key(); // 监听key
            self._is_watch = true;
        }
        self.init_food(); // 初始化food

        loop {
            if *(*self._is_display).borrow() {
                if self.is_eat_food() {
                    // panic!("eat_food");
                    self._snake.borrow_mut().add_body();
                    self.init_food();
                }

                app::sleep(0.20 - self._snake.borrow_mut().len() as f64 * 0.02); // sleep 时间决定了speed，长度越长，speed越快
                let result = self
                    ._snake
                    .borrow_mut()
                    .move_direction(consts::MOVE_STEP, false /*is_direction*/);

                if let Err(_) = result {
                    self.game_over();
                    break;
                }
                self.draw();
            } else {
                // 交出一点时间片。不然要卡死
                app::wait();
            }
        }
    }

    // 根据头节点判断是否吃到食物
    fn is_eat_food(&self) -> bool {
        let _snake = (*self._snake).borrow();
        let head = _snake.get_occupied_points().first().unwrap();

        // 间隔小于等于2倍body就是穿过了
        let x_space = max(
            head.x() + consts::BODY_SIZE - self._food.x(),
            self._food.x() + consts::BODY_SIZE - head.x(),
        );
        let y_space = max(
            head.y() + consts::BODY_SIZE - self._food.y(),
            self._food.y() + consts::BODY_SIZE - head.y(),
        );
        println!(
            "x_space:{},y_space:{},head.x:{},head.y:{},_food.x:{},food.y:{}",
            x_space,
            y_space,
            head.x(),
            head.y(),
            self._food.x(),
            self._food.y()
        );

        (head.x() == self._food.x() || head.y() == self._food.y()/*在同一条线*/)
            && (x_space <= 2 * consts::BODY_SIZE && y_space <= 2 * consts::BODY_SIZE/*有交叉*/)
    }

    fn game_over(&mut self) {
        *self._is_display.borrow_mut() = false;
        (*self._snake).borrow_mut().clear(); // 清除数据，重启的话重新开始

        // 绘画结束ui
        let width = self._window.width();
        let height = self._window.height();
        app::awake(); // 唤醒ui线程
        self._window.draw(move |f| {
            // 在 draw 中实现绘制逻辑，此处是根据缓存绘制
            // 绘制背景
            draw::set_draw_color(Color::Dark3);
            draw::draw_rectf(0, 0, width, height);
            // 设置字体和颜色
            draw::set_font(Font::HelveticaBold, 30);
            draw::set_draw_color(Color::White);
            // 在屏幕中央绘制 "Game Over" 文字
            let text = "Game Over";
            let (text_width, text_height) = draw::measure(text, true);
            let x = (width - text_width) / 2;
            let y = (height + text_height) / 2;
            draw::draw_text(text, x, y);
        });
        self._window.redraw();
        app::wait();
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
                draw::draw_rectf(point.x(), point.y(), consts::BODY_SIZE, consts::BODY_SIZE);
                draw::draw_circle_fill(food.x(), food.y(), consts::BODY_SIZE, Color::DarkYellow)
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

    // 初始化食物
    fn init_food(&mut self) {
        let occupied_points = (*self._snake).borrow().get_occupied_points().to_vec();
        // 分成对应的份数
        let max_x = (self._window.w() - consts::BODY_SIZE) / consts::BODY_SIZE;
        let max_y = (self._window.h() - consts::BODY_SIZE) / consts::BODY_SIZE;

        // 剩下的坐标点
        let all_points: Vec<Point> = (0..max_x)
            .flat_map(|x| {
                (0..max_y)
                    .map(move |y| snake::Point::new(x * consts::BODY_SIZE, y * consts::BODY_SIZE))
            })
            .filter(|point| !occupied_points.contains(point))
            .collect();
        let food_point = all_points
            .get(utils::rand_range(0, all_points.len()))
            .unwrap();
        self._food = Food::new(food_point.x(), food_point.y());
    }

    fn watch_key(&mut self) {
        let _snake = Rc::clone(&self._snake);
        let _display = Rc::clone(&self._is_display);
        let _food = self._food.clone();

        self._window.handle(move |w, ev| {
            match ev {
                Event::KeyDown => {
                    let key = app::event_key();
                    match key {
                        Key::Up => _snake
                            .borrow_mut()
                            .set_direction(snake::Direction::Up)
                            .unwrap(),
                        Key::Down => _snake
                            .borrow_mut()
                            .set_direction(snake::Direction::Down)
                            .unwrap(),
                        Key::Left => _snake
                            .borrow_mut()
                            .set_direction(snake::Direction::Left)
                            .unwrap(),
                        Key::Right => _snake
                            .borrow_mut()
                            .set_direction(snake::Direction::Right)
                            .unwrap(),
                        other_key => {
                            // pause
                            if other_key.bits() == 0x20 {
                                let mut is_display = _display.borrow_mut();
                                *is_display = !*is_display;
                                return true;
                            }
                        }
                    }
                    // 移动完马上渲染一次
                    app::awake(); // 唤醒ui线程
                    _snake
                        .borrow_mut()
                        .move_direction(consts::MOVE_STEP, true /*is_direction*/)
                        .unwrap();
                    app::wait();
                    true
                }
                _ => false, // 返回 false 表示未处理其他事件
            }
        });

        // self._window.set_callback(Box::new(x));
    }
}
