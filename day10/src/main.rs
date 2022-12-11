enum State {
    Addx(i32),
    Noop,
}

fn main() {
    let input = include_str!("input");
    //let input = include_str!("sample");

    // CPU setup
    let mut state = State::Noop;
    let mut instr_seq = input.lines();
    let mut cycle = 1;
    let mut reg_x = 1;

    // Capture for Part 1
    let mut capture = 20;
    let mut cap_sum = 0;

    // CRT for Part 2
    let mut crt: Vec<bool> = Vec::with_capacity(40*6);

    // Run the CPU
    loop {
        let pixel_pos = (cycle-1) % 40;
        if pixel_pos >= reg_x-1 && pixel_pos <= reg_x+1 {
            crt.push(true);
        }
        else {
            crt.push(false);
        }
        if let State::Addx(val) = state {
            reg_x += val;
            state = State::Noop;
        }
        else {
            let Some(instr) = instr_seq.next() else { break; };
            if let Some(val) = instr.strip_prefix("addx ") {
                state = State::Addx(val.parse::<i32>().unwrap());
            }
        }
        cycle += 1;
        if cycle >= capture {
            cap_sum += capture * reg_x;
            capture += 40;
        }
    }
    crt.pop();

    println!("Part 1 Answer: {}", cap_sum);
    println!("Part 2 CRT Display:");
    println!();
    for line in crt.chunks(40) {
        for pixel in line.iter() {
            if *pixel { print!("█"); } else { print!(" "); }
        }
        println!();
    }

}
