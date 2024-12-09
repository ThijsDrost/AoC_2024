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

    let mut holes: Vec<(u32, u32)> = Vec::new();
    let mut items: Vec<(u32, u32)> = Vec::new();
    let mut indexes = 0;

    for (index, value) in char_vec.iter().enumerate() {
        if index % 2 == 0 {
            items.push((indexes, *value));
            indexes += value;
        }
        else {
            holes.push((indexes, *value));
            indexes += value;
        }
    }

    let mut total2 = 0;
    let mut found = None;
    for (item_index, item) in items.iter().enumerate().rev() {
        found = None;
        for (hole_index, hole) in holes.iter_mut().enumerate(){
            if item.0 < hole.0 {
                break;
            }
            if item.1 <= hole.1 {
                for value in hole.0..(hole.0+item.1) {
                    total2 += item_index*value as usize;
                }
                hole.0 += item.1;
                hole.1 -= item.1;
                found = Some((item.0, item.1, hole_index));
                break;
            }
        }
        if let Some((idx, length, hole_index)) = found {
            for i in hole_index..holes.len() {
                if idx < holes[i].0 {
                    if (idx + length) == holes[i].0 {
                        holes[i].0 = idx;
                        holes[i].1 += length;
                    }
                    else {
                        holes.insert(i, (idx, length));
                    }
                    break;
                }
            }
        }
        else {
            for value in item.0..(item.0 + item.1) {
                total2 += item_index * value as usize;
            }
        }
    }
    (num, total2)
}