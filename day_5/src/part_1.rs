use std::{
    cmp::min,
    collections::HashMap,
    fs::{self},
    io::BufRead,
    ops::Range,
};

const INPUT_FILE: &str = "input.txt";
const LOCATION_KEY: &str = "location";
const MAP_SUFFIX: &str = " map:";

fn main() {
    let input_file_content = fs::read_to_string(INPUT_FILE).unwrap();

    let initial_seeds: Vec<u64> = input_file_content.lines().next().unwrap()[7..]
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();

    type SrcKey = String;
    type DstKey = String;
    type SrcRange = Range<u64>;
    type DstRange = Range<u64>;

    let mut transformations: HashMap<(SrcKey, DstKey), Vec<(SrcRange, DstRange)>> =
        HashMap::with_capacity(16);
    let mut transformation_sequence: Vec<(SrcKey, DstKey)> = Vec::with_capacity(16);

    input_file_content
        .split("\n\n")
        // Skip the "seeds:" header
        .skip(1)
        .for_each(|block| {
            let (src_key, dst_key) = block
                .lines()
                .next()
                // Get rid of " map:" suffix
                .map(|raw_header| &raw_header[..=raw_header.len() - MAP_SUFFIX.len()])
                // Collect "src_key" and "dst_key"
                .map(|header| header.split('-').collect::<Vec<&str>>())
                .map(|parts| (parts[0].trim().to_owned(), parts[2].trim().to_owned()))
                .unwrap();

            transformation_sequence.push((src_key.clone(), dst_key.clone()));

            block.lines().skip(1).for_each(|map_line| {
                let parts = map_line
                    .split(' ')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let (dst_range_start, src_range_start, range_length) =
                    (parts[0], parts[1], parts[2]);
                let entry = transformations
                    .entry((src_key.clone(), dst_key.clone()))
                    .or_default();
                entry.push((
                    src_range_start..src_range_start + range_length,
                    dst_range_start..dst_range_start + range_length,
                ));
            })
        });

    let mut min_location = None;
    for seed in initial_seeds {
        let mut id = seed;
        for key_pair in &transformation_sequence {
            let ranges = transformations[key_pair].clone();
            for (src, dst) in ranges {
                if src.contains(&id) {
                    let offset = id - src.start;
                    id = dst.start + offset;
                    break;
                }
            }
            if key_pair.1 == LOCATION_KEY {
                if let Some(ml) = min_location {
                    min_location = Some(min(ml, id));
                } else {
                    min_location = Some(id)
                }
            }
        }
    }
    println!("{min_location:?}");
}
