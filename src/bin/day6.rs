fn main() {
    let input = include_str!("day6.txt");
    let groups: Vec<&str> = input.split("\n\n").collect();

    let sum: usize = groups
        .iter()
        .map(|group| ('a'..='z').filter(|chr| group.contains(*chr)).count())
        .sum();

    println!("part 1: {}", sum);

    let sum: usize = groups
        .iter()
        .map(|group| group.lines().collect::<Vec<&str>>())
        .map(|group| {
            ('a'..='z')
                .filter(|chr| group.iter().all(|line| line.contains(*chr)))
                .count()
        })
        .sum();

    println!("part 2: {}", sum);
}
