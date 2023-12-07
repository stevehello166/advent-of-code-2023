use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};

fn main() {
    let pt1rep = pt_1("assets/day4/day4test.txt".into());
    println!("rep{}", pt1rep)
}

fn pt_1(filepath: PathBuf) -> u32{
    let file = File::open(filepath).expect("failed to read file");
    let reader = BufReader::new(file);

    let mut scratchcard: Vec<String> = Vec::new();
    let mut cardpointvec: Vec<u32> = Vec::new();
    for lineerr in reader.lines() {
        let line = lineerr.expect("e");
        let mut game: Vec<&str> = line.split(':').collect();
        let numbers: Vec<&str> = game[1].split('|').collect();
        
        let mut card_numbers: Vec<&str> = numbers[0].split(' ').collect();
        let mut winning_numbers: Vec<&str> = numbers[1].split(' ').collect();
        //println!("{:?}", card_numbers);
        //removes empty string from vec 
        winning_numbers.retain(|value| *value != "");
        card_numbers.retain(|value| *value != "");

        //println!("{:?}", card_numbers);
        
        let mut valid_card_numbers: Vec<u32> = Vec::new();
        for ns in &card_numbers {
            
            //println!("{:?}", card_numbers);
            let n: u32 = ns.parse().expect("failed to parse {n}");
            for win_str in winning_numbers.iter(){
                let win:u32 = win_str.parse().expect("failed to parse");
                if win == n {
                    valid_card_numbers.push(n);
                    
                }
            }
            
        }
        println!("{:?}", valid_card_numbers);
            println!("{}", valid_card_numbers.len());
            if valid_card_numbers.len() != 0 {
                if valid_card_numbers.len() == 1 {
                    cardpointvec.push(1);
                    println!("pushing 1")
                } else {
                    let mut val= 0;
                    let mut valu:u32=1;
                    let y = (valid_card_numbers.len() -1).pow(2)-1;
                    while val < valid_card_numbers.len() {
                        let x = valu.pow(2);
                        valu = x;

                        val +=1;
                    }

                    cardpointvec.push(y.try_into().expect("g"));
                    println!("pushing {}", valu);
                    println!("{y}")
                    //println!("{:?}", cardpointvec)
                }
                
            }
        println!("{:?}", cardpointvec);
    }
    let val = cardpointvec.iter().sum();
    return val;
}

mod tests{
    use super::*;
    #[test]
    fn test_pt_1() {
        assert_eq!(pt_1("assets/day4/day4test.txt".into()), 13)
    }
}