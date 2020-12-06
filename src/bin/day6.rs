use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("day6.txt");

    let sum: usize = input
        .split("\n\n")
        .map(|group| {
            group
                .replace('\n', "")
                .chars()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();

    println!("part 1: {}", sum);

    let sum: usize = input
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<&str>>())
        .map(|lines| {
            ('a'..='z')
                .filter(|chr| lines.iter().all(|line| line.contains(*chr)))
                .count()
        })
        .sum();

    println!("part 2: {}", sum);
}
