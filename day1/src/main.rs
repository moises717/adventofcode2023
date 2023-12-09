use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};


fn read_file(file_path: &str) -> io::Result<String> {
    // read file
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn calibration_value(file_content: String) -> usize {
    let numbers = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine")
      
    ]);
    
    let lines: Vec<&str> = file_content.split("\n").collect();
    let mut sum_of_numbers = 0;

   for line in lines {
       let mut flirt_number = String::new();
       let mut last_number = String::new();
       let mut new_line = String::from(line);

       for (number,i) in numbers.iter() {
           if line.contains(number) {
               new_line = new_line.replace(number, i);
               
            }
        }
    
    for c in new_line.chars() {
        if c.is_numeric() {
            flirt_number = c.to_string();
            break;
        }
    }
    for c in new_line.chars().rev() {
        if c.is_numeric() {
            last_number = c.to_string();
            break;
        }
    }
    
    let concatenated_numbers = flirt_number + &last_number;
    let number: i32 = concatenated_numbers.parse().unwrap();
    sum_of_numbers += number;
    
   }
    sum_of_numbers as usize

}

fn main() -> io::Result<()> {
    let file_path = "input01.txt";

    match read_file(file_path) {
        Ok(file_content) => {
            let calibration = calibration_value(file_content);
            println!("Calibration: {}", calibration);
            
            Ok(())
        }
        Err(err) => {
            // Maneja el error al leer el archivo
            eprintln!("Error al leer el archivo: {}", err);
            Err(err)
        }
    }
}
