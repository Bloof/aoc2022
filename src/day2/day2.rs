use crate::http_get::get_aoc_day_data;
use std::error::Error;

pub fn day_two(day: i32) {
    let data = get_aoc_day_data(&day.to_string());

    let test = &parse_data(data.to_owned()).unwrap();
    println!("Day 2 part 2: {:?}", day_two_part_one(test.to_owned()));
}

#[derive(PartialEq, Eq)]
enum Shapes {
    Rock,
    Paper,
    Scissors,
}

struct ShapeRules {
    kind: Shapes,
    wins_to: Shapes,
    loses_to: Shapes,
    value: i32,
}

fn day_two_part_one(data: Vec<String>) -> i32 {
    let mut points = 0;

    for line in data.iter() {
        let mut played_shapes = line.splitn(2, " ");
        let opponent = get_opponent_shape(played_shapes.next().unwrap());
        let player = get_player_shape(played_shapes.next().unwrap());

        points += check_points(opponent, get_rules(player));

    }

    return points;
}

fn check_points(opponent: Shapes, player: ShapeRules) -> i32 {

    // Draw
    if player.kind == opponent{
        return player.value + 3;
    } 
    // Player wins
    else if player.wins_to == opponent {
        return player.value + 6;
    } 
    // Player loses
    else if player.loses_to == opponent {
        return player.value;
    } 
    panic!("This should not happen");
    
}

fn get_opponent_shape(shape: &str) -> Shapes {
    return match shape {
        "A" => Shapes::Rock,
        "B" => Shapes::Paper,
        "C" => Shapes::Scissors,
        _ => panic!("Value not implemented"),
    };
}

fn get_player_shape(shape: &str) -> Shapes {
    return match shape {
        "X" => Shapes::Rock,
        "Y" => Shapes::Paper,
        "Z" => Shapes::Scissors,
        _ => panic!("Value not implemented"),
    };
}

fn get_rules(shape: Shapes) -> ShapeRules {

    if shape == Shapes::Rock {
        return ShapeRules {
            kind: shape,
            wins_to: Shapes::Scissors,
            loses_to: Shapes::Paper,
            value: 1,
        };
    } else if shape == Shapes::Paper {
        return ShapeRules {
            kind: shape,
            wins_to: Shapes::Rock,
            loses_to: Shapes::Scissors,
            value: 2,
        };
    } else {
        //Scissors
        return ShapeRules {
            kind: shape,
            wins_to: Shapes::Paper,
            loses_to: Shapes::Rock,
            value: 3,
        };
    }
}

fn parse_data(data: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut parsed: Vec<String> = Vec::new();

    for line in data.lines() {
        let line = line.trim_end();
        parsed.push(line.to_string());
    }
    Ok(parsed)
}
