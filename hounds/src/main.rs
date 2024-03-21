use std::collections::HashMap;

use parser::{ReplyInput, ReplyOutput};

fn find_combinations(target: u32, numbers: &[u32]) -> Vec<Vec<u32>> {
    let mut combinations = Vec::new();
    let mut current_combination = Vec::new();

    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();

    backtrack(
        &mut combinations,
        &mut current_combination,
        &sorted_numbers,
        target,
        0,
    );

    combinations
}

fn backtrack(
    combinations: &mut Vec<Vec<u32>>,
    current_combination: &mut Vec<u32>,
    numbers: &[u32],
    remaining_target: u32,
    start_index: usize,
) {
    if remaining_target == 0 {
        combinations.push(current_combination.clone());
        return;
    }

    for i in start_index..numbers.len() {
        let num = numbers[i];
        if num > remaining_target {
            break;
        }

        current_combination.push(num);
        backtrack(
            combinations,
            current_combination,
            numbers,
            remaining_target - num,
            i + 1, // Increase the start_index to avoid reusing the same number
        );
        current_combination.pop();
    }
}

fn find_combo(
    days_min: u32,
    days_max: u32,
    mia: Vec<u32>,
    genga: Vec<u32>,
    w: u32,
) -> Option<Vec<(Vec<u32>, Vec<u32>)>> {
    let sum: u32 = mia.iter().sum();
    let max_possible = days_max * w;

    if sum > max_possible {
        return None;
    }

    for try_sum in (1..w + 1).rev() {
        let mia_picks = find_combinations(try_sum, &mia);
        let genga_picks = find_combinations(try_sum, &genga);

        if mia_picks.is_empty() {
            continue;
        }

        let mut possible_picks = Vec::with_capacity(mia_picks.len() * genga_picks.len());
        for mia_pick in &mia_picks {
            for genga_picks in &genga_picks {
                possible_picks.push((mia_pick, genga_picks));
            }
        }

        for (mia_pick, genga_pick) in possible_picks {
            let mut mia = mia.clone();
            mia_pick.iter().for_each(|n| {
                mia.remove(mia.iter().position(|item| item == n).unwrap());
            });
            let mut genga = genga.clone();
            genga_pick.iter().for_each(|n| {
                genga.remove(genga.iter().position(|item| item == n).unwrap());
            });

            if mia.is_empty() {
                if days_min != 0 {
                    continue;
                }
                return Some(vec![(mia_pick.clone(), genga_pick.clone())]);
            }

            if let Some(mut after) = find_combo(
                days_min.saturating_sub(1),
                days_max.saturating_sub(1),
                mia,
                genga,
                w,
            ) {
                let total_days = after.len() as u32 + 1;
                if total_days < days_min || total_days > days_max {
                    continue;
                }

                after.push((mia_pick.clone(), genga_pick.clone()));
                return Some(after);
            }
        }
    }

    return None;
}

fn main() {
    let input = ReplyInput::load(
        "./case.txt".into(),
        Box::new(|meta| {
            (
                2,
                HashMap::from([
                    ("a_min".to_string(), meta[0]),
                    ("a_max".to_string(), meta[2]),
                    ("l_mia".to_string(), meta[2]),
                    ("l_genga".to_string(), meta[3]),
                    ("w".to_string(), meta[4]),
                ]),
            )
        }),
    );

    let res = input
        .cases
        .into_iter()
        .map(|case| {
            let mia: Vec<u32> = case.data[0]
                .iter()
                .map(|cell| cell.parse().unwrap())
                .collect::<Vec<_>>();

            let genga: Vec<u32> = case.data[1]
                .iter()
                .map(|cell| cell.parse().unwrap())
                .collect::<Vec<_>>();

            let combo = find_combo(
                *case.meta.get("a_min").unwrap() as u32,
                *case.meta.get("a_max").unwrap() as u32,
                mia,
                genga,
                *case.meta.get("w").unwrap() as u32,
            )
            .unwrap();

            let days = combo.len();

            let combo = combo
                .into_iter()
                .map(|(mia, mut genga)| {
                    let mut vec = mia;
                    vec.push(0);
                    vec.append(&mut genga);
                    vec.push(0);
                    vec.into_iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>();

            format!("{days}\n{}", combo.join("\n"))
        })
        .collect::<Vec<_>>();

    ReplyOutput(res).save("./output.txt".into())
}
