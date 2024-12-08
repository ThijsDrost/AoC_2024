use std::fs;
use std::collections::{HashSet, HashMap};

pub fn day8(base_path: &str, real: bool) -> (usize, usize) {
    let path: String = {
        if real {
            base_path.to_owned() + r"\inputs\day8.txt"
        } else {
            base_path.to_owned() + r"\inputs\day8_sample.txt"
        }
    };
    let values = fs::read_to_string(&path).unwrap();
    let mappert = values.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    let height = mappert.len();
    let width = mappert[0].len();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();


    for (i1, line) in mappert.into_iter().enumerate() {
        for (i2, char) in line.into_iter().enumerate() {
            if char != '.' {
                if antennas.contains_key(&char) {
                    antennas.get_mut(&char).unwrap().push((i1, i2));
                }
                else {
                    antennas.insert(char, vec![(i1, i2)]);
                }
            }
        }
    }
    let mut places: HashSet<(usize, usize)> = HashSet::new();
    for key in antennas.keys() {
        let vec = antennas.get(key).unwrap();
        for i1 in 0..(vec.len() - 1) {
            for i2 in (i1 + 1)..vec.len() {
                let diff = (vec[i1].0 as i32 - vec[i2].0 as i32, vec[i1].1 as i32 - vec[i2].1 as i32);
                let loc1 = (vec[i1].0 as i32 + diff.0, vec[i1].1 as i32 + diff.1);
                let loc2 = (vec[i2].0 as i32 - diff.0, vec[i2].1 as i32 - diff.1);

                if 0 <= loc1.0 && (loc1.0 as usize) < height && 0 <= loc1.1 && (loc1.1 as usize) < width {
                    places.insert((loc1.0 as usize, loc1.1 as usize));
                }
                if 0 <= loc2.0 && (loc2.0 as usize) < width && 0 <= loc2.1 && (loc2.1 as usize) < width {
                    places.insert((loc2.0 as usize, loc2.1 as usize));
                }
            }
        }
    }

    #[inline]
    fn harmonic_inserter(mut location: (i32, i32), diff: (i32, i32), mut set: HashSet<(usize, usize)>, width: i32, height: i32)
                         -> HashSet<(usize, usize)> {
        while 0 <= location.0 && location.0 < height && 0 <= location.1 && location.1 < width {
            set.insert((location.0 as usize, location.1 as usize));
            location = (location.0 + diff.0, location.1 + diff.1);
        }
        set
    }

    let mut harmonic_places: HashSet<(usize, usize)> = HashSet::new();
    for key in antennas.keys() {
        let vec = antennas.get(key).unwrap();
        for i1 in 0..(vec.len() - 1) {
            for i2 in (i1 + 1)..vec.len() {
                let diff = (vec[i1].0 as i32 - vec[i2].0 as i32, vec[i1].1 as i32 - vec[i2].1 as i32);
                let loc1 = (vec[i1].0 as i32, vec[i1].1 as i32);
                let loc2 = (vec[i2].0 as i32, vec[i2].1 as i32);

                harmonic_places = harmonic_inserter(loc1, diff, harmonic_places, width as i32, height as i32);
                harmonic_places = harmonic_inserter(loc2, (-diff.0, -diff.1), harmonic_places, width as i32, height as i32);
            }
        }
    }
    (places.len(), harmonic_places.len())
}