#![deny(clippy::all)]

use std::io::{stdin, stdout, Write};
use clearscreen;

fn  checking () {
    loop {
        //clearscreen::clear().expect("failed to clear screen");
        let mut name = String::new();
        print!("Please Guese my fav Word: ");
        let _=stdout().flush();
        stdin().read_line(&mut name).expect("Did not enter a correct string");

        if name.trim() == "tender" {
            clearscreen::clear().expect("failed to clear screen");
            println!("GoodJob");
            break;
        } else {
            clearscreen::clear().expect("failed to clear screen");
            println!("Please Try again");
            continue;
        }                    
    }        
}  

fn main() {
    checking();

}
