use std::collections::HashSet;

type Coord = (i32, i32);

fn find_trailhead_pos(topography_map: &[String]) -> Vec<Coord> {
    let mut trailheads = Vec::new();
    for (i, row) in topography_map.iter().enumerate() {
        for j in 0..row.len() {
            trailheads.push((i as i32, j as i32));
        }
    }
    trailheads
}

fn in_bounds(topography_map: &[String], pos: Coord) -> bool {
    let (x, y) = pos;
    x >= 0
        && y >= 0
        && x < topography_map.len() as i32
        && y < topography_map[x as usize].len() as i32
}

fn search_helper(
    topography_map: &[String],
    pos: Coord,
    expected_value: i32,
    mut seen: Option<&mut HashSet<Coord>>,
) -> usize {
    if !in_bounds(topography_map, pos) {
        return 0;
    }

    let (x, y) = pos;

    let current_char = topography_map[x as usize].chars().nth(y as usize).unwrap();
    let current_value = current_char.to_digit(10).unwrap() as i32;

    if current_value != expected_value {
        return 0;
    }

    if let Some(seen) = seen.as_deref() {
        if seen.contains(&pos) {
            return 0;
        }
    }

    if expected_value == 9 {
        if let Some(seen) = seen {
            seen.insert(pos);
        }
        return 1;
    }

    let next_value = expected_value + 1;

    let right = search_helper(topography_map, (x + 1, y), next_value, seen.as_deref_mut());
    let left = search_helper(topography_map, (x - 1, y), next_value, seen.as_deref_mut());
    let down = search_helper(topography_map, (x, y + 1), next_value, seen.as_deref_mut());
    let up = search_helper(topography_map, (x, y - 1), next_value, seen);

    right + left + down + up
}

fn search_trail(
    topography_map: &[String],
    pos: Coord,
    expected_value: i32,
    seen: &mut HashSet<Coord>,
) -> usize {
    search_helper(topography_map, pos, expected_value, Some(seen))
}

fn search_all_trails(topography_map: &[String], pos: Coord, expected_value: i32) -> usize {
    search_helper(topography_map, pos, expected_value, None)
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args().nth(1).unwrap_or("input.in".to_string());

    let file_content = utility::get_raw_text(&filename)?;
    let topography_map = utility::get_vec_from_txt(&file_content);

    let trailheads = find_trailhead_pos(&topography_map);

    let mut partial_trail_count = 0;
    let mut total_trail_count = 0;

    for starting_pos in trailheads {
        let mut seen = HashSet::new();

        let partial = search_trail(&topography_map, starting_pos, 0, &mut seen);
        let full_trails = search_all_trails(&topography_map, starting_pos, 0);

        partial_trail_count += partial;
        total_trail_count += full_trails;
    }

    println!("Partial trail count: {}", partial_trail_count);
    println!("Total trail count: {}", total_trail_count);

    Ok(())
}
