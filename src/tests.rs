#[cfg(test)]

mod tests {
    use crate::{RouteMachine, RouteScorer};

    #[test]
    fn line_one_example() {
        let input = "[({(<(())[]>[[{[]{<()<>>";

        let result = RouteMachine::new().process_string(input);

        assert_eq!(Ok(true), result);
    }

    #[test]
    fn line_three_example() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>";

        let result = RouteMachine::new().process_string(input);

        assert_eq!(Err('{'), result);
    }

    #[test]
    fn example_score() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".lines();

        let result = RouteScorer::new().process(input);

        assert_eq!(26397, result);
    }
}