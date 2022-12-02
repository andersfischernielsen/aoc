pub fn calculate(contents: &String) -> u64 {
    let sums = sum_calories(contents);
    let max = sums.iter().take(3).sum::<u64>();
    return max;
}

fn sum_calories(input: &String) -> Vec<u64> {
    let chunks = input.split("\n\n");
    let sums = chunks.map(|chunk| {
        chunk
            .split("\n")
            .map(|calories| calories.parse::<u64>().unwrap_or(0))
            .reduce(|sum, calories| sum + calories)
            .unwrap()
    });
    let mut collected: Vec<u64> = sums.collect();
    collected.sort_by(|f, s| s.cmp(f));
    return collected.to_vec();
}
