use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};

fn main() {
    let rp1 = pt_1("assets/day3tst.txt".into());
    println!("{rp1}")
}

fn pt_1(filename: PathBuf) -> u32 {
    let file = File::open(filename).expect("failed to read file");
    let reader = BufReader::new(file);
    let mut repvec: Vec<u32> = Vec::new();

    let mut numvec:Vec<String> = Vec::new();
    let mut vertvec: Vec<Vec<char>> = Vec::new();
    let mut spec_char_vec: Vec<String> = Vec::new();
    let mut vertnum = 0;
    for lineserr in reader.lines() {
        let line = lineserr.expect("msg"); 
        let mut numstring = String::new();
        let mut horvec:Vec<char> = Vec::new();
        
        let mut charnum = 0;
        for character in line.chars() {
            horvec.push(character);
            if character.is_ascii_digit() {
                numstring.push(character);
            } else if !character.is_ascii_digit() && !numstring.is_empty() {
                numstring.push_str((": ".to_string() + charnum.to_string().as_str() + " | " + vertnum.to_string().as_str() + " | " + (numstring.len()-1).to_string().as_str() ).as_str());
                numvec.push(numstring);
                numstring = String::new()
            } else if character != '.' && !character.is_numeric() {
                spec_char_vec.push( charnum.to_string() + "|" + vertnum.to_string().as_str())
            }
            charnum += 1;
        }

        
        vertvec.push(horvec);
        vertnum +=1;
    }

    
    //put in array of two slots
    for numstring in numvec {
        let numarr: Vec<&str> = numstring.split(':').collect();
        


        let mut numpos: Vec<String> = Vec::new();
        for n in numarr[1].split('|') {
             numpos.push(n.to_string());
             println!("{:?}",numpos)
        }
        //numarr[1].split('|').collect::<Vec<_>>();
        for val in &spec_char_vec {
        
            let charpos:Vec<&str> = val.split('|').collect();

            let mut xs = numpos[0].trim().parse::<u32>().expect("msg");
            if xs > 0{
                xs -= 1;
            }
            let xl:u32 = numpos[0].trim().parse::<u32>().expect("msg") +1;
            let mut ys = numpos[1].trim().parse::<u32>().expect("msg");
            if ys > 0{
                ys -= 1;
            }
            let yl:u32 = numpos[1].trim().parse::<u32>().expect("msg") +1;

            if (xs..xl).contains(&charpos[0].parse::<u32>().unwrap()) &&
            (ys..yl).contains(&charpos[1].parse::<u32>().unwrap()) 
            {
                println!("{}", numarr[0]);
                repvec.push(numarr[0].parse::<u32>().expect("msg"))
            }
        }

        
    }
    
    
    return repvec.iter().sum();
} 