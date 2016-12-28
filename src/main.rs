extern crate csv;
use csv::{Reader, Writer};

extern crate imdb;
use imdb::models::{Rating, RatingCompare, Row};

use std::io;
use std::env;
use std::path::{Path, PathBuf};



fn read_ratings(path: &Path) -> Vec<Rating> {
    let mut reader = Reader::from_file(path).unwrap().has_headers(true);
    let mut records: Vec<Rating> = Vec::new();
    for row in reader.decode() {
        let row: Row = row.unwrap();
        let rating = Rating::new(row.title, row.rating, row.year, row.url);
        records.push(rating);
    }
    records.sort_by(|a,b| (&a.title).cmp(&b.title));
    records

}

fn compare_ratings(person1_ratings: &[Rating], person2_ratings: &[Rating]) {
    let mut shared_ratings: Vec<RatingCompare> = Vec::new();
    for rating1 in person1_ratings{
        for rating2 in person2_ratings {
            if rating1.title == rating2.title {
                shared_ratings.push(
                RatingCompare { rating1: rating1, rating2: rating2}
                );
                break;
            }
        }
    }
    println!("You have both rated the following titles: ");
    let mut counter: usize = 1;
    for rating in &shared_ratings {
        println!("{}. {}", counter, rating.rating1.title);
        counter+=1;
    }

    let mut active = true;

    while active {
        println!("Please enter the number of the rating you would like to see (or press 0 to exit): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("You suck at this");

        // Thanks stack overflow
        let trimmed = input.trim();
        let choice = match trimmed.parse::<usize>() {
            Ok(i) =>  i,
            Err(..) => {
                println!("this was not an integer: {}", trimmed);
                0
            }
        };
        if choice > shared_ratings.len() || choice < 0 {
            println!("Invalid entry, please try again");
            continue;
        }
        if choice == 0 {
            active = false;
            continue;
        }
        let target_rating = shared_ratings.get(choice - 1).unwrap();
        println!("You chose: {}. {}", choice, target_rating.rating1.title);
        println!("You rated this film: \t\t{}", target_rating.rating1.rating);
        println!("Your friend rated this film: \t{}", target_rating.rating2.rating);
    }
}

fn help() { println!("You've goofed")}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {           
            let person1_rating_file = PathBuf::from(&args[1]);
            let person2_rating_file = PathBuf::from(&args[2]);
            let person1_ratings = read_ratings(person1_rating_file.as_path());
            let person2_ratings = read_ratings(person2_rating_file.as_path());
            compare_ratings(&person1_ratings, &person2_ratings);

        },
        _ => help(),


    }
}
