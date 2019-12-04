pub fn part1(input: &[String]) -> i32 {
    find_closest_intersection(input[0].split(',').collect(), input[1].split(',').collect())
}

fn find_closest_intersection(cable1: Vec<&str>, cable2: Vec<&str>) -> i32 {
    let positions1 = get_positions(cable1);
    let positions2 = get_positions(cable2);

    positions1
        .iter()
        .filter(|(x, y)| positions2.contains(&(x.to_owned(), y.to_owned())))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("Min not found")
}

fn get_positions(input: Vec<&str>) -> Vec<(i32, i32)> {
    let mut actual_position: (i32, i32) = (0, 0);
    let mut positions: Vec<(i32, i32)> = vec![];

    input.iter().for_each(|i| {
        match i.to_lowercase().split_at(1) {
            ("r", x) => (1..=x.parse::<i32>().expect("")).for_each(|_| {
                actual_position.0 += 1;
                positions.push(actual_position);
            }),
            ("l", x) => (1..=x.parse::<i32>().expect("")).for_each(|_| {
                actual_position.0 -= 1;
                positions.push(actual_position);
            }),
            ("u", y) => (1..=y.parse::<i32>().expect("")).for_each(|_| {
                actual_position.1 += 1;
                positions.push(actual_position);
            }),
            ("d", y) => (1..=y.parse::<i32>().expect("")).for_each(|_| {
                actual_position.1 -= 1;
                positions.push(actual_position);
            }),
            _ => unreachable!(),
        };
    });
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1_test() {
        let input1: Vec<&str> = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let input2: Vec<&str> = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        let expected: i32 = 159;

        assert_eq!(expected, find_closest_intersection(input1, input2));
    }

    #[test]
    fn part1_2_test() {
        let input1: Vec<&str> = vec![
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let input2: Vec<&str> = vec![
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        let expected: i32 = 135;

        assert_eq!(expected, find_closest_intersection(input1, input2));
    }
}
