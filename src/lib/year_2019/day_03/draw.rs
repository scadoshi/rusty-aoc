use crate::year_2019::day_03::{instruction::Wire, point::Point};

pub trait Draw {
    fn draw(&self);
}

const BORDER: i32 = 1;

impl Draw for Wire {
    fn draw(&self) {
        let min_x = self.iter().map(|point| point.x).min().unwrap();
        let min_y = self.iter().map(|point| point.y).min().unwrap();
        let max_x = self.iter().map(|point| point.x).max().unwrap();
        let max_y = self.iter().map(|point| point.y).max().unwrap();

        let display: Vec<String> = (min_y - BORDER..=max_y + BORDER)
            .map(|y| {
                (min_x - BORDER..=max_x + BORDER)
                    .map(|x| {
                        let current = Point { x, y };
                        if self.contains(&current) {
                            return "x";
                        } else if current == Point::new() {
                            return "o";
                        } else {
                            return " ";
                        }
                    })
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect();
        for line in display.iter().rev() {
            println!("{}", line);
        }
    }
}

impl Draw for (Wire, Wire) {
    fn draw(&self) {
        let (wire1, wire2) = self;
        let min_x = wire1
            .iter()
            .map(|point| point.x)
            .min()
            .unwrap()
            .min(wire2.iter().map(|point| point.x).min().unwrap());
        let min_y = wire1
            .iter()
            .map(|point| point.y)
            .min()
            .unwrap()
            .min(wire2.iter().map(|point| point.y).min().unwrap());
        let max_x = wire1
            .iter()
            .map(|point| point.x)
            .max()
            .unwrap()
            .max(wire2.iter().map(|point| point.x).max().unwrap());
        let max_y = wire1
            .iter()
            .map(|point| point.y)
            .max()
            .unwrap()
            .max(wire2.iter().map(|point| point.y).max().unwrap());

        let display: Vec<String> = (min_y - BORDER..=max_y + BORDER)
            .map(|y| {
                (min_x - BORDER..=max_x + BORDER)
                    .map(|x| {
                        let current = Point { x, y };
                        if wire1.contains(&current) && wire2.contains(&current) {
                            return "#";
                        } else if wire1.contains(&current) || wire2.contains(&current) {
                            return "x";
                        } else if current == Point::new() {
                            return "o";
                        } else {
                            return " ";
                        }
                    })
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect();
        for line in display.iter().rev() {
            println!("{}", line);
        }
    }
}
