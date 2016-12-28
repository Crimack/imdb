
//use rustc_serialize::{Decodable, Encodable};
use std::cmp::Ordering;


#[derive(Debug, PartialOrd, Eq, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Rating {
    pub title: String,
    pub rating: usize,
    pub year: usize,
    pub url: String
}

impl Rating {
    pub fn new(title: String, rating: usize, year: usize, url: String) -> Rating {
        Rating {
            title: title,
            rating: rating,
            year: year,
            url: url
        }
    }
}

impl Ord for Rating {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by best rating to worst
        (&self.rating).cmp(&(&other.rating)).reverse()
    }
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct RatingCompare<'a> {
    pub rating1: &'a Rating,
    pub rating2: &'a Rating
}

impl<'a> Ord for RatingCompare<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.rating1.title).cmp((&other.rating2.title))
    }
}


#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Row {
    pub row_num: usize,
    pub constant: String,
    pub date: String,
    pub modified: String,
    pub description: String,
    pub title: String,
    pub film_type: String,
    pub director: String,
    pub rating: usize,
    pub average_rating: Option<f64>,
    pub runtime: Option<usize>,
    pub year: usize,
    pub genres: String,
    pub num_votes: u32,
    pub release_date: String,
    pub url: String
}