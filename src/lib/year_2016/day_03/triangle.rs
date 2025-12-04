use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidTriangle {
    #[error("must provide three sides")]
    SideCount,
    #[error("each side must be valid integer")]
    SideNotNumber,
}

#[derive(Debug, Clone)]
pub struct Triangle(pub u32, pub u32, pub u32);

impl TryFrom<&[u32]> for Triangle {
    type Error = InvalidTriangle;
    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(InvalidTriangle::SideCount);
        }
        Ok(Triangle(value[0], value[1], value[2]))
    }
}

impl TryFrom<&str> for Triangle {
    type Error = InvalidTriangle;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sides: Vec<u32> = value
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(|_| InvalidTriangle::SideNotNumber)?;
        Triangle::try_from(sides.as_slice())
    }
}

impl Triangle {
    pub fn is_valid(&self) -> bool {
        let mut sides = [self.0, self.1, self.2];
        sides.sort();
        sides[0] + sides[1] > sides[2]
    }
}
