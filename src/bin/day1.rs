fn main() {
    let input = include_str!("day1.txt");

    let nums: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let (a, b) = nums
        .iter()
        .enumerate()
        .map(|(i, a)| nums[(i + 1)..].iter().map(move |b| (*a, *b)))
        .flatten()
        .find(|(a, b)| a + b == 2020)
        .unwrap();

    println!("first: {}", a * b);

    let (a, b, c) = nums
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let rest = &nums[(i + 1)..];
            rest.iter()
                .enumerate()
                .map(move |(i, b)| rest[(i + 1)..].iter().map(move |c| (*a, *b, *c)))
                .flatten()
        })
        .flatten()
        .find(|(a, b, c)| a + b + c == 2020)
        .unwrap();

    println!("second: {}", a * b * c);
}
