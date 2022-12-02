pub fn calculate(contents: &String) -> u64 {
    let sums = calculate_scores(contents);
    return sums;
}

fn calculate_round(opponent: &str, us: &str) -> u64 {
    let score = |choice| match choice {
        "A" => 1, // A = Rock
        "B" => 2, // B = Paper
        "C" => 3, // C = Scissors
        "X" => 1, // X = Rock
        "Y" => 2, // Y = Paper
        "Z" => 3, // Z = Scissors
        _ => 0,
    };

    let outcome = |opponent, us| match (opponent, us) {
        ("A", "X") => 3, // 3 = Draw
        ("A", "Y") => 6, // 6 = Win
        ("A", "Z") => 0, // 0 = Lost
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => 0,
    };

    let determine_choice = |opponent, outcome| match (opponent, outcome) {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => "",
    };

    let choice = determine_choice(opponent, us);
    return score(choice) + outcome(opponent, choice);
}

fn calculate_scores(input: &String) -> u64 {
    let chunks = input.split("\n");
    let sum: u64 = chunks
        .map(|chunk| {
            let split: Vec<&str> = chunk.split(" ").take(2).collect();
            dbg!(&split);
            if split.len() == 1 {
                return 0;
            };
            let (opponent, us) = (split[0], split[1]);
            return calculate_round(opponent, us);
        })
        .sum();

    return sum;
}
