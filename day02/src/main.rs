struct Rules<'a> {
    letter: &'a str,
    min: usize,
    max: usize,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let num_valid = input
        .split("\n")
        .filter(|line| {
            let parts: Vec<&str> = line.splitn(2, ":").collect();
            let (rules_str, pass) = (parts[0], parts[1]);

            // split on :, then on space, then -
            let rules = parse_rules(rules_str);
            check_pass(rules, pass)
        })
        .count();

    println!("Part 1: {}", num_valid);
}

fn parse_rules(input: &str) -> Rules {
    let parts: Vec<&str> = input.splitn(2, " ").collect();
    let (range, letter) = (parts[0], parts[1]);

    let range_parts: Vec<usize> = range.split("-").map(|p| p.parse().unwrap()).collect();
    let (min, max) = (range_parts[0], range_parts[1]);

    Rules { letter, min, max }
}

fn check_pass(rules: Rules, input: &str) -> bool {
    let c = input.to_owned().matches(rules.letter).count();
    c >= rules.min && c <= rules.max
}
