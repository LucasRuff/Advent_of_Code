use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut top_three :[i32; 3] = [0,0,0]; 
    let mut sub_total = 0;
    
    for line in reader.lines() {
        let unwrapped_line = &line.unwrap()[..];
        let parsed_line = unwrapped_line.parse::<i32>();
        match parsed_line {
            Ok(line_value) => {sub_total = sub_total + line_value;},
            Err(_) => {
                for value in top_three.into_iter().enumerate() {
                    let (i, x) :(usize, i32) = value;
                    if sub_total > x {
                        top_three[i] = sub_total;
                        break;
                    }
                }
                top_three.sort();
                sub_total = 0;
            }
        }
    }
    let max_calories = top_three[0] + top_three[1] + top_three[2];
    println!("{}\n{}\n{}\n{}", top_three[0],top_three[1],top_three[2],max_calories);

    Ok(())
}
