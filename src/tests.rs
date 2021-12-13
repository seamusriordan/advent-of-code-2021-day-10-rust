#[cfg(test)]

mod tests {
    use crate::{Corrupt, RouteMachine, RouteScorer};
    use crate::Problem::Incomplete;

    #[test]
    fn line_one_example() {
        let input = "[({(<(())[]>[[{[]{<()<>>";

        let result = RouteMachine::new().process_string(input);

        assert_eq!(Err(Incomplete(String::from("}}]])})]"))), result);
    }

    #[test]
    fn line_three_example() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>";

        let result = RouteMachine::new().process_string(input);

        assert_eq!(Err(Corrupt('{')), result);
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

        assert_eq!(288957, result);
    }
}