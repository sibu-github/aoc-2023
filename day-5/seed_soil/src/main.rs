#[derive(Debug)]
struct Seed(u64);

#[derive(Debug)]
struct SeedToSoilMap {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug)]
struct SoilToFertilizerMap {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug)]
struct FertilizerToWaterMap {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug)]
struct WaterToLightMap {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug)]
struct LightToTempMap {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug)]
struct TempToHumidityMap {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug)]
struct HumidityToLocMap {
    dst: u64,
    src: u64,
    rng: u64,
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let mut lines = content.lines();
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|s| {
            let v = s.parse().unwrap();
            Seed(v)
        })
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let mut seed_to_soil_maps = vec![];
    if let Some("seed-to-soil map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = SeedToSoilMap { dst, src, rng };
            seed_to_soil_maps.push(m)
        }
    }
    let mut soil_to_fertilizer_maps = vec![];
    if let Some("soil-to-fertilizer map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = SoilToFertilizerMap { dst, src, rng };
            soil_to_fertilizer_maps.push(m)
        }
    }
    let mut fertilizer_to_water_maps = vec![];
    if let Some("fertilizer-to-water map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = FertilizerToWaterMap { dst, src, rng };
            fertilizer_to_water_maps.push(m)
        }
    }
    let mut water_to_light_maps = vec![];
    if let Some("water-to-light map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = WaterToLightMap { dst, src, rng };
            water_to_light_maps.push(m)
        }
    }
    let mut light_to_temp_maps = vec![];
    if let Some("light-to-temperature map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = LightToTempMap { dst, src, rng };
            light_to_temp_maps.push(m)
        }
    }
    let mut temp_to_humidity_maps = vec![];
    if let Some("temperature-to-humidity map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = TempToHumidityMap { dst, src, rng };
            temp_to_humidity_maps.push(m)
        }
    }
    let mut humidity_to_loc_maps = vec![];
    if let Some("humidity-to-location map:") = lines.next() {
        while let Some(s) = lines.next() {
            if s.is_empty() {
                break;
            }
            let (dst, rst) = s.split_once(' ').unwrap();
            let dst = dst.parse().unwrap();
            let (src, rng) = rst.trim().split_once(' ').unwrap();
            let src = src.parse().unwrap();
            let rng = rng.parse().unwrap();
            let m = HumidityToLocMap { dst, src, rng };
            humidity_to_loc_maps.push(m)
        }
    }
    let find_in_map = |dst: u64, src: u64, rng: u64, val: u64| -> Option<u64> {
        if val >= src && val <= src + rng - 1 {
            let diff = val - src;
            Some(dst + diff)
        } else {
            None
        }
    };
    let mut locs = vec![];
    for seed in seeds {
        let seed_no = seed.0;
        let mut soil = seed_no;
        for v in seed_to_soil_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, seed_no){
                soil = n;
                break;
            }
        }
        let mut fertilizer = soil;
        for v in soil_to_fertilizer_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, soil) {
                fertilizer = n;
                break;
            }
        }
        let mut water = fertilizer;
        for v in fertilizer_to_water_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, fertilizer) {
                water = n;
                break;
            }
        }
        let mut light = water;
        for v in water_to_light_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, water) {
                light = n;
                break;
            }
        }
        let mut temp = light;
        for v in light_to_temp_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, light) {
                temp = n;
                break;
            }
        }
        let mut humidity = temp;
        for v in temp_to_humidity_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, temp) {
                humidity = n;
                break;
            }
        }
        let mut loc = humidity;
        for v in humidity_to_loc_maps.iter() {
            if let Some(n) = find_in_map(v.dst, v.src, v.rng, humidity) {
                loc = n;
                break;
            }
        }
        locs.push(loc);
    }
    locs.sort_unstable();
    println!("{}", locs[0]);
}
