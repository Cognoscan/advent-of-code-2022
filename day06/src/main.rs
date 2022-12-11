fn main() {
    let input = include_str!("input");
    let start_packet = 4 + input.as_bytes()
        .windows(4)
        .position(|bytes| {
            bytes.iter().enumerate().all(|(i, b0)| {
                bytes.iter().skip(i+1).all(|b1| b0 != b1)
            })
        })
        .unwrap();

    let start_message = 14 + input.as_bytes()
        .windows(14)
        .position(|bytes| {
            bytes.iter().enumerate().all(|(i, b0)| {
                bytes.iter().skip(i+1).all(|b1| b0 != b1)
            })
        })
        .unwrap();
    println!("Start packet position is {}", start_packet);
    println!("Start message position is {}", start_message);
}
