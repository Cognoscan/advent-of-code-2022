fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let this_elf = std::mem::take(&mut elf);
            elves.push(this_elf);
        }
        else {
            elf.push(line.parse::<u32>().unwrap());
        }
    }
    if !elf.is_empty() { elves.push(elf); }

    let mut sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum()).collect();
    sums.sort_unstable();

    println!("Got {} elves", elves.len());
    println!("Top 3 elves:");
    let mut max = 0;
    for cals in sums.iter().rev().take(3) {
        println!("{cals}");
        max += cals;
    }
    println!("Total of top 3: {}", max);
}
