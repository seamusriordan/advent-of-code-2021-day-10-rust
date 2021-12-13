use std::fs;
use day_10_rust::RouteScorer;

fn main() {
    let input = fs::read_to_string("input.txt");

    let score = RouteScorer::new().process(input.unwrap().lines());

    println!("{}", score);
}
