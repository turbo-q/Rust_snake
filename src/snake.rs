use std::{cmp::max, collections::HashMap};

use fltk::{prelude::*, *};

use crate::{
    consts::{self, BODY_SIZE},
    utils,
};

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Point {
    x: i32, // x方向位置
    y: i32, // y方向位置
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
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
    last_tail_point: Point,      // 上一次尾节点，可以用来新增节点
    is_change: bool,
}

impl Snake {
    pub fn new(x: i32, y: i32, window: window::DoubleWindow) -> Snake {
        // 初始direction设置，哪边距离长就哪边
        let (left, right, up, down) =
            (x, window.w() - x - BODY_SIZE, y, window.h() - y - BODY_SIZE);
        let max_ = max(max(left, right), max(up, down));
        let default_direction = match max_ {
            _ if max_ == left => Direction::Left,
            _ if max_ == right => Direction::Right,
            _ if max_ == up => Direction::Up,
            _ if max_ == down => Direction::Down,
            _ => Direction::Right,
        };

        Snake {
            len: 1,
            window: window,
            direction: default_direction,
            occupied_points: vec![Point { x, y }], // 已经占用的点
            last_tail_point: Point { x: x, y: y },
            is_change: false,
        }
    }
    pub fn clear(&mut self) {
        self.len = 1;
        // init snake/根据consts::BODY_SIZE 分为相应的份数
        let max_x = (self.window.w() - consts::BODY_SIZE) / consts::BODY_SIZE;
        let max_y = (self.window.h() - consts::BODY_SIZE) / consts::BODY_SIZE;
        let rand_x: i32 = utils::rand_range(0, max_x) * consts::BODY_SIZE;
        let rand_y = utils::rand_range(0, max_y) * consts::BODY_SIZE;

        self.occupied_points = vec![Point {
            x: rand_x,
            y: rand_y,
        }];
        self.last_tail_point = Point {
            x: rand_x,
            y: rand_y,
        };

        let (left, right, up, down) = (
            rand_x,
            self.window.w() - rand_x,
            rand_y,
            self.window.h() - rand_y,
        );
        let max_ = max(max(left, right), max(up, down));
        self.direction = match max_ {
            _ if max_ == left => Direction::Left,
            _ if max_ == right => Direction::Right,
            _ if max_ == up => Direction::Up,
            _ if max_ == down => Direction::Down,
            _ => Direction::Right,
        };
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
    pub fn set_direction(&mut self, direction: Direction) -> Result<(), String> {
        let reverse_direction: HashMap<Direction, Direction> = {
            let mut m = HashMap::new();
            m.insert(Direction::Up, Direction::Down);
            m.insert(Direction::Down, Direction::Up);
            m.insert(Direction::Left, Direction::Right);
            m.insert(Direction::Right, Direction::Left);
            m
        };
        // 如果大于两个节点肯定不能向相反方向移动
        if reverse_direction.get(&self.direction).unwrap().to_owned() == direction && self.len() > 1
        {
            return Err(String::from("不能移动相反方向"));
        }
        if direction != self.direction {
            self.is_change = true;
        }
        self.direction = direction;
        Ok(())
    }

    // 添加body逻辑
    // 在蛇尾添加，默认与移动方向相反
    // 比如当前蛇头向左移动，则添加蛇尾的右边
    // 当前蛇头向上移动，则添加到蛇尾的下边
    // 当前也要进行边界值的判定
    pub fn add_body(&mut self) {
        let last_point = self.get_occupied_points().last().unwrap();
        let mut can_move = vec![
            (Direction::Up, 0, 0),
            (Direction::Down, 0, 0),
            (Direction::Left, 0, 0),
            (Direction::Right, 0, 0),
        ];

        // 能不能在上边添加
        if last_point.y() - consts::BODY_SIZE <= 0 {
            can_move.retain(|x| x.0 != Direction::Up)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x();
            let new_y = last_point.y() - consts::BODY_SIZE;
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                can_move.retain(|x| x.0 != Direction::Up)
            } else {
                let idx = can_move.iter().position(|x| x.0 == Direction::Up).unwrap();
                can_move[idx] = (Direction::Up, new_x, new_y)
            }
        }
        // 能不能在下边添加
        if last_point.y() + 2 * consts::BODY_SIZE >= self.window.h() {
            can_move.retain(|x| x.0 != Direction::Down)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x();
            let new_y = last_point.y() + consts::BODY_SIZE;
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                can_move.retain(|x| x.0 != Direction::Down)
            } else {
                let idx = can_move
                    .iter()
                    .position(|x| x.0 == Direction::Down)
                    .unwrap();
                can_move[idx] = (Direction::Down, new_x, new_y)
            }
        }
        // 能不能在左边添加
        if last_point.x() - consts::BODY_SIZE <= 0 {
            println!("3");
            can_move.retain(|x| x.0 != Direction::Left)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x() - consts::BODY_SIZE;
            let new_y = last_point.y();
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                println!("3=");
                can_move.retain(|x| x.0 != Direction::Left)
            } else {
                let idx = can_move
                    .iter()
                    .position(|x| x.0 == Direction::Left)
                    .unwrap();
                can_move[idx] = (Direction::Left, new_x, new_y)
            }
        }
        // 能不能在右边添加
        if last_point.x() + 2 * consts::BODY_SIZE >= self.window.w() {
            can_move.retain(|x| x.0 != Direction::Right)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x() + consts::BODY_SIZE;
            let new_y = last_point.y();
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                can_move.retain(|x| x.0 != Direction::Right)
            } else {
                let idx = can_move
                    .iter()
                    .position(|x| x.0 == Direction::Right)
                    .unwrap();
                can_move[idx] = (Direction::Right, new_x, new_y)
            }
        }
        // 没有可以移动
        if can_move.len() == 0 {
            panic!("Game over")
        }

        // 根据移动方向调整优先级
        // 把高优先级的方向优先添加
        let priority_direction: Direction;
        match self.direction {
            Direction::Up => priority_direction = Direction::Down,
            Direction::Down => priority_direction = Direction::Up,
            Direction::Left => priority_direction = Direction::Right,
            Direction::Right => priority_direction = Direction::Left,
        }
        if let Some(idx) = can_move.iter().position(|x| x.0 == priority_direction) {
            can_move.swap(0, idx)
        }
        let move_direction = can_move.get(0).unwrap();

        self.len += 1;
        self.occupied_points
            .push(Point::new(move_direction.1, move_direction.2))
    }

    // 移动主要就是新增加一个node 当作head，新增加的head指向当前最新的head，删除tail
    pub fn move_direction(&mut self, size: i32, is_direction: bool) -> Result<(), String> {
        // 方向改变放弃这次渲染，渲染方向的改变（方向的改变已经调用过一次move_direction了）
        if self.is_change && !is_direction {
            self.is_change = false;
            return Ok(());
        }
        let first = self.occupied_points.first();
        if let Some(head) = first {
            let mut x = head.x;
            let mut y = head.y;
            let last_direction = self.direction.clone();
            match self.direction {
                Direction::Down => y += size * consts::BODY_SIZE,
                Direction::Up => y -= size * consts::BODY_SIZE,
                Direction::Right => x += size * consts::BODY_SIZE,
                Direction::Left => x -= size * consts::BODY_SIZE,
            }

            // 超出边界
            if x < 0 || x > self.window.width() - consts::BODY_SIZE {
                return Err(String::from("Game over"));
            }
            if y < 0 || y > self.window.height() - consts::BODY_SIZE {
                return Err(String::from("Game over"));
            }

            // 被其他地方修改了方向，为了丝滑放弃本次渲染
            if last_direction != self.direction && !is_direction {
                return Ok(());
            }

            self.last_tail_point = self.occupied_points.pop().unwrap(); // 最后一个丢掉
            self.occupied_points.insert(0, Point { x, y }); // 记录新的点
            return Ok(());
        }

        Ok(())
    }

    fn is_mix_point(&self, point1: &Point, point2: &Point) -> bool {
        // 间隔小于等于2倍body就是穿过了
        let x_space = max(
            point1.x() + consts::BODY_SIZE - point2.x(),
            point2.x() + consts::BODY_SIZE - point1.x(),
        );
        let y_space = max(
            point1.y() + consts::BODY_SIZE - point2.y(),
            point2.y() + consts::BODY_SIZE - point1.y(),
        );
        (point1.x() == point2.x() || point1.y() == point2.y()/*在同一条线*/)
            && (x_space < 2 * consts::BODY_SIZE && y_space < 2 * consts::BODY_SIZE/*有交叉*/)
    }

    fn is_mix_snake(&self, point_: &Point) -> bool {
        self.get_occupied_points().iter().any(move |p| {
            if self.is_mix_point(point_, p) {
                return true;
            }
            false
        })
    }
}
