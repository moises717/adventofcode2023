use std::fs::File;
use std::io::{BufRead, BufReader};

enum GameConfig {
    RED = 12,
    GREEN = 13,
    BLUE = 14,
}

fn is_game_possible(game: &String) -> bool {
let splited_game: Vec<&str> = game.split(";").collect();
let mut is_possible = true;

    for game in splited_game {
        let game: Vec<&str> = game.split(",").collect();
        
        for game  in game {
              let game_number: Vec<&str> = game.split_whitespace().collect();
              let game_number: i32 = game_number[0].parse().unwrap();

             if game.contains("blue") && game_number > GameConfig::BLUE as i32 {
                is_possible = false;
                break;
             } else if game.contains("red") && game_number > GameConfig::RED as i32 {
                is_possible = false;
                break;
             } else if game.contains("green") && game_number > GameConfig::GREEN as i32 {
                is_possible = false;
                break;
             }
        }
    }

    is_possible
}

fn fewest_number_cubes(game: &String) -> usize {
    let splited_game: Vec<&str> = game.split(";").collect();
    let mut min_blue: usize = 0;
    let mut min_red: usize = 0;
    let mut min_green: usize = 0;


    for game in splited_game {
        let game: Vec<&str> = game.split(",").collect();

        for game in game {
            let game_number: Vec<&str> = game.split_whitespace().collect();
            let game_number: usize = game_number[0].parse().unwrap();

            if game.contains("blue") && game_number > min_blue  {
                min_blue = game_number;
            } else if game.contains("red") && game_number > min_red {
                min_red = game_number;
            } else if game.contains("green") && game_number > min_green {
                min_green = game_number;
            }
        }

    }
    min_blue * min_red * min_green
} 

fn main() -> std::io::Result<()> {
    let file = File::open("input02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: i64 = 0;
    let mut power_sum = 0;

    for line in reader.lines() {
            let line_game = &line.unwrap();
            let game_ids: Vec<&str> = line_game.split(":").collect();
            let game_ids_parts: Vec<&str> = game_ids[0].split_whitespace().collect();
            let game_id: i64 = game_ids_parts[1].parse().unwrap();
            let semi_colon_pos = line_game.find(":").unwrap();
           
           let game: String = line_game[semi_colon_pos+1..].to_string().trim().to_string();
            
            let is_possible = is_game_possible(&game);
            let mul_minimus = fewest_number_cubes(&game);
            power_sum += mul_minimus as i64;
           
            if is_possible {
                sum += game_id;
            } 

    }

    println!("Total of possible games: {}", sum);
    println!("Total of power: {}", power_sum);

    Ok(())
}
