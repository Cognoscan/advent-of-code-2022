fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (score1, score2): (u32,u32) = input.lines().map(|line| {
        let score1 = match line {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!(),
        };
        let score2 = match line {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!(),
        };
        (score1,score2)
    }).fold((0,0), |a,b| (a.0+b.0, a.1+b.1));
    println!("Total score for part 1: {}", score1);
    println!("Total score for part 2: {}", score2);
}
