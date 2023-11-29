fn main() {
    let input = include_str!("../../input/day03.txt").lines();

    let lines: Vec<_> = input.collect();

    let length = lines.first().unwrap().len();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let mut oxygen_generator = lines.clone();
    let mut co2_scrubber = lines.clone();

    for i in 0..length {
        let vertical_line = lines.iter().map(|line| line.chars().nth(i).unwrap()).collect::<String>();
        let ones = vertical_line.matches('1').count();
        let zeros = vertical_line.matches('0').count();

        if ones > zeros {
            gamma_rate |= 1 << ((length - 1) - i);
        } else {
            epsilon_rate |= 1 << ((length - 1) - i);
        }

        oxygen_generator = retained_respecitvely(oxygen_generator, i,true);
        co2_scrubber = retained_respecitvely(co2_scrubber, i,false);
    }

    println!("Part 1: {}", gamma_rate * epsilon_rate);

    let oxygen_generator = i32::from_str_radix(oxygen_generator.first().unwrap().trim(), 2).unwrap();
    let co2_scrubber = i32::from_str_radix(co2_scrubber.first().unwrap().trim(), 2).unwrap();

    println!("Part 2: {}", oxygen_generator * co2_scrubber);
}

fn retained_respecitvely(lines: Vec<&str>, index: usize, most_common: bool) -> Vec<&str> {
    if lines.len() == 1 {
        return lines;
    }

    let mut lines = lines.clone();
    let vertical_line = lines.iter().map(|line| line.chars().nth(index).unwrap()).collect::<String>();
    let ones = vertical_line.matches('1').count();
    let zeros = vertical_line.matches('0').count();

    if ones >= zeros {
        if most_common {
            lines.retain(|line| line.chars().nth(index).unwrap() == '1');
        } else {
            lines.retain(|line| line.chars().nth(index).unwrap() == '0');
        }
    } else if most_common {
        lines.retain(|line| line.chars().nth(index).unwrap() == '0');
    } else {
        lines.retain(|line| line.chars().nth(index).unwrap() == '1');
    }

    lines
}
