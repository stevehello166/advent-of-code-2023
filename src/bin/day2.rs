use std::{fs::File, io::{self, BufRead, BufReader, Lines}, path::Path};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn main () -> io::Result<()> {
    let file = File::open("assets/day2.txt")?;
    let reader = BufReader::new(file);

    let mut valid_games: Vec<u32> = Vec::new();
    let mut game = 1;
    for line in reader.lines() {
        let line = line.expect("msg");
        let splitvec:Vec<&str> = line.split(';').collect();

        let mut redvec: Vec<String> = Vec:: new();
        let mut greenvec: Vec<String> = Vec:: new();
        let mut bluevec: Vec<String> = Vec:: new();
        for string in splitvec {
            let quartervec: Vec<&str> = string.split(',').collect();
            for cstring in quartervec {
                if cstring.contains("red") {
                    redvec.push(cstring.to_string());
                } else if cstring.contains("green") {
                    greenvec.push(cstring.to_string());
                } else if cstring.contains("blue") {
                    bluevec.push(cstring.to_string());
                }
            }
        }
        let mut red_collect = get_numbers(redvec);
        let mut green_collect = get_numbers(greenvec);
        let mut blue_collect = get_numbers(bluevec);

        red_collect.sort();
        red_collect.reverse();
        green_collect.sort();
        green_collect.reverse();
        blue_collect.sort();
        blue_collect.reverse();

        if RED > red_collect[0]  {
            if GREEN > green_collect[0]{
                if BLUE > blue_collect[0]{
                    valid_games.push(game);
                    
                }
            }
        }
        
        //println!("{:?}", green_collect[0]);
        game += 1;
    }
    
    println!("{:?}", valid_games.iter().sum::<u32>());
    
    Ok(())
}

fn get_numbers(
    colourvec: Vec<String>
) -> Vec<u32>{
    let mut  colour_collect: Vec<u32> = Vec::new();
    for string in colourvec.clone() {
        let mut collection_string = String::new();
        for char in string.chars(){
            if char.is_ascii_digit() {
                collection_string.push(char);
            }
        }
        let collection_num:u32 = collection_string.parse().expect("msg");
        colour_collect.push(collection_num);
        
    }
    return colour_collect;
}