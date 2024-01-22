use std::{borrow::Borrow, ops::Deref, rc::Rc};

use fltk::{enums::Color, prelude::*, *};

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// body大小，小方框
pub const BODY_SIZE: i32 = 10;

#[derive(PartialEq, Clone)]
pub struct Point {
    x: i32, // x方向位置
    y: i32, // y方向位置
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}

// snake
pub struct Snake {
    len: i32,
    direction: Direction, // 移动方向
    window: window::DoubleWindow,
    occupied_points: Vec<Point>, // 已经占用的点
}

impl Snake {
    pub fn new(x: i32, y: i32, window: window::DoubleWindow) -> Snake {
        Snake {
            len: 1,
            window: window,
            direction: Direction::Right,
            occupied_points: vec![Point { x, y }], // 已经占用的点
        }
    }

    // 获取当前🐍的长度
    pub fn len(&self) -> i32 {
        self.len
    }

    // 获取已经占用的点
    pub fn get_occupied_points(&self) -> &Vec<Point> {
        &self.occupied_points
    }

    // 改变移动方向
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    // 移动主要就是新增加一个node 当作head，新增加的head指向当前最新的head，删除tail
    pub fn move_direction(&mut self, size: i32, is_direction: bool) -> Result<(), String> {
        let data = self.occupied_points.first();
        if let Some(head) = data {
            let mut x = head.x;
            let mut y = head.y;
            let last_direction = self.direction.clone();
            match self.direction {
                Direction::Down => y += size,
                Direction::Up => y -= size,
                Direction::Right => x += size,
                Direction::Left => x -= size,
            }

            // 超出边界
            if x <= self.window.x() || x >= self.window.x() + self.window.width() {
                return Err(String::from("Game over"));
            }
            if y <= self.window.y() || y >= self.window.y() + self.window.height() {
                return Err(String::from("Game over"));
            }

            // 被其他地方修改了方向，为了丝滑放弃本次渲染
            if last_direction != self.direction && !is_direction {
                return Ok(());
            }
            // 只有一个节点，直接重置
            if self.occupied_points.len() == 1 {
                self.occupied_points.pop(); // 最后一个丢掉
                self.occupied_points.push(Point { x, y }); // 记录新的点
                return Ok(());
            }

            // todo 多个节点的时候
        }

        Ok(())
    }
}
