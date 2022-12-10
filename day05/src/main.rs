
struct Command {
    src: usize,
    dst: usize,
    cnt: usize,
}

impl Command {
    fn parse(s: &str) -> Self {
        let mut s = s.split_ascii_whitespace();
        // "move cnt from src to dst"
        let _ = s.next().unwrap();
        let cnt = s.next().unwrap().parse::<usize>().unwrap();
        let _ = s.next().unwrap();
        let src = s.next().unwrap().parse::<usize>().unwrap() - 1;
        let _ = s.next().unwrap();
        let dst = s.next().unwrap().parse::<usize>().unwrap() - 1;
        Self { src, dst, cnt }
    }
}

fn main() {
    let input = include_str!("input");
    let mut command_section = false;

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut stacks2: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut crane = Vec::new();
    for line in input.lines() {
        if !command_section {
            if !line.trim_start_matches(' ').starts_with('[') {
                stacks.iter_mut().for_each(|s| s.reverse());
                stacks2 = stacks.clone();
                command_section = true;
                continue;
            }
            let line = line.as_bytes();
            for i in 0usize..9 {
                let c = line[i*4+1];
                if c != b' ' {
                    stacks[i].push(c as char);
                }
            }
        }
        else {
            if line.is_empty() { continue; }
            let command = Command::parse(line);

            // Part 1
            let len = stacks[command.src].len();
            crane.extend(stacks[command.src].drain(len-command.cnt..));
            crane.reverse();
            stacks[command.dst].append(&mut crane);

            // Part 2
            let len = stacks2[command.src].len();
            crane.extend(stacks2[command.src].drain(len-command.cnt..));
            stacks2[command.dst].append(&mut crane);
        }
    }

    print!("Part 1: ");
    for s in stacks.iter() {
        print!("{}", s.last().unwrap());
    }
    println!();

    print!("Part 2: ");
    for s in stacks2.iter() {
        print!("{}", s.last().unwrap());
    }
    println!();

}
