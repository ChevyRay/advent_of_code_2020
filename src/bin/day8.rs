fn main() {
    let input = include_str!("day8.txt");

    let mut code: Vec<(&str, i32)> = input
        .lines()
        .map(|line| line.split(" "))
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    fn run_code(code: &Vec<(&str, i32)>) -> (usize, i32) {
        let mut called = vec![false; code.len()];
        std::iter::successors(Some((0, 0i32)), |&(i, acc)| {
            if i < called.len() && !called[i] {
                called[i] = true;
                match code[i].0 {
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
    
    let (_, acc) = run_code(&code);

    println!("part 1: {}", acc);



    for s in 0..code.len() {
        let prev = code[s].0;
        if prev != "acc" {
            code[s].0 = match prev {
                "jmp" => "nop",
                "nop" => "jmp",
                _ => "acc",
            };
            let (i, acc) = run_code(&code);
            code[s].0 = prev;
            if i == code.len() {
                println!("part 2: {}", acc);
                break;
            }
        }
    }
}
