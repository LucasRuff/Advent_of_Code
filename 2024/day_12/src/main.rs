use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_12.txt") {
        let mut total_cost = 0;
        let mut prev_region = '~';
        let (total_cost_1, total_cost_2) = attempt_two(file_iter);
        println!("Total cost: {}", total_cost_1);
        println!("Part 2: {}", total_cost_2);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn attempt_two(ifile: io::Lines<io::BufReader<File>>) -> (usize, usize) {
    let mut fences: HashMap<(usize, usize), Fence> = HashMap::new();
    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut total_input: Vec<Vec<char>> = Vec::new();
    let mut total_cost = 0;
    let mut part_2_cost = 0;
    for line in ifile {
        let text = line.unwrap();
        let mut input_row_chars: Vec<char> = text.chars().collect();
        total_input.push(input_row_chars);
    }
    let g_cols = total_input[0].len();
    let g_rows = total_input.len();
    let mut frontier: Vec<(usize, usize)> = Vec::new();
    
    for i in 0..g_rows {
        for j in 0..g_cols {
            if visited_locations.contains(&(i,j)) {
                continue;
            }
            frontier.push((i,j));
            let mut area = 0;
            let mut perimeter = 0;
            let region_id = total_input[i][j];
            fences.clear();
            loop {
                match frontier.pop() {
                    Some((f_i, f_j)) => {
                        if visited_locations.contains(&(f_i,f_j)) {
                            continue;
                        }
                        area += 1;
                        visited_locations.insert((f_i, f_j));
                        //look up
                        if f_i.wrapping_sub(1) < g_rows {
                            if total_input[f_i - 1][f_j] != region_id {
                                fences.entry((f_i, f_j)).and_modify(|f| f.up = true).or_insert(Fence{up: true, down: false, left: false, right: false});
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i - 1, f_j)) {
                                    frontier.push((f_i - 1, f_j));
                                }
                            }
                        } else {
                            fences.entry((f_i, f_j)).and_modify(|f| f.up = true).or_insert(Fence{up: true, down: false, left: false, right: false});
                            perimeter += 1;
                        }
                        //look down
                        if f_i + 1 < g_rows {
                            if total_input[f_i + 1][f_j] != region_id {
                                fences.entry((f_i, f_j)).and_modify(|f| f.down = true).or_insert(Fence{up: false, down: true, left: false, right: false});
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i + 1, f_j)) {
                                    frontier.push((f_i + 1, f_j));
                                }
                                
                            }
                        } else {
                            fences.entry((f_i, f_j)).and_modify(|f| f.down = true).or_insert(Fence{up: false, down: true, left: false, right: false});
                            perimeter += 1;
                        }
                        //look right
                        if f_j + 1 < g_cols {
                            if total_input[f_i][f_j + 1] != region_id {
                                fences.entry((f_i, f_j)).and_modify(|f| f.right = true).or_insert(Fence{up: false, down: false, left: false, right: true});
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i, f_j + 1)) {
                                    frontier.push((f_i, f_j + 1));
                                }
                            }
                        } else {
                            fences.entry((f_i, f_j)).and_modify(|f| f.right = true).or_insert(Fence{up: false, down: false, left: false, right: true});
                            perimeter += 1;
                        }
                        //look left
                        if f_j.wrapping_sub(1) < g_cols {
                            if total_input[f_i][f_j - 1] != region_id {
                                fences.entry((f_i, f_j)).and_modify(|f| f.left = true).or_insert(Fence{up: false, down: false, left: true, right: false});
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i, f_j - 1)) {
                                    frontier.push((f_i, f_j - 1));
                                }
                            }
                        } else {
                            fences.entry((f_i, f_j)).and_modify(|f| f.left = true).or_insert(Fence{up: false, down: false, left: true, right: false});
                            perimeter += 1;
                        }
                        
                    },
                    None => break,
                }
            }
            let part_2_perimeter = calculate_true_perimter(&fences, g_rows, g_cols);
            //println!("Out of nodes in this garden: a:{}, p:{}, c:{}", area, perimeter, region_id);
            total_cost += area * perimeter;
            let part_cost = area * part_2_perimeter;
            //println!("region {} has cost {}", region_id, part_cost);
            part_2_cost += area * part_2_perimeter;
        }
    }
    return (total_cost, part_2_cost);
}

fn calculate_true_perimter(fence_map: &HashMap<(usize, usize), Fence>, g_rows: usize, g_cols: usize) -> usize {
    let mut fused_fences = 0;
    for i in 0..g_rows {
        for j in 0..g_cols {
            match fence_map.get(&(i,j)) {
                Some(f_1) => {
                    //look up
                    match fence_map.get(&(i.wrapping_sub(1), j)) {
                        Some(f_2) => {
                            match (f_1.left, f_2.left, f_1.right, f_2.right) {
                                (false, _, false, _) | (_, true, _, true) | (false, _, _, true) | (_, true, false, _) => {},
                                (true, false, true, true) | (true, true, true, false) | (true, false, false, _) | (false, _, true, false) => fused_fences += 1,
                                (true, false, true, false) => fused_fences += 2,
                            }
                        },
                        None => {
                            fused_fences += match (f_1.left, f_1.right) {
                                (true, true) => 2,
                                (true, false) | (false, true) => 1,
                                (false, false) => 0,
                            };
                        },
                    }
                    //look left
                    match fence_map.get(&(i, j.wrapping_sub(1))) {
                        Some(f_2) => {
                            match (f_1.up, f_2.up, f_1.down, f_2.down) {
                                (false, _, false, _) | (_, true, _, true) | (false, _, _, true) | (_, true, false, _) => {},
                                (true, false, true, true) | (true, true, true, false) | (true, false, false, _) | (false, _, true, false) => fused_fences += 1,
                                (true, false, true, false) => fused_fences += 2,
                            }
                        },
                        None => {
                            fused_fences += match (f_1.up, f_1.down) {
                                (true, true) => 2,
                                (true, false) | (false, true) => 1,
                                (false, false) => 0,
                            };
                        },
                    }
                    
                },
                None => continue,
            }
        }
    }
    return fused_fences;
}

/* THIS CODE PASSES TESTS BUT NOT ACTUAL INPUT
fn get_regions(map: io::Lines<io::BufReader<File>>) -> Vec<Region> {
    let mut region_candidates_vec: Vec<Vec<(char, usize, usize, usize, bool, bool)>> = Vec::new();
    let mut region_number = 0;
    for line in map {
        let text = line.unwrap();
        let mut line_regions: Vec<(char, usize, usize, usize, bool, bool)> = Vec::new();
        let mut char_iter = text.chars();
        let mut current_cha = char_iter.next().unwrap();
        let mut start_cha = 0;
        let mut line_length = 1;
        'readline: for (i,cha) in char_iter.enumerate() {
            line_length += 1;
            while cha == current_cha {
                current_cha = cha;
                continue 'readline;
            }
            line_regions.push((current_cha, region_number, start_cha, i, true, true));
            current_cha = cha;
            start_cha = i+1;
            region_number += 1;
        }
        line_regions.push((current_cha, region_number, start_cha, line_length - 1, true, true));
        region_candidates_vec.push(line_regions);
    }
    //let refined_region_candidates = north_to_south(region_candidates_vec);
    //let better_regions = south_to_north(refined_region_candidates);
    let mut changes_made = true;
    while(changes_made) {
        let mut temp_reg;
        (temp_reg, changes_made) = south_to_north(north_to_south(region_candidates_vec));
        region_candidates_vec = temp_reg;
    }
    let final_regions = collapse_regions(region_candidates_vec);
    return final_regions;
}

fn north_to_south(uncollapsed_regions: Vec<Vec<(char, usize, usize, usize, bool, bool)>>) -> Vec<Vec<(char, usize, usize, usize, bool, bool)>> {
    let mut region_template = uncollapsed_regions.clone();
    for i in 0..uncollapsed_regions.len()-1 {
        let row_a = region_template[i].clone();
        let row_b = region_template[i+1].clone();
        for region_a in &row_a {
            for (j, region_b) in row_b.iter().enumerate() {
                if region_a.0 == region_b.0 && ((region_a.2 <= region_b.3 && region_a.3 >= region_b.2) || (region_b.2 <= region_a.3 && region_b.3 >= region_a.2)) {
                    region_template[i+1][j].1 = region_a.1;
                    region_template[i+1][j].4 = false;
                }
            }
        }
    }
    return region_template;
}

fn south_to_north(uncollapsed_regions: Vec<Vec<(char, usize, usize, usize, bool, bool)>>) -> (Vec<Vec<(char, usize, usize, usize, bool, bool)>>, bool) {
    let mut region_template = uncollapsed_regions.clone();
    let num_rows = uncollapsed_regions.len();
    let mut changes_made = false;
    for i in 2..num_rows + 1 {
        let row_a = region_template[num_rows - i].clone();
        //println!("Row a is {:?}", row_a);
        let row_b = region_template[num_rows - i + 1].clone();
        //println!("Row b is {:?}", row_b);
        for (j, region_a) in row_a.iter().enumerate() {
            for region_b in &row_b {
                if region_a.0 == region_b.0 && ((region_a.2 <= region_b.3 && region_a.3 >= region_b.2) || (region_b.2 <= region_a.3 && region_b.3 >= region_a.2)) {
                    if region_a.1 != region_b.1 {
                        region_template[num_rows - i][j].1 = region_b.1;
                        changes_made = true;
                    }
                    
                    region_template[num_rows - i][j].5 = false;
                    
                }
            }
        }
    }
    return (region_template, changes_made);
}

fn collapse_regions(uncollapsed_regions: Vec<Vec<(char, usize, usize, usize, bool, bool)>>) -> Vec<Region> {
    let mut region_vec: Vec<Region> = Vec::new();
    for (i, row) in uncollapsed_regions.iter().enumerate() {
        'region_portions: for region_portion in row {
            for region_candidate in &mut region_vec {
                if region_candidate.id == region_portion.1 {
                    for try_row in &mut region_candidate.rows {
                        if try_row[0].row == i {
                            try_row.push(RowPortion{row: i, start: region_portion.2, end: region_portion.3, is_top: region_portion.4, is_bottom: region_portion.5});
                            continue 'region_portions;
                        }
                    }
                    region_candidate.rows.push(vec![RowPortion{row: i, start: region_portion.2, end: region_portion.3, is_top: region_portion.4, is_bottom: region_portion.5}]);
                    continue 'region_portions;
                }
            }
            region_vec.push(Region{cha: region_portion.0, id: region_portion.1, rows: vec![vec![RowPortion{row: i, start: region_portion.2, end: region_portion.3, is_top: region_portion.4, is_bottom: region_portion.5}]]});
        }
    }
    return region_vec;
}

fn calculate_cost(region: Region) -> usize {
    let area = get_area(&region);
    let perimeter = get_perimeter(&region);
    let total = area * perimeter;
    if total < 4 {
        println!("Small region found at {}, {}, cha: {}", region.rows[0][0].row, region.rows[0][0].start, region.cha);
    }
    return area * perimeter;
}

fn get_area(region: &Region) -> usize {
    let mut total = 0;
    for row in &region.rows {
        for row_portion in row {
            total += row_portion.end + 1 - row_portion.start;
        }
    }
    return total;
}

fn get_perimeter(region: &Region) -> usize {
    let total = horizontal_fences(region) + vertical_fences(region);
    return total;
}

fn horizontal_fences(region: &Region) -> usize {
    let mut fences = 0;
    for i in 0..region.rows.len()-1 { // iterate through each Vec of Vec<RowPortion>
        let top_row = &region.rows[i];
        let bottom_row = &region.rows[i + 1];
        let mut total_overlap = 0;
        let mut overlapping_top_total = 0;
        let mut overlapping_bot_total = 0;
        for row_portion in top_row {
            if !row_portion.is_bottom {
                overlapping_top_total += row_portion.end + 1 - row_portion.start;
            }
        }
        for row_portion in bottom_row {
            if !row_portion.is_top {
                overlapping_bot_total += row_portion.end + 1 - row_portion.start;
            }
        }
        for top_portion in top_row {
            for bottom_portion in bottom_row {
                if does_overlap(top_portion, bottom_portion) {
                    total_overlap += get_overlap(top_portion, bottom_portion);
                    //perimeter += (top_portion.end + 1 - top_portion.start) + (bottom_portion.end + 1 - bottom_portion.start) - (2 * portion_overlap);
                }
            }
        }
        fences += overlapping_top_total + overlapping_bot_total - 2 * total_overlap;
    }
    for i in 0..region.rows.len() {
        for portion in &region.rows[i] {
            if portion.is_bottom {
                fences += portion.end + 1 - portion.start;
            }
            if portion.is_top {
                fences += portion.end + 1 - portion.start;
            }
        }
    }
    return fences;
}

fn vertical_fences(region: &Region) -> usize {
    let mut fences = 0;
    for row in &region.rows {
        for _ in 0..row.len() {
            fences += 2;
        }
    }
    return fences;
}

fn does_overlap(row_a: &RowPortion, row_b: &RowPortion) -> bool {
    return (row_a.start <= row_b.end && row_a.end >= row_b.start) || (row_b.start <= row_a.end && row_b.end >= row_a.start);
}

fn get_overlap(top: &RowPortion, bottom: &RowPortion) -> usize {
    match (top.start <= bottom.start, top.end <= bottom.end) {
        (true, true) => return top.end - bottom.start + 1,
        (true, false) => return bottom.end - bottom.start + 1,
        (false, true) => return top.end - top.start + 1,
        (false, false) => return bottom.end - top.start + 1,
    }
}
*/


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Clone)]
struct Fence {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
/*
#[derive(Debug, PartialEq, Clone)]
struct Region {
    cha: char,
    id: usize,
    rows: Vec<Vec<RowPortion>>, // row, start_char, end_char, is_top, is_bottom
}

#[derive(Debug, PartialEq, Clone)]
struct RowPortion {
    row: usize,
    start: usize,
    end: usize,
    is_top: bool,
    is_bottom: bool,
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn attempt_two_test() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(attempt_two(file_iter), (1930, 1206));
        }
    }
    #[test]
    fn attempt_two_part_2() {
        if let Ok(file_iter) = read_lines("easy_test.txt") {
            assert_eq!(attempt_two(file_iter).1, 368);
        }
    }
}
