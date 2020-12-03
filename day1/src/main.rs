use itertools::Itertools;

fn main() {
    
    let input_str = include_str!("input.txt");
    let input: Vec<usize> = input_str
        .split('\n')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // PART 1:
    //  1) find the 2 numbers that sum to 2020
    //  2) multiply those numbers
    if let [a, b] = (
        input
        .iter()
        .combinations(2)
        .find(|n| n[0] + n[1] == 2020)
        .unwrap()
    )[..] {
        println!("result:\n\t{} + {} = 2020\n\t{} * {} = {}", a, b, a, b, a * b);
    }

    // PART 2:
    //  1) find 3 numbers that follow the same criteria as part 1
    if let [a, b, c] = (
        input
        .iter()
        .combinations(3)
        .find(|n| n[0] + n[1] + n[2] == 2020)
        .unwrap()
    )[..] {
        println!("result:\n\t{} + {} + {} = 2020\n\t{} * {} * {} = {}", a, b, c, a, b, c, a * b * c);
    }
}
