#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let inst = &line[0..3];
        let arg: i32 = line[4..].parse().unwrap();
        match inst {
            "acc" => Self::Acc(arg),
            "jmp" => Self::Jmp(arg),
            "nop" => Self::Nop(arg),
            _ => panic!(),
        }
    }
}

type InputType = Vec<Instruction>;

pub fn input(input: &str) -> InputType {
    input.lines().map(|l| Instruction::from(l.trim())).collect()
}

fn run(input: &[Instruction]) -> (i32, bool) {
    let mut acc = 0;
    let mut ip = 0;
    let mut visited = vec![false; input.len()];
    while ip < visited.len() && !visited[ip] {
        visited[ip] = true;
        match input[ip] {
            Instruction::Acc(arg) => {
                acc += arg;
                ip += 1;
            }
            Instruction::Jmp(arg) => {
                if arg > 0 {
                    ip += arg as usize;
                } else {
                    ip -= -arg as usize;
                }
            }
            Instruction::Nop(_) => {
                ip += 1;
            }
        }
    }

    (acc, ip < visited.len())
}

pub fn part1(input: InputType) -> i32 {
    run(&input).0
}

pub fn part2(mut input: InputType) -> i32 {
    let mut idx = 0;
    let first = run(&input);
    if !first.1 {
        return first.0;
    }
    loop {
        let replacement: Instruction;
        loop {
            match input[idx] {
                Instruction::Acc(_) => {}
                Instruction::Jmp(arg) => {
                    replacement = Instruction::Nop(arg);
                    break;
                }
                Instruction::Nop(arg) => {
                    replacement = Instruction::Jmp(arg);
                    break;
                }
            }
            idx += 1;
        }
        let old = std::mem::replace(&mut input[idx], replacement);
        let res = run(&input);
        if !res.1 {
            return res.0;
        }
        input[idx] = old;
        idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        assert!(part1(input(input_str)) == 5);
    }

    #[test]
    fn p2() {
        let input_str = r"nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        assert!(part2(input(input_str)) == 8);
    }
}
