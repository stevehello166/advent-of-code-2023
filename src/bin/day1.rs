use std::{fs::File, io::{BufReader, BufRead, self}, error::Error};

fn main () -> io::Result<()> {
    let file = File::open("assets/day1.txt")?;
    let reader = BufReader::new(file);

    let mut stringvec: Vec<String> = Vec::new();
    for line in reader.lines() {
        let mut r_string = String::new();
        for characters in line.expect("error").chars() {
            if characters.is_ascii_digit() {
                r_string.push(characters);
            }
        }
        stringvec.push(r_string)
        
    }

    let mut responsevec: Vec<u32> = Vec::new();
    let mut reps = 0;
    while reps < stringvec.len() {
        let mut rstring:String = String::new();
        let char_vec: Vec<char> = stringvec[reps].chars().collect();
        let ch1 = char_vec[0];
        let ch2 = char_vec[char_vec.len() - 1];
        rstring.push(ch1);
        rstring.push(ch2);

        let rstring = rstring.parse().expect("error");
        responsevec.push(rstring);
        
        reps += 1;
    }
    println!("{:?}", responsevec);
    let finalrep: u32 = responsevec.iter().sum();
    println!("{}", finalrep);
    Ok(())
}