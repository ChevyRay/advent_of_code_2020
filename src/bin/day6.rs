fn main() {
    let input = include_str!("day6.txt");

    let sum: usize = input
        .split("\n\n")
        .map(|group| {
            ('a'..='z')
                .filter(|chr| group.contains(*chr))
                .count()
        })
        .sum();

    println!("part 1: {}", sum);

    let sum: usize = input
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<&str>>())
        .map(|group| {
            ('a'..='z')
                .filter(|chr| group.iter().all(|line| line.contains(*chr)))
                .count()
        })
        .sum();

    println!("part 2: {}", sum);
}
