use coord::Coord;
use itertools::Itertools;
use nalgebra::{Matrix3, Matrix4, RowVector4, Vector3, Vector4};
use std::collections::{BTreeMap, HashSet};

mod coord; // was done for learning purposes, better to use nalgebra

type ScannerID = usize;
type ScannerMap = BTreeMap<ScannerID, HashSet<Coord>>; // ScannerID -> Set of Coordinates for their beacons
type Distance = i64; // distance between two coordinates
type LocalDistanceMap = BTreeMap<(Coord, Coord), Distance>; // (beacon_a, beacon_b) -> Distance.

trait AsCoord {
    fn as_coord(self) -> Coord;
}
impl AsCoord for Vector3<f64> {
    fn as_coord(self) -> Coord {
        Coord::new(self.x as i64, self.y as i64, self.z as i64)
    }
}
impl AsCoord for Vector4<f64> {
    fn as_coord(self) -> Coord {
        Coord::new(self.x as i64, self.y as i64, self.z as i64)
    }
}
impl AsCoord for RowVector4<f64> {
    fn as_coord(self) -> Coord {
        Coord::new(self.x as i64, self.y as i64, self.z as i64)
    }
}


pub fn main() {
    let input = include_str!("../input/input");
    // let input = include_str!("../input/input_2d_test");
    // let input = include_str!("../input/input_test");
    let mut scanners = parse_input(input);
    
    let mut distance_map: BTreeMap<ScannerID, LocalDistanceMap> = BTreeMap::new();
    for (id, beacons) in scanners.iter() {
        let sc0_beacons_distances = create_distance_map(beacons);
        distance_map.insert(*id, sc0_beacons_distances);
        }

    let mut transformation_map: Vec<(ScannerID, Vector3<f64>, Matrix3<f64>)> = Vec::new();
    let mut scanner_ids: Vec<usize> = scanners.keys()
                                        .skip(1)
                                        .map(|x| *x)
                                        .collect();

    scanner_ids.reverse(); // is most of the time a good idea ;)
    while scanner_ids.len() > 0 {
        let id = scanner_ids.pop().unwrap();
        let scan_matches = compare_scanner_ids(id, &distance_map);
        
        if scan_matches.len() >= 4 {
            // 4 is the minimum number of matches to be able to calculate a transformation
            let (offset, rot_matrix) = get_pos_and_rot_matrix(&scan_matches);
            transformation_map.push((id, offset, rot_matrix));
            // let result_coord = transform_coord(&test_coord1, &offset, &rot_matrix);
            let l_dmap = distance_map[&id].clone();
            let zero_dmap = distance_map.get_mut(&0).unwrap();
            for ((pos1, pos2), dist12) in l_dmap.iter() {
                let new_pos1 = transform_coord(pos1, &offset, &rot_matrix);
                let new_pos2 = transform_coord(pos2, &offset, &rot_matrix);
                let new_pos1 = new_pos1.as_coord();
                let new_pos2 = new_pos2.as_coord();
                zero_dmap.insert((new_pos1, new_pos2), *dist12);
                
                //add scanners[&id] beacons to scanners[0] if they are not already there
                let zero_beacons = scanners.get_mut(&0).unwrap();
                zero_beacons.insert(new_pos1);
                zero_beacons.insert(new_pos2);
            }
        }
        else {
            // current id has not enough matches to be able to calculate a transformation
            // so we add it to the beginning of the list
            let mut id_vc = vec![id];
            id_vc.append(&mut scanner_ids);
            scanner_ids = id_vc;
        }
    }

    for &(id, offset, rot_matrix) in transformation_map.iter() {
        println!(" ID: {} \n Scanner_Pos: {} Scanner_Rotation: {}", id, offset, rot_matrix);
    }


    let zero_beacons = scanners.get_mut(&0).unwrap();
    println!("Found {} beacons", zero_beacons.len());
   
    // lets do part 2:
    // need to find the largest manhattan distance between two offsets (can get this from the transformation map)

    let mut scanner_pos: HashSet<Coord> = HashSet::new();
    for &(_id, offset, _rot_matrix) in transformation_map.iter() {
        scanner_pos.insert(offset.as_coord());
    }
    let scanner_pos_distances = create_distance_map_manhattan(&scanner_pos);
    // get largest value from scanner_pos_distances
    let mut max_distance = 0;
    for (_, dist) in scanner_pos_distances.iter() {
        if *dist > max_distance {
            max_distance = *dist;
        }
    }
    println!("Largest distance between two offsets: {}", max_distance);

}

pub fn transform_coord(
    coord: &Coord,
    offset: &Vector3<f64>,
    rot_matrix: &Matrix3<f64>,
) -> Vector3<f64> {
    let mut transformed_coord = Vector3::new(coord.x as f64, coord.y as f64, coord.z as f64);
    transformed_coord = rot_matrix * transformed_coord;
    transformed_coord += offset;
    transformed_coord
}

pub fn get_pos_and_rot_matrix(
    scan_matches: &Vec<(Coord, Coord, i64)>,
) -> (Vector3<f64>, Matrix3<f64>) {
    // only need 4 shared coordinates to determine position and rotation of the scanner.
    let mut scanner_1_pos = Vector3::zeros();
    let mut rot_matrix = Matrix3::zeros();
    let mut m = Matrix4::zeros();
    let mut vx = Vector4::zeros();
    let mut vy = Vector4::zeros();
    let mut vz = Vector4::zeros();

    for i in 0..4 {
        // each row which (i) represents one coordinate match from scan_matches
        let c1 = scan_matches[i].0.into_vector();
        let c2 = scan_matches[i].1;
        m.set_row(
            i,
            &RowVector4::new(c2.x as f64, c2.y as f64, c2.z as f64, 1.0),
        );
        vx[i] = c1[0];
        vy[i] = c1[1];
        vz[i] = c1[2];
    }

    let decomp = m.lu();
    if let (Some(sol_x), Some(sol_y), Some(sol_z)) =
        (decomp.solve(&vx), decomp.solve(&vy), decomp.solve(&vz))
    {
        let dx = sol_x[3].round();
        let dy = sol_y[3].round();
        let dz = sol_z[3].round();
        scanner_1_pos = Vector3::new(dx, dy, dz);
        rot_matrix = Matrix3::new(
            sol_x[0], sol_x[1], sol_x[2], sol_y[0], sol_y[1], sol_y[2], sol_z[0], sol_z[1],
            sol_z[2],
        );
    }
    // lets round all values within rot_matrix to 3 decimal places
    rot_matrix = rot_matrix.map(|x| (1000.0 * x).round() / 1000.0);
    (scanner_1_pos, rot_matrix)
}

pub fn compare_scanner_ids(
    id_b: ScannerID,
    distance_map: &BTreeMap<ScannerID, LocalDistanceMap>,
) -> Vec<(Coord, Coord, i64)> {
    let sc0_beacons_distances = distance_map.get(&0).unwrap();
    let sc1_beacons_distances = distance_map.get(&id_b).unwrap();
    let match_id_map = create_match_map(sc0_beacons_distances, &sc1_beacons_distances);
    // match_id_map finds possible beacons which could be in the same location
    let filtered_matches = filter_matches(&match_id_map);
    // filtered_matches finds the beacons which are actually in the same location, with a value
    // indicating the number of matches found.
    filtered_matches
}


pub fn create_match_map(
    sc0_beacons_distances: &LocalDistanceMap,
    sc1_beacons_distances: &LocalDistanceMap,
) -> BTreeMap<Coord, BTreeMap<Coord, i64>> {
    let mut id_match_map: BTreeMap<Coord, BTreeMap<Coord, i64>> = BTreeMap::new();
    // id_match_map counts which coord it matches with most.
    for ((ba, bb), val) in sc0_beacons_distances {
        if sc1_beacons_distances.values().contains(&val) {
            // now we need to check if for the same beacon location, other distances match or
            // beacons match
            // works as long as there is only one match, if it fails we should skip item.
            let (bc, bd) = sc1_beacons_distances
                .iter()
                .find(|&((_, _), val_c)| val_c == val)
                .unwrap()
                .0;

            let key1 = id_match_map.entry(*ba).or_insert_with(BTreeMap::new);
            let count = key1.entry(*bd).or_insert(0);
            *count += 1;
            let count = key1.entry(*bc).or_insert(0);
            *count += 1;

            let key2 = id_match_map.entry(*bb).or_insert_with(BTreeMap::new);
            let count = key2.entry(*bd).or_insert(0);
            *count += 1;
            let count = key2.entry(*bc).or_insert(0);
            *count += 1;
        }
    }
    id_match_map
}

pub fn filter_matches(
    id_match_map: &BTreeMap<Coord, BTreeMap<Coord, i64>>,
) -> Vec<(Coord, Coord, i64)> {
    let mut match_map: Vec<(Coord, Coord, i64)> = Vec::new();
    for (id, ids) in id_match_map {
        // which ids key has the largest value?
        let mut max_count = 0;
        let mut max_id = Coord { x: 0, y: 0, z: 0 };
        for (&id_matched, &count) in ids {
            if count > max_count {
                max_count = count;
                max_id = id_matched;
            }
        }
        if max_count > 4 {
            match_map.push((*id, max_id, max_count));
        }
    }
    match_map
}

pub fn create_distance_map_manhattan(beacon_map: &HashSet<Coord>) -> LocalDistanceMap {
    // input: HashSet of beacon coordinates
    // output: HashMap of distances between beacon coordinates
    let mut distance_map = LocalDistanceMap::new();
    for (i, coord_a) in beacon_map.iter().enumerate() {
        for coord_b in beacon_map.iter().skip(i + 1) {
            let distance = coord_a.distance_manhattan(coord_b);
            distance_map.insert((*coord_a, *coord_b), distance);
            // distance_map.insert((*coord_b, *coord_a), distance);
        }
    }
    distance_map
}

pub fn create_distance_map(beacon_map: &HashSet<Coord>) -> LocalDistanceMap {
    // input: HashSet of beacon coordinates
    // output: HashMap of distances between beacon coordinates
    let mut distance_map = LocalDistanceMap::new();
    for (i, coord_a) in beacon_map.iter().enumerate() {
        for coord_b in beacon_map.iter().skip(i + 1) {
            let distance = coord_a.distance(coord_b);
            distance_map.insert((*coord_a, *coord_b), distance);
            // distance_map.insert((*coord_b, *coord_a), distance);
        }
    }
    distance_map
}

pub fn parse_input(input: &str) -> ScannerMap {
    // the input contains coordinates for multiple scanners.
    // after a line containing "--- scanner 0 ---", the next line contains the coordinates of the scanner.
    // the coordinates are separated by a comma.
    // the coordinates are in the form "x,y,z"
    let mut scanners: ScannerMap = BTreeMap::new();
    let mut scanner_id: usize = 0;
    let _: Vec<usize> = input
        .lines()
        .map(|line| {
            if line.contains("scanner") {
                // parse the scanner id from the line
                // example line: "--- scanner 12 ---"
                // scanner id is 12
                // we need to parse the number after the "---"
                let scanner_id_str = line.split_whitespace().nth(2).unwrap();
                scanner_id = scanner_id_str.parse::<usize>().unwrap();
            } else if line.is_empty() { /*do nothing*/
            } else {
                let scanner_coords: Vec<&str> = line.split(',').collect();
                let x: i64 = scanner_coords[0].parse().unwrap();
                let y: i64 = scanner_coords[1].parse().unwrap();
                let z: i64 = scanner_coords[2].parse().unwrap();
                let coord = Coord { x, y, z };
                scanners
                    .entry(scanner_id)
                    .or_insert_with(HashSet::new)
                    .insert(coord);
            }
            scanner_id
        })
        .collect();
    scanners
}

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn test_subtract_offset() {
        let input = include_str!("../input/input_2d_test");
        let (scanners, _ids) = super::parse_input(input);
        let mut scanner_map = scanners.clone();
        let scanner_map_0 = scanners.get(&0).unwrap().clone();
        // super::offset_map(super::Coord { x: 0, y: 0, z: 1 }, &mut scanner_map_0);
        scanner_map.insert(0, scanner_map_0);
        dbg!(scanner_map);
        assert_eq!(0, 0);
    } */
}
