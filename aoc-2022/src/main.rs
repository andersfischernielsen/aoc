use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = read_file(path);
    let sums = sum_calories(&contents);
    let max = sums.iter().take(3).sum::<u64>();
    println!("{}", max);
}

fn read_file(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
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
