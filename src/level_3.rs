use itertools::Itertools;
use std::collections::HashMap;
use std::fs::canonicalize;
use std::iter::Map;

pub fn level_3(content: String) -> String {
    let lines = content.lines().skip(2);
    let coins: Vec<_> = lines
        .map(|line| {
            line.split(" ")
                .map(|element| element.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let lines = coins
        .into_iter()
        .map(|coins| {
            let mut content = String::new();
            for value in 10..101 {
                let result = find_coins(&coins, &mut HashMap::new(), value, None);

                let res = result
                    .unwrap()
                    .into_iter()
                    .sorted_by(|a, b| b.0.cmp(&a.0))
                    .map(|(key, value)| format!("{}x{}", value, key))
                    .join(" ");
                content.push_str(&res);
                content.push('\n');
            }
            content
        })
        .join("\n");

    lines
}

fn find_coins(
    coins: &[usize],
    amounts: &mut HashMap<usize, usize>,
    goal: usize,
    mut best: Option<HashMap<usize, usize>>,
) -> Option<HashMap<usize, usize>> {
    if calc_count(&amounts) > best.clone().map(|best| calc_count(&best)).unwrap_or(usize::MAX) {
        return None;
    }
    let current_sum: usize = amounts.iter().map(|(value, amount)| value * amount).sum();
    if current_sum == goal {
        return Some(amounts.clone());
    } else if current_sum > goal {
        // INGORE
        return None;
    }

    for coin in coins.iter().rev() {
        let value = amounts.get(coin).cloned().unwrap_or_default();
        amounts.insert(*coin, value + 1);
        match find_coins(coins, amounts, goal, best.clone()) {
            None => {}
            Some(test_best) => {
                if calc_count(&test_best)
                    < best
                        .clone()
                        .map(|best| calc_count(&best))
                        .unwrap_or(usize::MAX)
                {
                    best = Some(test_best);
                }
            }
        }
        amounts.insert(*coin, value);
    }

    best
}

fn calc_count(a: &HashMap<usize, usize>) -> usize {
    a.values().sum::<usize>()
}
