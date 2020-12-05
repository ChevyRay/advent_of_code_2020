fn main() {
    let input = include_str!("day4.txt");

    let values: Vec<(usize, usize)> = input
        .split("\n\n")
        .map(|group| {
            group
                .replace('\n', " ")
                .split(' ')
                .filter_map(|item| {
                    let mut pair = item.split(':');
                    Some((pair.next()?, pair.next()?))
                })
                .map(|(key, val)| match key {
                    "byr" => (
                        true,
                        val.len() == 4
                            && val
                                .parse::<usize>()
                                .and_then(|y| Ok(y >= 1920 && y <= 2002))
                                .unwrap_or(false),
                    ),
                    "iyr" => (
                        true,
                        val.len() == 4
                            && val
                                .parse::<usize>()
                                .and_then(|y| Ok(y >= 2010 && y <= 2020))
                                .unwrap_or(false),
                    ),
                    "eyr" => (
                        true,
                        val.len() == 4
                            && val
                                .parse::<usize>()
                                .and_then(|y| Ok(y >= 2020 && y <= 2030))
                                .unwrap_or(false),
                    ),
                    "hgt" => (
                        true,
                        (&val[0..val.len() - 2])
                            .parse::<usize>()
                            .and_then(|h| {
                                Ok(match &val[(val.len() - 2)..] {
                                    "cm" => h >= 150 && h <= 193,
                                    "in" => h >= 59 && h <= 76,
                                    _ => false,
                                })
                            })
                            .unwrap_or(false),
                    ),
                    "hcl" => (
                        true,
                        val.len() == 7
                            && val
                                .find('#')
                                .and_then(|i| {
                                    Some(i == 0 && usize::from_str_radix(&val[1..], 16).is_ok())
                                })
                                .unwrap_or(false),
                    ),
                    "ecl" => (
                        true,
                        match val {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                            _ => false,
                        },
                    ),
                    "pid" => (true, val.len() == 9 && val.chars().all(|c| c.is_numeric())),
                    "cid" => (false, false),
                    _ => (false, false),
                })
                .map(|(a, b)| (a as usize, b as usize))
                .fold((0, 0), |(na, nb), (a, b)| (na + a, nb + b))
        })
        .collect();

    let valid1 = values.iter().filter(|(a, _)| a >= &7).count();
    println!("part 1: {}", valid1);

    let valid2 = values.iter().filter(|(_, b)| b >= &7).count();
    println!("part 2: {}", valid2);
}
