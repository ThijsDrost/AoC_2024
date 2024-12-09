use std::fs;

pub fn day9(base_path: &str, real: bool) -> (usize, usize) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day9.txt"
        } else {
            base_path.to_owned() + r"\inputs\day9_sample.txt"
        }
    };
    let values = fs::read_to_string(&path).unwrap();
    let char_vec: Vec<u32> = values.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let mut i1 = 0;
    let mut i2 = char_vec.len() - 1;
    let mut put: usize = 0;
    let mut index: usize = 0;
    let mut num = 0;

    'outer: loop {
        for _ in 0..char_vec[i1] {
            num += (i1/2)*index;
            index += 1;
        }
        i1 += 1;

        for _ in 0..char_vec[i1] {
            if put == char_vec[i2] as usize {
                i2 -= 2;
                put = 0;
                if i2 < i1 {
                    break 'outer;
                }
            }
            num += (i2/2)*index;
            index += 1;
            put += 1;
        }
        i1 += 1;

        if i1 == i2 {
            for _ in put..(char_vec[i1] as usize) {
                num += (i1/2)*index;
                index += 1;
                break 'outer;
            }
        }
    }

    (num, 0)
}
