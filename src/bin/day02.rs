fn main() {
    let input = include_str!("../../input/day02.txt");

    let mut horizontal_position = 0;
    let mut depth_one = 0;
    let mut depth_two = 0;
    let mut aim = 0;

    for line in input.lines() {
        let (action, amount) = line.split_once(' ').unwrap();
        let amount: i32 = amount.parse().unwrap();

        match action {
            "forward" => {
                horizontal_position += amount;
                depth_two += aim * amount;
            },
            "up" => {
                depth_one -= amount;
                aim -= amount;
            },
            "down" => {
                depth_one += amount;
                aim += amount;
            },
            _ => panic!("Unknown action: {}", action),
        }
    }

    println!("Part 1: {}", horizontal_position * depth_one);
    println!("Part 2: {}", horizontal_position * depth_two);
}
