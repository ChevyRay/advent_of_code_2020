fn main() {
    let input = include_str!("day8.txt");

    let code: Vec<(&str, i32)> = input
        .lines()
        .map(|line| line.split(" "))
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    fn run_code(code: &Vec<(&str, i32)>, swap_i: usize, mut swap_op: Option<&str>) -> (usize, i32) {
        let mut called = vec![false; code.len()];
        std::iter::successors(Some((0, 0i32)), |&(i, acc)| {
            if i < called.len() && !called[i] {
                called[i] = true;
                let op = if i == swap_i {
                    swap_op.take().unwrap_or(code[i].0)
                } else {
                    code[i].0
                };
                match op {
                    "acc" => Some((i + 1, acc + code[i].1)),
                    "jmp" => Some((i.wrapping_add(code[i].1 as usize), acc)),
                    _ => Some((i + 1, acc)),
                }
            } else {
                None
            }
        })
        .fold((0, 0), |_, x| x)
    }

    let (_, acc) = run_code(&code, 0, None);

    println!("part 1: {}", acc);

    let (_, acc) = (0..code.len())
        .filter(|&s| code[s].0 != "acc")
        .map(|s| {
            run_code(
                &code,
                s,
                Some(match code[s].0 {
                    "jmp" => "nop",
                    _ => "acc",
                }),
            )
        })
        .find(|(i, _)| *i == code.len())
        .unwrap();

    println!("part 2: {}", acc);
}
