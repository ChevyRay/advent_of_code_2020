fn main() {
    let input = include_str!("day9.txt");
    let nums: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let (index, weakness) = nums
        .iter()
        .enumerate()
        .skip(25)
        .find(|&(i, num)| {
            nums[i - 25..i]
                .iter()
                .flat_map(|a| nums[i - 25..i].iter().map(move |b| (a, b)))
                .find(|&(a, b)| a + b == *num)
                .is_none()
        })
        .unwrap();

    println!("part 1: {}", weakness);

    let (i, j) = (0..index - 1)
        .filter_map(|i| {
            (i..index - 1)
                .map(|j| (j, nums[i..j].iter().sum::<usize>()))
                .find(|(_, sum)| sum == weakness)
                .and_then(|(j, _)| Some((i, j)))
        })
        .fold((0, 0), |_, x| x);

    let min = nums[i..j].iter().min().unwrap();
    let max = nums[i..j].iter().max().unwrap();

    println!("part 2: {}", min + max);
}
