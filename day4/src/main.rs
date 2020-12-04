fn main() {
    let input = include_str!("input.txt");

    let valid_count = input
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
                    "byr" => {
                        val.len() == 4
                            && val
                                .parse::<usize>()
                                .and_then(|y| Ok(y >= 1920 && y <= 2002))
                                .unwrap_or(false)
                    }
                    "iyr" => {
                        val.len() == 4
                            && val
                                .parse::<usize>()
                                .and_then(|y| Ok(y >= 2010 && y <= 2020))
                                .unwrap_or(false)
                    }
                    "eyr" => {
                        val.len() == 4
                            && val
                                .parse::<usize>()
                                .and_then(|y| Ok(y >= 2020 && y <= 2030))
                                .unwrap_or(false)
                    }
                    "hgt" => (&val[0..val.len() - 2])
                        .parse::<usize>()
                        .and_then(|h| {
                            Ok(match &val[(val.len() - 2)..] {
                                "cm" => h >= 150 && h <= 193,
                                "in" => h >= 59 && h <= 76,
                                _ => false,
                            })
                        })
                        .unwrap_or(false),
                    "hcl" => {
                        val.len() == 7
                            && val
                                .find('#')
                                .and_then(|i| {
                                    Some(i == 0 && usize::from_str_radix(&val[1..], 16).is_ok())
                                })
                                .unwrap_or(false)
                    }
                    "ecl" => match val {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false,
                    },
                    "pid" => val.len() == 9 && val.chars().all(|c| c.is_numeric()),
                    "cid" => false,
                    _ => false,
                })
                .filter(|&v| v)
                .count()
        })
        .filter(|&v| v >= 7)
        .count();

    println!("valid passports: {}", valid_count);
}
