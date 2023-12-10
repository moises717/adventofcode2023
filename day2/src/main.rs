use std::fs::File;
use std::io::{BufRead, BufReader};

enum GameConfig {
    RED = 12,
    GREEN = 13,
    BLUE = 14,
}

fn is_game_possible(game: String) -> bool {
let splited_game: Vec<&str> = game.split(";").collect();
let mut is_possible = true;

    for game in splited_game {
        println!("{}", game);
        let game: Vec<&str> = game.split(",").collect();
        
        for game  in game {
              let game_number: Vec<&str> = game.split_whitespace().collect();
              let game_number: i32 = game_number[0].parse().unwrap();

             if game.contains("blue") && game_number > GameConfig::BLUE as i32 {
                is_possible = false;
             } else if game.contains("red") && game_number > GameConfig::RED as i32 {
                is_possible = false;
             } else if game.contains("green") && game_number > GameConfig::GREEN as i32 {
                is_possible = false;
             }
        }
    }

    is_possible

 
}

fn main() -> std::io::Result<()> {
    let file = File::open("input02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: i64 = 0;

    for line in reader.lines() {
            let line_game = &line.unwrap();
            let game_id: Vec<&str> = line_game.split(":").collect();
            let game_id: Vec<&str> = game_id[0].split_whitespace().collect();
            let game_id: i64 = game_id[1].parse().unwrap();
            let semi_colon_pos = line_game.find(":").unwrap();
           
           let game = line_game[semi_colon_pos+1..].to_string().trim().to_string();
            
            let is_possible = is_game_possible(game);

            if is_possible {
                println!("{}", game_id);
                sum += game_id;
            } 

    }

    println!("Total of possible games: {}", sum);

    Ok(())
}
