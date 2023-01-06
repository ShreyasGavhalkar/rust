use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();
    
    let mut max = 0;
    let mut sum = 0;
    for l in lines {
        
        if (l.as_ref().unwrap().eq("")){
            if(sum>max){
                max = sum;
            }
            sum=0;
            println!("Blank line");
            continue;
        }
        let x:i32=l.unwrap().parse::<i32>().unwrap();
        sum+=x;
        println!("{}", x);
    }
    if(sum>max){
                max = sum;
            }
    println!("Max is {}", max);
}
