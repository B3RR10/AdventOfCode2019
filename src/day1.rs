pub fn part1(input: &[i32]) -> i32 {
    input.iter().map(|n| get_fuel(n.to_owned())).sum()
}

pub fn part2(input: &[i32]) -> i32 {
    input.iter().map(|n| get_fuel_recursive(n.to_owned())).sum()
}

fn get_fuel(mass: i32) -> i32 {
    (((mass as f32) / 3.0_f32).floor() - 2.0_f32) as i32
}

fn get_fuel_recursive(mass: i32) -> i32 {
    let mut fuel = get_fuel(mass);
    if fuel > 0 {
        fuel += get_fuel_recursive(fuel);
    } else {
        fuel = 0;
    }
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_for_12_test() {
        let input = 12;
        let expected = 2;
        assert_eq!(expected, get_fuel(input));
    }

    #[test]
    fn fuel_for_14_test() {
        let input = 14;
        let expected = 2;
        assert_eq!(expected, get_fuel(input));
    }

    #[test]
    fn fuel_for_1969_test() {
        let input = 1969;
        let expected = 654;
        assert_eq!(expected, get_fuel(input));
    }

    #[test]
    fn fuel_for_100756_test() {
        let input = 100_756;
        let expected = 33583;
        assert_eq!(expected, get_fuel(input));
    }

    #[test]
    fn fuel_recurse_for_14_test() {
        let input = 12;
        let expected = 2;
        assert_eq!(expected, get_fuel_recursive(input));
    }

    #[test]
    fn fuel_recurse_for_1969_test() {
        let input = 1969;
        let expected = 966;
        assert_eq!(expected, get_fuel_recursive(input));
    }

    #[test]
    fn fuel_recurse_for_100756_test() {
        let input = 100_756;
        let expected = 50346;
        assert_eq!(expected, get_fuel_recursive(input));
    }
}
