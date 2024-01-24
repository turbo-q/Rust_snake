use std::cmp::max;

use fltk::{prelude::*, *};

use crate::consts;

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Debug)]
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
}

impl Snake {
    pub fn new(
        window_x: i32,
        window_y: i32,
        x: i32,
        y: i32,
        window: window::DoubleWindow,
    ) -> Snake {
        Snake {
            len: 1,
            window: window,
            direction: Direction::Right,
            occupied_points: vec![Point { x, y }], // 已经占用的点
            last_tail_point: Point { x: x, y: y },
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
            println!("1");
            can_move.retain(|x| x.0 != Direction::Up)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x();
            let new_y = last_point.y() - consts::BODY_SIZE;
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                println!("1=");
                can_move.retain(|x| x.0 != Direction::Up)
            } else {
                let idx = can_move.iter().position(|x| x.0 == Direction::Up).unwrap();
                can_move[idx] = (Direction::Up, new_x, new_y)
            }
        }
        // 能不能在下边添加
        if last_point.y() + 2 * consts::BODY_SIZE >= self.window.h() {
            println!("2");
            can_move.retain(|x| x.0 != Direction::Down)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x();
            let new_y = last_point.y() + consts::BODY_SIZE;
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                println!("2=");
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
            println!("4");
            can_move.retain(|x| x.0 != Direction::Right)
        } else {
            // 可以添加判断添加后是否与现有节点交叉
            let new_x = last_point.x() + consts::BODY_SIZE;
            let new_y = last_point.y();
            let is_mix = self.is_mix_snake(&Point { x: new_x, y: new_y });
            if is_mix {
                println!("4=");
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

        self.occupied_points
            .push(Point::new(move_direction.1, move_direction.2))
    }

    // 移动主要就是新增加一个node 当作head，新增加的head指向当前最新的head，删除tail
    pub fn move_direction(&mut self, size: i32, is_direction: bool) -> Result<(), String> {
        println!("points===={:?}", self.occupied_points);
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
            if x <= 0 || x >= self.window.width() - consts::BODY_SIZE {
                return Err(String::from("Game over"));
            }
            if y <= 0 || y >= self.window.height() - consts::BODY_SIZE {
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
