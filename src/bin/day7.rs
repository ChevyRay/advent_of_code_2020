use std::collections::HashMap;

type Bag = HashMap<&'static str, usize>;
type Bags = HashMap<&'static str, Bag>;

fn main() {
    let input = include_str!("day7.txt");

    let bags: Bags = input
        .lines()
        .map(|line| line[..line.len() - 1].split(" bags contain "))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap().split(", ")))
        .map(|(outer, bags)| {
            (
                outer,
                bags.map(|bag| (bag, bag.find(' ').unwrap(), bag.rfind(' ').unwrap()))
                    .map(|(bag, i, j)| {
                        (
                            match &bag[0..i] {
                                "no" => 0,
                                _ => bag[0..i].parse::<usize>().unwrap(),
                            },
                            &bag[i + 1..j],
                        )
                    })
                    .filter(|(n, _)| n > &0)
                    .map(|(n, ty)| (ty, n))
                    .collect::<Bag>(),
            )
        })
        .collect();

    fn can_hold(bags: &Bags, outer: &Bag, inner: &str) -> bool {
        outer.contains_key(inner)
            || outer
                .keys()
                .any(|&outer| can_hold(bags, bags.get(outer).unwrap(), inner))
    }

    let count: usize = bags
        .values()
        .filter(|bag| can_hold(&bags, bag, "shiny gold"))
        .count();

    println!("part 1: {}", count);

    fn count_inner(bags: &Bags, outer: &Bag) -> usize {
        outer
            .iter()
            .map(|(inner, n)| n + n * count_inner(bags, bags.get(inner).unwrap()))
            .sum::<usize>()
    }

    let count: usize = count_inner(&bags, bags.get("shiny gold").unwrap());

    println!("part 2: {}", count);
}
