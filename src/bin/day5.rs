fn main() {
    let input = include_str!("day5.txt");

    let mut ids: Vec<usize> = input
        .lines()
        .map(|line| {
            (
                line[0..7]
                    .chars()
                    .fold((0, 128), |(a, b), chr| match chr {
                        'F' => (a, b - (b - a) / 2),
                        _ => (a + (b - a) / 2, b),
                    })
                    .0,
                line[7..10]
                    .chars()
                    .fold((0, 8), |(a, b), chr| match chr {
                        'L' => (a, b - (b - a) / 2),
                        _ => (a + (b - a) / 2, b),
                    })
                    .0,
            )
        })
        .map(|(row, col)| (row * 8 + col))
        .collect();

    let max_id = *ids.iter().max().unwrap();
    println!("highest id: {}", max_id);

    ids.sort();

    let my_id = *ids[0..ids.len() - 1]
        .iter()
        .enumerate()
        .find(|(i, id)| *id + 1 != ids[i + 1])
        .unwrap().1 + 1;

    println!("my id: {}", my_id);
}
