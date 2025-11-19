#[derive(Debug, Clone)]
pub struct MarbleGroup {
    pub color: String,
    pub count: u8,
}

impl From<&str> for MarbleGroup {
    fn from(value: &str) -> Self {
        // e.g. 10 green
        let (count_str, color_str) = value.split_once(" ").unwrap();
        let (count, color) = (count_str.parse::<u8>().unwrap(), color_str.to_string());
        Self { color, count }
    }
}

#[derive(Debug)]
pub struct Handful(Vec<MarbleGroup>);

impl Handful {
    pub fn marble_groups(&self) -> Vec<MarbleGroup> {
        self.0.clone()
    }
}

impl From<&str> for Handful {
    fn from(value: &str) -> Self {
        // e.g.
        // 10 green, 5 blue
        Handful(
            value
                .split(",")
                .map(|x| MarbleGroup::from(x.trim()))
                .collect::<Vec<MarbleGroup>>(),
        )
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u8,
    pub handfuls: Vec<Handful>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        // e.g.
        // Game 1: 10 green, 5 blue; 1 red, 9 green, 10 blue; 5 blue, 6 green, 2 red; 7 green, 9 blue, 1 red; 2 red, 10 blue, 10 green; 7 blue, 1 red
        let (game_str, handfuls_str) = value.split_once(":").unwrap();
        let (_, id_str) = game_str.split_once(" ").unwrap();
        let id: u8 = id_str.parse().unwrap();

        let handfuls: Vec<Handful> = handfuls_str
            .split(";")
            .map(|x| Handful::from(x.trim()))
            .collect();

        Self { id, handfuls }
    }
}
