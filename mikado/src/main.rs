use std::collections::{HashMap, HashSet};

use parser::{ReplyInput, ReplyOutput};

fn main() {
    let input = ReplyInput::load(
        "./case.txt".into(),
        Box::new(|meta| {
            (
                meta[1] as usize,
                HashMap::from([("sticks".to_string(), meta[0])]),
            )
        }),
    );

    let output: Vec<_> = input
        .cases
        .into_iter()
        .map(|case| {
            // Map<ID, (Set<ID of above>, Set<ID of below>)>
            let mut sticks: HashMap<u32, (HashSet<u32>, HashSet<u32>)> = HashMap::from_iter(
                (0..*case.meta.get("sticks").unwrap() as u32)
                    .map(|i| (i, (HashSet::new(), HashSet::new()))),
            );

            for rule in case.data {
                let x = rule[1].parse().unwrap();
                let y = rule[0].parse().unwrap();
                sticks.get_mut(&y).unwrap().0.insert(x);
                sticks.get_mut(&x).unwrap().1.insert(y);
            }

            let mut order: Vec<u32> = Vec::with_capacity(sticks.len());

            while !sticks.is_empty() {
                let mut remove = 0;
                for (id, stick) in sticks.iter() {
                    if stick.0.is_empty() {
                        remove = *id;
                        break;
                    }
                }

                order.push(remove);
                sticks
                    .remove(&remove)
                    .unwrap()
                    .1
                    .clone()
                    .iter()
                    .for_each(|below| {
                        if let Some(below) = sticks.get_mut(below) {
                            below.0.remove(&remove);
                        }
                    });
            }

            order
                .iter()
                .map(u32::to_string)
                .rev()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    ReplyOutput(output).save("./output.txt".into())
}
