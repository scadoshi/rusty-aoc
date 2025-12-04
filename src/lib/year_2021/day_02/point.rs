use crate::year_2021::day_02::movement::{Direction as D, Movement};

#[derive(Default)]
pub struct Point {
    x: i32,
    y: i32,
}


impl Point {
    pub fn up(&mut self, distance: i32) {
        self.y += distance;
    }

    pub fn down(&mut self, distance: i32) {
        self.y -= distance;
    }

    pub fn forward(&mut self, distance: i32) {
        self.x += distance;
    }

    pub fn traverse(&mut self, movement: &Movement) {
        match movement.direction {
            D::Forward => self.forward(movement.distance),
            D::Up => self.up(movement.distance),
            D::Down => self.down(movement.distance),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn product(&self) -> i32 {
        self.x * self.y
    }
}

#[derive(Default)]
pub struct AimedPoint {
    pub point: Point,
    pub aim: i32,
}


impl AimedPoint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aim_up(&mut self, distance: i32) {
        self.aim += distance;
    }

    pub fn aim_down(&mut self, distance: i32) {
        self.aim -= distance;
    }

    pub fn adjust(&mut self, movement: &Movement) {
        match movement.direction {
            D::Forward => {
                self.point.forward(movement.distance);
                // i know this is funny to look at
                // if aim is positive we should move down basically
                self.point.down(self.aim * movement.distance);
            }
            D::Up => self.aim_up(movement.distance),
            D::Down => self.aim_down(movement.distance),
        }
    }
}
