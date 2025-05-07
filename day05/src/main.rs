use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::Result;
use utility::{get_raw_text, get_vec_from_txt};

type Rules = HashMap<i64, Vec<i64>>;

fn is_correct_update(update_order: &[i64], priority_rules: &Rules) -> bool {
    let position: HashMap<i64, usize> = update_order
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    for (page, dependencies) in priority_rules {
        if !position.contains_key(page) {
            continue;
        }
        for dependency in &dependencies.clone() {
            if position.contains_key(dependency) && position[page] > position[dependency] {
                return false;
            }
        }
    }

    true
}

fn reorder_update(update_order: &[i64], priority_rules: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut graph: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut in_degree: HashMap<i64, usize> = HashMap::new();

    for &page in update_order {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    for (x, dependencies) in priority_rules {
        for &y in dependencies {
            if update_order.contains(x) && update_order.contains(&y) {
                graph.entry(*x).or_default().push(y);
                *in_degree.entry(y).or_insert(0) += 1;
            }
        }
    }

    let mut queue: VecDeque<i64> = VecDeque::new();
    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    // Topological sort using Kahn's algorithm
    let mut sorted_order: Vec<i64> = Vec::new();
    while let Some(page) = queue.pop_front() {
        sorted_order.push(page);

        if let Some(dependents) = graph.get(&page) {
            for &dependent in dependents {
                if let Some(degree) = in_degree.get_mut(&dependent) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dependent);
                    }
                }
            }
        }
    }

    if sorted_order.len() == update_order.len() {
        sorted_order
    } else {
        vec![] // Return an empty vector if the topological sort is incomplete
    }
}

fn main() -> Result<()> {
    let filename = env::args().nth(1).unwrap_or("input.txt".to_string());

    let txt = get_raw_text(&filename)?;
    let parts: Vec<&str> = txt.trim().split("\n\n").collect();

    let (rules, updates) = (parts[0], parts[1]);

    let mut priority_rules: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in get_vec_from_txt(rules) {
        let parts: Vec<i64> = line
            .split("|")
            .map(|s| s.trim().parse().expect("Failed to parse"))
            .collect();

        let (x, y) = (parts[0], parts[1]);

        priority_rules.entry(x).or_default().push(y);
    }

    let mut sum = 0;
    let mut sum2 = 0;
    for line in updates.lines() {
        let order: Vec<i64> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Failed to parse"))
            .collect();

        if !is_correct_update(&order, &priority_rules) {
            let new_order = reorder_update(&order, &priority_rules);
            sum2 += new_order[new_order.len() / 2];
            continue;
        }

        let mid = order.len() / 2;
        sum += order[mid];
    }

    println!("{}", sum);
    println!("{}", sum2);

    Ok(())
}
