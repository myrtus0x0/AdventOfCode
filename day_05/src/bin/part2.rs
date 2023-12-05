use itertools::Itertools;

#[derive(Copy, Debug, Clone)]
struct Map {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

#[derive(Debug, Clone)]
struct GardenMap {
    translation_maps: Vec<Map>
}

fn parse_map(map_data:&str) -> GardenMap {
    let mut gard_map = GardenMap {
        translation_maps: Vec::new()
    };

    for (i, info) in map_data.split("\n").enumerate() {
        if i == 0 {
            continue;
        }

        let my_map = Map{
            dest_range_start: info.split(" ").nth(0).unwrap().parse::<u64>().unwrap(),
            src_range_start: info.split(" ").nth(1).unwrap().parse::<u64>().unwrap(),
            range_length: info.split(" ").nth(2).unwrap().parse::<u64>().unwrap(),
        };

        gard_map.translation_maps.push(my_map);
    }

    return gard_map
}

fn translate_map_to_seed(garden_map_slice: &[GardenMap]) -> Vec<u64> {
    let mut results = Vec::new();
    // we only care about the last map
    let focus_map = garden_map_slice.iter().nth(garden_map_slice.len() -1).unwrap();
    
    // for each later in the last map, we need to find the valid candidates
    for f_layer in &focus_map.translation_maps {
        
        // we only need to prcess the start and end of the src range
        let tries = vec![f_layer.src_range_start, f_layer.src_range_start+f_layer.range_length];
        for candidate in tries {
            let mut rev_traversal = candidate;
            
            // reverse traverse our map translating the src input to the initial seed
            for (_i, map) in garden_map_slice.iter().rev().enumerate() {
                for layer in &map.translation_maps {
                    if  rev_traversal >= layer.dest_range_start && rev_traversal < layer.dest_range_start + layer.range_length {
                        rev_traversal = layer.src_range_start + (rev_traversal - layer.dest_range_start);
                        break;
                    }
                }
            }
            
            if !results.contains(&rev_traversal) {
                results.push(rev_traversal);
            }
        }
    }

    return results;
}

fn reverse_traverse_maps(seed_info: Vec<u64>, maps:&Vec<GardenMap>) -> Vec<u64> {
    let mut res = Vec::new();
    for (i, _map) in maps.iter().enumerate() {
        // we can take the lowest value here in the translation layers and walk backwards in the maps to get a seed
        // if the seed is in our initial set then we are going to process it 
        let seeds = translate_map_to_seed(&maps[0..i+1]);
        for seed in seeds {
            for info in seed_info.chunks(2) {
                let start = *info.iter().nth(0).unwrap();
                let range = *info.iter().nth(1).unwrap();
                // if the seeds we generated from filtering are in the 
                // inital list of seeds then we append
                if seed >= start && seed <= start + range {
                    res.push(seed);
                } 
            }
        }      
    }

    return res;
}

fn traverse_maps(seed:u64, maps:&Vec<GardenMap>) -> u64 {
    let mut traversal: u64 = seed;

    for map in maps {        
        for layout_range in &map.translation_maps {
            if  traversal >= layout_range.src_range_start && 
                traversal < layout_range.src_range_start + layout_range.range_length {
                    traversal = layout_range.dest_range_start + (traversal - layout_range.src_range_start);
                    break
            }
        }
    }

    return traversal;
}

fn gather_seed_info(chunk:&str) -> Vec<u64> {
    let mut seeds = Vec::new();
    let seed_info = chunk.split(": ").nth(1).unwrap().split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    for mut temp_chunk in &seed_info.iter().chunks(2) {
        let start = temp_chunk.next().unwrap();
        let range = temp_chunk.next().unwrap();
        seeds.push(*start as u64);
        seeds.push(*range as u64);
    }

    return seeds;
}

fn part2(seed_map:&str) -> u64 {
    let mut maps: Vec<GardenMap> = Vec::new();
    let mut seed_info: Vec<u64> = Vec::new();
    for (i, chunk) in seed_map.split("\n\n").enumerate() {
        // parse out the seeds
        if i == 0 {
            seed_info = gather_seed_info(chunk);
        } else {
            maps.push(parse_map(chunk));
        }
    }

    let pre_filtered_seeds:u64 = seed_info.iter().step_by(2).sum();
    dbg!(pre_filtered_seeds);
    let filtered_seeds = reverse_traverse_maps(seed_info, &maps);
    dbg!(&filtered_seeds.len());

    let mut final_destinations: Vec<u64> = Vec::new();
    for seed in filtered_seeds {
        let traversal = traverse_maps(seed, &maps);
        final_destinations.push(traversal);
    }
    
    return *final_destinations.iter().min().unwrap();
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, 46);
        // assert_eq!(part2(include_str!("../puzzle")), 6472060);
    }
}