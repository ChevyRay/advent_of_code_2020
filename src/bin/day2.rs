fn main() {
    let input = include_str!("day2.txt");

    let (valid1, valid2) = input
        .lines()
        .map(|line| {
            let mut line_split = line.split(' ');
            let nums = line_split.next().unwrap();
            let letter = line_split.next().unwrap().chars().next().unwrap();
            let password = line_split.next().unwrap();
            let mut nums_split = nums.split('-');
            let min = nums_split.next().unwrap().parse().unwrap();
            let max = nums_split.next().unwrap().parse().unwrap();
            let chars: Vec<char> = password.chars().collect();
            let letter_count = chars.iter().filter(|&c| c == &letter).count();
            let (a, b) = (chars[min - 1], (chars[max - 1]));
            (
                (letter_count >= min && letter_count <= max) as usize,
                ((a == letter || b == letter) && (a == letter) != (b == letter)) as usize
            )
        })
        .fold((0, 0), |(na, nb),(a, b)| (na + a, nb + b));
    
    println!("part 1: {}", valid1);
    println!("part 2: {}", valid2);
}
