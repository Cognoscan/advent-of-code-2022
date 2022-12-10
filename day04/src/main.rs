use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let ranges: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = input
        .lines()
        .map(|line| {
            let (r0, r1) = line.split_once(',').unwrap();
            let (r00, r01) = r0.split_once('-').unwrap();
            let (r10, r11) = r1.split_once('-').unwrap();
            (
                r00.parse::<u32>().unwrap()..=r01.parse::<u32>().unwrap(),
                r10.parse::<u32>().unwrap()..=r11.parse::<u32>().unwrap(),
            )
        })
        .collect();
    let sum1: u32 = ranges.iter()
        .map(|(r0, r1)| {
            let cond = r0.contains(r1.start()) && r0.contains(r1.end())
                || r1.contains(r0.start()) && r1.contains(r0.end());
            u32::from(cond)
        })
        .sum();
    let sum2: u32 = ranges.iter()
        .map(|(r0, r1)| {
            let cond = r0.contains(r1.start()) || r0.contains(r1.end())
                || r1.contains(r0.start()) || r1.contains(r0.end());
            u32::from(cond)
        })
        .sum();

    println!("Part 1 sum: {}", sum1);
    println!("Part 2 sum: {}", sum2);
}
