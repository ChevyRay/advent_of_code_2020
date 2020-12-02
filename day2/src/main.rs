// line format: "[min]-[max] [letter]: [password]"
fn split_line<'a>(line: &'a str) -> (usize, usize, char, &'a str) {
    let mut line_split = line.split(' ');
    let nums = line_split.next().unwrap();
    let letter = line_split.next().unwrap().chars().next().unwrap();
    let password = line_split.next().unwrap();
    let mut nums_split = nums.split('-');
    let min = nums_split.next().unwrap().parse::<usize>().unwrap();
    let max = nums_split.next().unwrap().parse::<usize>().unwrap();
    (min, max, letter, password)
}

fn main() {
    let input = include_str!("input.txt");

    // PART 1)
    //  Each line gives the password policy and then the password.
    //  The password policy indicates the lowest and highest number
    //  of times a given letter must appear for the password to be
    //  valid. For example, 1-3 a means that the password must
    //  contain a at least 1 time and at most 3 times.
    let valid_passwords: usize = input.split('\n').filter(|line| {
        let (min, max, letter, password) = split_line(line);
        let letter_count = password.chars().filter(|c| c == &letter).count();
        letter_count >= min && letter_count <= max
    }).count();
    println!("{}", valid_passwords);

    // PART 2)
    //  Each policy actually describes two positions in the password,
    //  where 1 means the first character, 2 means the second character,
    //  and so on. (Be careful; Toboggan Corporate Policies have no
    //  concept of "index zero"!) Exactly one of these positions must
    //  contain the given letter. Other occurrences of the letter are
    //  irrelevant for the purposes of policy enforcement.
    let valid_passwords: usize = input.split('\n').filter(|line| {
        let (a, b, letter, password) = split_line(line);
        let chars: Vec<char> = password.chars().collect();
        let (a, b) = (chars[a - 1], (chars[b - 1]));
        (a == letter || b == letter) && (a == letter) != (b == letter)
    }).count();
    println!("{}", valid_passwords);
}
