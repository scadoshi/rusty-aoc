#[derive(Debug)]
pub struct GiftBox {
    height: i32,
    width: i32,
    length: i32,
}

impl GiftBox {
    pub fn new(height: i32, width: i32, length: i32) -> Self {
        GiftBox {
            height,
            width,
            length,
        }
    }

    pub fn wrapping_paper_required(&self) -> i32 {
        let front_back = self.height * self.width;
        let left_right = self.height * self.length;
        let top_bottom = self.width * self.length;
        let smallest = front_back.min(left_right).min(top_bottom);
        front_back * 2 + left_right * 2 + top_bottom * 2 + smallest
    }

    pub fn volume(&self) -> i32 {
        self.height * self.length * self.width
    }

    pub fn ribbon_required(&self) -> i32 {
        let mut sides = vec![self.height, self.width, self.length];
        sides.sort();
        sides[0] * 2 + sides[1] * 2 + self.volume()
    }
}

impl From<&str> for GiftBox {
    fn from(value: &str) -> Self {
        // e.g. 10x20x30
        let dimensions: Vec<i32> = value
            .split("x")
            .map(|str| str.trim().parse::<i32>().expect("failed to parse i32"))
            .collect();
        GiftBox::new(dimensions[0], dimensions[1], dimensions[2])
    }
}
