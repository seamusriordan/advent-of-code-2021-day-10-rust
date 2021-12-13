use std::str::Lines;
use crate::Problem::Corrupt;

mod tests;

#[derive(Debug, Eq, PartialEq)]
pub enum Problem {
    Corrupt(char),
    Incomplete(&'static str)
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

    pub fn process_string(mut self, s: &str) -> Result<bool, Problem> {
        for c in s.chars() {
            match self.process_char(&c) {
                Err(e) => return Err(e),
                _ => {}
            }
        }

        Ok(true)
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
    score: i32,
}

impl RouteScorer {
    pub fn new() -> RouteScorer {
        return RouteScorer {
            score: 0
        };
    }

    pub fn process(&mut self, lines: Lines) -> i32 {
        for line in lines {
            let route_machine = RouteMachine::new();
            match route_machine.process_string(line) {
                Err(Corrupt('(')) => self.score += 3,
                Err(Corrupt('[')) => self.score += 57,
                Err(Corrupt('{')) => self.score += 1197,
                Err(Corrupt('<')) => self.score += 25137,
                _ => {}
            }
        }

        self.score
    }
}


