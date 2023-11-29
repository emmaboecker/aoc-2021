fn main() {
    let input = include_str!("../../input/day01.txt").lines().map(|line| line.parse::<i32>().unwrap());

    let mut increases = 0;


    let mut peekable = input.clone().peekable();
    while let Some(line) = peekable.next() {
        let next = peekable.peek();
        if next.is_none() {
            break;
        }
        let next = next.unwrap();

        if line < *next {
            increases += 1;
        }
    }

    println!("Part 1: {}", increases);

    increases = 0;

    let vec = input.collect::<Vec<i32>>();
    let mut peekable = vec.windows(3).peekable();
    while let Some(window) = peekable.next() {
        let next = peekable.peek();
        if next.is_none() {
            break;
        }
        let next = next.unwrap();

        let current_sum: i32 = window.iter().sum();
        let next_sum: i32 = next.iter().sum();

        if current_sum < next_sum {
            increases += 1;
        }
    }

    println!("Part 2: {}", increases);
}
