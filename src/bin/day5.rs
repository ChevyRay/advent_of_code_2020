fn main() {
    let input = include_str!("day5.txt");

    fn search(line: &str, range: usize) -> usize {
        line.chars()
            .fold((0, range), |(a, b), chr| match chr {
                'F' | 'L' => (a, b - (b - a) / 2),
                _ => (a + (b - a) / 2, b),
            })
            .0
    }

    let mut ids: Vec<usize> = input
        .lines()
        .map(|line| (search(&line[0..7], 128), search(&line[7..10], 8)))
        .map(|(row, col)| (row * 8 + col))
        .collect();

    let max_id = *ids.iter().max().unwrap();
    println!("highest id: {}", max_id);

    ids.sort();

    let my_id = 1 + *ids[0..ids.len() - 1]
        .iter()
        .enumerate()
        .find(|(i, id)| *id + 1 != ids[i + 1])
        .unwrap()
        .1;

    println!("my id: {}", my_id);
}
