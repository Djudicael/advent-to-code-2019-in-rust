pub fn day_two() {
    /* let noun = 12;
    let verb = 2;
    let mut data = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 9, 19, 23, 2, 23, 10, 27,
        1, 6, 27, 31, 1, 31, 6, 35, 2, 35, 10, 39, 1, 39, 5, 43, 2, 6, 43, 47, 2, 47, 10, 51, 1,
        51, 6, 55, 1, 55, 6, 59, 1, 9, 59, 63, 1, 63, 9, 67, 1, 67, 6, 71, 2, 71, 13, 75, 1, 75, 5,
        79, 1, 79, 9, 83, 2, 6, 83, 87, 1, 87, 5, 91, 2, 6, 91, 95, 1, 95, 9, 99, 2, 6, 99, 103, 1,
        5, 103, 107, 1, 6, 107, 111, 1, 111, 10, 115, 2, 115, 13, 119, 1, 119, 6, 123, 1, 123, 2,
        127, 1, 127, 5, 0, 99, 2, 14, 0, 0,
    ];

    data[1] = noun;
    data[2] = verb;
    println!("test   {:?}", computer(&data)[0]);*/

    let mut output: i64 = -1;

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut data = vec![
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 9, 19, 23, 2, 23,
                10, 27, 1, 6, 27, 31, 1, 31, 6, 35, 2, 35, 10, 39, 1, 39, 5, 43, 2, 6, 43, 47, 2,
                47, 10, 51, 1, 51, 6, 55, 1, 55, 6, 59, 1, 9, 59, 63, 1, 63, 9, 67, 1, 67, 6, 71,
                2, 71, 13, 75, 1, 75, 5, 79, 1, 79, 9, 83, 2, 6, 83, 87, 1, 87, 5, 91, 2, 6, 91,
                95, 1, 95, 9, 99, 2, 6, 99, 103, 1, 5, 103, 107, 1, 6, 107, 111, 1, 111, 10, 115,
                2, 115, 13, 119, 1, 119, 6, 123, 1, 123, 2, 127, 1, 127, 5, 0, 99, 2, 14, 0, 0,
            ];

            data[1] = noun;
            data[2] = verb;
            let result = computer(&data)[0];
            if result == 19690720 {
                output = 100 * noun + verb;
                break 'outer;
            }
        }
    }
    println!("outpu   {}", output);
}

fn calculate(prog: Vec<i64>) -> i64 {
    let mut p = prog.clone();
    let mut last = 0;
    let mut pos = 0;

    while pos < p.len() {
        match p[pos] {
            1 => {
                let p1 = p[pos + 1] as usize;
                let p2 = p[pos + 2] as usize;
                let p3 = p[pos + 3] as usize;
                p[p3] = p[p1] + p[p2];
                pos += 4;
                last = p[p3]
            }
            2 => {
                let p1 = p[pos + 1] as usize;
                let p2 = p[pos + 2] as usize;
                let p3 = p[pos + 3] as usize;
                p[p3] = p[p1] * p[p2];
                pos += 4;
                last = p[p3]
            }
            99 => {
                return last;
            }
            _ => panic!("error: wrong op code"),
        }
    }
    last
}

fn computer(input: &[i64]) -> Vec<i64> {
    let mut result = input.to_vec();
    let mut index = 0;
    loop {
        match result[index] {
            1 => {
                let first = result[index + 1] as usize;
                let second = result[index + 2] as usize;
                let third = result[index + 3] as usize;
                result[third] = result[first] + result[second];
                index += 4;
            }
            2 => {
                let first = result[index + 1] as usize;
                let second = result[index + 2] as usize;
                let third = result[index + 3] as usize;
                result[third] = result[first] * result[second];
                index += 4;
            }
            99 => break,
            _ => panic!("error: wrong op code"),
        }
    }
    result
}

#[cfg(test)]
mod test {

    use super::*;

    fn dump(program: Vec<i64>) -> String {
        program
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn test_computer() {
        assert_eq!(dump(computer(&[1, 0, 0, 0, 99])), "2,0,0,0,99");
        assert_eq!(dump(computer(&[2, 3, 0, 3, 99])), "2,3,0,6,99");
        assert_eq!(dump(computer(&[2, 4, 4, 5, 99, 0])), "2,4,4,5,99,9801");
        assert_eq!(
            dump(computer(&[1, 1, 1, 4, 99, 5, 6, 0, 99])),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
