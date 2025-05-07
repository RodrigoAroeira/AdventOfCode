use utility::{get_raw_text, get_vec_from_txt};

fn transpose_matrix(lines: &[String]) -> Vec<String> {
    let mut transposed: Vec<String> = Vec::new();

    if let Some(first_line) = lines.first() {
        for _ in 0..first_line.len() {
            transposed.push(String::new());
        }
        for line in lines {
            for (i, c) in line.chars().enumerate() {
                transposed[i].push(c);
            }
        }
    }

    transposed
}

fn get_diagonals_lr(lines: &[String]) -> Vec<String> {
    let mut diagonals = Vec::new();
    let rows = lines.len();

    for start in 0..rows {
        let mut diagonal = Vec::new();
        let (mut row, mut col) = (start, 0);

        while row < rows && col < lines[row].len() {
            let c = lines[row].chars().nth(col).unwrap();
            diagonal.push(c.to_string());
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal.concat());
    }

    let max = lines.iter().map(|row| row.len()).max().unwrap();
    for start in 1..max {
        let mut diagonal = Vec::new();
        let (mut row, mut col) = (0, start);

        while row < rows && col < lines[row].len() {
            let c = lines[row].chars().nth(col).unwrap();
            diagonal.push(c.to_string());
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal.concat());
    }

    diagonals
}

// TODO: Find out why not working as intended, like python
fn _get_diagonals_rl(lines: &[String]) -> Vec<String> {
    let mut diagonals = Vec::new();
    let rows = lines.len();

    for start in 0..rows {
        let mut diagonal = Vec::new();
        let mut row = start;
        let mut col = (lines[start].len() as i64) - 1;

        while row < rows && col >= 0 {
            let c = lines[row].chars().nth(col as usize).unwrap();
            diagonal.push(c.to_string());
            row += 1;
            col -= 1;
        }
        diagonals.push(diagonal.into_iter().collect());
    }

    let max = lines.iter().map(|row| row.len()).max().unwrap();
    for start in (0..max - 2).rev() {
        let mut diagonal = Vec::new();
        let (mut row, mut col): (usize, i64) = (0, start as i64);

        while row < rows && col >= 0 && col < lines[row].len() as i64 {
            let c = lines[row].chars().nth(col as usize).unwrap();
            diagonal.push(c.to_string());
            row += 1;
            col -= 1;
        }
        diagonals.push(diagonal.into_iter().collect());
    }

    diagonals
}

fn get_diagonals_rl(lines: &[String]) -> Vec<String> {
    let mut diagonals = Vec::new();
    let rows = lines.len();

    // Top-right to bottom-left diagonals
    for start in 0..rows {
        let mut diagonal = Vec::new();
        let mut row = start;
        let mut col = lines[start].len().saturating_sub(1);

        while row < rows && col < lines[row].len() {
            if let Some(c) = lines[row].chars().nth(col) {
                diagonal.push(c);
            }
            row += 1;
            if col == 0 {
                break;
            }

            col -= 1;
        }
        diagonals.push(diagonal.into_iter().collect());
    }

    let max = lines.iter().map(|row| row.len()).max().unwrap_or(0);
    for start in (0..max.saturating_sub(1)).rev() {
        let mut diagonal = Vec::new();
        let (mut row, mut col) = (0, start);

        while row < rows && col < lines[row].len() {
            if let Some(c) = lines[row].chars().nth(col) {
                diagonal.push(c);
            }
            row += 1;
            if col > 0 {
                col -= 1;
            } else {
                break;
            }
        }
        diagonals.push(diagonal.into_iter().collect());
    }

    diagonals
}

fn find_xmas_by_line(line: &str) -> usize {
    line.matches("XMAS").count()
}

fn find_xmas_by_line_and_reverse(line: &str) -> usize {
    let mut counter = 0;
    counter += find_xmas_by_line(line);

    let reversed: String = line.chars().rev().collect();
    counter += find_xmas_by_line(&reversed);

    counter
}

fn find_xmas_in_lines(lines: &[String]) -> usize {
    let mut counter = 0;
    for line in lines {
        counter += find_xmas_by_line_and_reverse(line);
    }

    counter
}

fn is_center_of_3x3(pos: (usize, usize), file_lines: &[String]) -> bool {
    let (x, y) = (pos.0 as i64, pos.1 as i64);

    if x - 1 < 0
        || x + 1 >= file_lines.len().try_into().unwrap()
        || y - 1 < 0
        || y + 1 >= file_lines[0].len().try_into().unwrap()
    {
        return false;
    }
    true
}

fn main() {
    let txt = get_raw_text("input.txt").unwrap();
    let lines = get_vec_from_txt(&txt);

    let transposed = transpose_matrix(&lines);
    let lr_diag = get_diagonals_lr(&lines);
    let rl_diag = get_diagonals_rl(&lines);

    let mut counter = 0;
    counter += find_xmas_in_lines(&lines);
    counter += find_xmas_in_lines(&transposed);
    counter += find_xmas_in_lines(&lr_diag);
    counter += find_xmas_in_lines(&rl_diag);

    println!("{}", counter);

    let mut counter2 = 0;
    for (i, line) in lines.iter().enumerate() {
        for j in 0..line.len() {
            if !is_center_of_3x3((i, j), &lines) {
                continue;
            }
            if !line.chars().nth(j).unwrap().eq_ignore_ascii_case(&'A') {
                continue;
            }

            let diag1 = format!(
                "{}{}{}",
                lines[i - 1].chars().nth(j - 1).unwrap(),
                lines[i].chars().nth(j).unwrap(),
                lines[i + 1].chars().nth(j + 1).unwrap()
            )
            .to_uppercase();
            let diag2 = format!(
                "{}{}{}",
                lines[i - 1].chars().nth(j + 1).unwrap(),
                lines[i].chars().nth(j).unwrap(),
                lines[i + 1].chars().nth(j - 1).unwrap()
            )
            .to_uppercase();

            let diag1_rev = diag1.chars().rev().collect::<String>();
            let diag2_rev = diag2.chars().rev().collect::<String>();
            if (diag1 == "MAS" || diag1_rev == "MAS") && (diag2 == "MAS" || diag2_rev == "MAS") {
                counter2 += 1;
            }
        }
    }

    println!("{}", counter2);
}
