use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    // read file
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> io::Result<()> {
    let file_path = "input01.txt";

    match read_file(file_path) {
        Ok(file_content) => {
            let lines: Vec<&str> = file_content.split("\n").collect();
             let mut sum_of_numbers = 0;

            for line in lines {
                let mut flirt_number = String::new();
                let mut last_number = String::new();
                
                for c in line.chars() {
                    if c.is_numeric() {
                        flirt_number = c.to_string();
                        break;
                    }
                }
                for c in line.chars().rev() {
                    if c.is_numeric() {
                        last_number = c.to_string();
                        break;
                    }
                }
                
               let concatenated_numbers = flirt_number + &last_number;
                let number: i32 = concatenated_numbers.parse().unwrap();
                sum_of_numbers += number;
                
             
            }

            println!("Suma de los números: {}", sum_of_numbers);
            
            Ok(())
        }
        Err(err) => {
            // Maneja el error al leer el archivo
            eprintln!("Error al leer el archivo: {}", err);
            Err(err)
        }
    }
}
