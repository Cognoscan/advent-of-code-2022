fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let sum1: u32 = input.lines()
        .map(|line| {
            let (l1, l2) = line.split_at(line.len()/2);
            let mut c = 'a';
            for c1 in l1.chars() {
                if l2.chars().any(|c2| c1 == c2) { 
                    c = c1; 
                    break;
                }
            }
            match c {
                'a'..='z' => u32::from(c) - u32::from('a') + 1,
                'A'..='Z' => u32::from(c) - u32::from('A') + 27,
                _ => 0,
            }
        })
        .sum();
    let sum2: u32 = lines.chunks_exact(3)
        .map(|elves| {
            let e1 = elves[0];
            let e2 = elves[1];
            let e3 = elves[2];
            let mut c = 'a';
            for c1 in e1.chars() {
                if e2.chars().any(|c2| c1 == c2) &&
                    e3.chars().any(|c3| c1 == c3) {
                        c = c1;
                        break;
                }
            }
            match c {
                'a'..='z' => u32::from(c) - u32::from('a') + 1,
                'A'..='Z' => u32::from(c) - u32::from('A') + 27,
                _ => 0,
            }
        })
        .sum();
    println!("Total for part 1: {}", sum1);
    println!("Total for part 2: {}", sum2);
}
