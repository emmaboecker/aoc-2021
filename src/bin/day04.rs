use std::collections::VecDeque;

type Board = Vec<Vec<(i32, bool)>>;

fn main() {
    let input = include_str!("../../input/day04.txt");

    let mut boards: VecDeque<&str> = input.split("\r\n\r\n").collect();

    let random_numbers = boards.pop_front().unwrap();
    let random_numbers: Vec<i32> = random_numbers.split(',').map(|number| number.parse().unwrap()).collect();

    let mut boards: Vec<Board> = boards.into_iter().map(|board| {
        board.lines().map(|line| {
            line.split(' ').filter(|string| !string.is_empty()).map(|number| (number.parse().unwrap(), false)).collect::<Vec<(i32, bool)>>()
        }).collect::<Board>()
    }).collect();

    let mut winning_sequence: Vec<(usize, i32, Vec<i32>)> = vec![];

    for number in random_numbers {
        boards.iter_mut().flatten().flatten().filter(|(value, _)| *value == number).for_each(|(_, marked)| *marked = true);

        'check: for (index, board) in boards.iter().enumerate() {
            if winning_sequence.iter().any(|(containing, _, _)| *containing == index) { continue; }

            for row in board {
                if row.iter().all(|(_, marked)| *marked) {
                    winning_sequence.push((index, number, board.iter().flatten().filter(|(_, marked)| !marked).map(|(number, _)| number.to_owned()).collect::<Vec<i32>>()));
                    continue 'check;
                }
            }

            for column in 0..board.len() {
                let mut all_marked = true;
                for row in board {
                    if !row[column].1 {
                        all_marked = false;
                        break;
                    }
                }

                if all_marked {
                    winning_sequence.push((index, number, board.iter().flatten().filter(|(_, marked)| !marked).map(|(number, _)| number.to_owned()).collect::<Vec<i32>>()));
                    break 'check;
                }
            }
        }
    }

    let first_win = &winning_sequence.first().unwrap();
    let part1 = first_win.2.iter().sum::<i32>() * winning_sequence.first().unwrap().1;

    println!("Part 1: {}", part1);

    let last_win = &winning_sequence.last().unwrap();
    let part2 = last_win.2.iter().sum::<i32>() * winning_sequence.last().unwrap().1;

    println!("Part 2: {}", part2);
}
