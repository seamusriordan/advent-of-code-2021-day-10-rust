use std::str::Lines;
use crate::Problem::{Corrupt, Incomplete};

mod tests;

#[derive(Debug, Eq, PartialEq)]
pub enum Problem {
    Corrupt(char),
    Incomplete(String)
}

pub struct RouteMachine {
    state: Vec<char>,
}

impl RouteMachine {
    pub fn new() -> RouteMachine {
        return RouteMachine {
            state: vec![]
        };
    }

    pub fn process_string(&mut self, s: &str) -> Result<bool, Problem> {
        for c in s.chars() {
            match self.process_char(&c) {
                Err(e) => return Err(e),
                _ => {}
            }
        }

        let mut remaining: String = String::from("");
        for c in self.state.iter().rev() {
            match c {
                '(' => remaining.push(')'),
                '[' => remaining.push(']'),
                '{' => remaining.push('}'),
                '<' => remaining.push('>'),
                _ => {}
            }
        }

        Err(Incomplete(remaining))
    }

    pub fn process_char(&mut self, c: &char) -> Result<bool, Problem> {
        match c {
            '(' | '[' | '{' | '<' => {
                return self.add_opener(c);
            }
            ')' => {
                return self.match_opener('(');
            }
            ']' => {
                return self.match_opener('[');
            }
            '}' => {
                return self.match_opener('{');
            }
            '>' => {
                return self.match_opener('<');
            }

            _ => panic!("Unknown char")
        }
    }

    fn add_opener(&mut self, c: &char) -> Result<bool, Problem> {
        self.state.push(*c);
        return Ok(true);
    }

    fn match_opener(&mut self, c: char) -> Result<bool, Problem> {
        if let Some(last) = self.state.last() {
            if last == &c {
                self.state.pop();
                return Ok(true);
            }
            return Err(Corrupt(c));
        }

        panic!("Unexpected")
    }
}

pub struct RouteScorer {
    corrupt_score: u64,
    incomplete_scores: Vec<u64>
}

impl RouteScorer {
    pub fn new() -> RouteScorer {
        return RouteScorer {
            corrupt_score: 0,
            incomplete_scores: vec![]
        };
    }

    pub fn process(&mut self, lines: Lines) -> u64 {
        for line in lines {
            let mut route_machine = RouteMachine::new();
            match route_machine.process_string(line) {
                Err(Corrupt('(')) => self.corrupt_score += 3,
                Err(Corrupt('[')) => self.corrupt_score += 57,
                Err(Corrupt('{')) => self.corrupt_score += 1197,
                Err(Corrupt('<')) => self.corrupt_score += 25137,
                Err(Incomplete(s)) => {
                    let mut score = 0;
                    for c in s.chars() {
                        score *= 5;
                        score += match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0
                        }
                    }
                    self.incomplete_scores.push(score)
                }
                _ => {}
            }
        }

        self.incomplete_scores.sort();
        self.incomplete_scores[self.incomplete_scores.len()/2]
    }
}


