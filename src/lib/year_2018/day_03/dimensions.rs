#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl From<&str> for Dimensions {
    fn from(value: &str) -> Self {
        // e.g. "22x10"
        let (width_str, height_str) = value.trim().split_once('x').unwrap();
        let (width, height) = (
            width_str.parse::<usize>().unwrap(),
            height_str.parse::<usize>().unwrap(),
        );
        Dimensions::new(width, height)
    }
}
