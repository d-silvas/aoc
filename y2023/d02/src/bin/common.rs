use regex::Regex;

#[derive(Debug)]
pub struct Draw {
    pub game_id: i32,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub fn draws_from_string(draws_str: &str, game_id: i32) -> Vec<Draw> {
    let index_regex: Regex = Regex::new(r#"Game \d+:"#).unwrap();
    let blue_regex = Regex::new(r#"(?<blue>\d+) blue"#).unwrap();
    let red_regex = Regex::new(r#"(?<red>\d+) red"#).unwrap();
    let green_regex = Regex::new(r#"(?<green>\d+) green"#).unwrap();
    // We could have used the row index that comes from the file,
    // but we are using the loop index instead
    let str = index_regex.replace_all(draws_str, "");
    let draws_str = str.split(';');
    let mut draws_vec: Vec<Draw> = Vec::new();

    for g in draws_str {
        let blue = match blue_regex.captures(g) {
            Some(caps) => caps["blue"].parse::<i32>().unwrap(),
            None => 0_i32,
        };
        let red = match red_regex.captures(g) {
            Some(caps) => caps["red"].parse::<i32>().unwrap(),
            None => 0_i32,
        };
        let green = match green_regex.captures(g) {
            Some(caps) => caps["green"].parse::<i32>().unwrap(),
            None => 0_i32,
        };
        let draw = Draw {
            game_id,
            blue,
            red,
            green,
        };
        draws_vec.push(draw);
    }
    draws_vec
}
