pub fn part1(input: &[i32]) -> i32 {
    let mod_input = modifiy_input(input.to_vec(), 12, 2);
    run_intcode(mod_input)[0]
}

pub fn part2(input: &[i32]) -> i32 {
    let searched_ouptut = 19_690_720;

    for i in 0..100 {
        for j in 0..100 {
            let mod_input = modifiy_input(input.to_vec(), i, j);
            if run_intcode(mod_input)[0] == searched_ouptut {
                return i * 100 + j;
            }
        }
    }
    0
}

fn modifiy_input(input: Vec<i32>, first: i32, second: i32) -> Vec<i32> {
    let mut mod_input = input.clone();
    mod_input[1] = first;
    mod_input[2] = second;
    mod_input
}

fn run_intcode(input: Vec<i32>) -> Vec<i32> {
    let mut result = input.clone();
    for i in (0..input.len()).step_by(4) {
        match result[i] {
            1 => {
                let first_pos = result[i + 1] as usize;
                let second_pos = result[i + 2] as usize;
                let res_pos = result[i + 3] as usize;
                result[res_pos] = result[first_pos] + result[second_pos];
            }
            2 => {
                let first_pos = result[i + 1] as usize;
                let second_pos = result[i + 2] as usize;
                let res_pos = result[i + 3] as usize;
                result[res_pos] = result[first_pos] * result[second_pos];
            }
            99 => break,
            _ => unreachable!(),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1_test() {
        let input: Vec<i32> = vec![1, 0, 0, 0, 99];
        let expected: Vec<i32> = vec![2, 0, 0, 0, 99];
        assert_eq!(expected, run_intcode(input));
    }

    #[test]
    fn part1_2_test() {
        let input: Vec<i32> = vec![2, 3, 0, 3, 99];
        let expected: Vec<i32> = vec![2, 3, 0, 6, 99];
        assert_eq!(expected, run_intcode(input));
    }

    #[test]
    fn part1_3_test() {
        let input: Vec<i32> = vec![2, 4, 4, 5, 99, 0];
        let expected: Vec<i32> = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(expected, run_intcode(input));
    }

    #[test]
    fn part1_4_test() {
        let input: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected: Vec<i32> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(expected, run_intcode(input));
    }
}
