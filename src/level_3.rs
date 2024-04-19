use itertools::Itertools;
use std::collections::HashMap;

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
            for value in 1..101 {
                let mut map: HashMap<usize, usize> = HashMap::new();
                for coin in coin_change(value, &coins).1 {
                    let value = map.get(&coin).cloned().unwrap_or_default();
                    map.insert(coin, value+1);
                };
                
                let res = map
                    .into_iter()
                    .sorted_by(|a, b| b.0.cmp(&a.0))
                    .map(|(key, value)| format!("{}x{}", value, key))
                    .join(" ");
                content.push_str(&res);
                content.push('\n');
            }
            content
        })
        .join("");

    lines
}

fn coin_change(target_amount: usize, denominations: &[usize]) -> (usize, Vec<usize>) {
    // Initialize a vector to store the minimum coins needed for each amount up to the target amount
    // Start with a large number for each amount except 0 (which requires 0 coins)
    let mut min_coins = vec![usize::MAX - 10; target_amount + 1];
    min_coins[0] = 0;  // No coins are needed to make amount 0

    // Initialize a vector of vectors to store the coin combination for each amount
    let mut coin_combinations: Vec<Vec<usize>> = vec![Vec::new(); target_amount + 1];

    // Loop through each denomination
    for &coin in denominations.iter() {
        // Update the min_coins and coin_combinations lists
        for amount in coin..=target_amount {
            if min_coins[amount - coin] + 1 < min_coins[amount] {
                min_coins[amount] = min_coins[amount - coin] + 1;
                // Update the coin combination for the current amount
                coin_combinations[amount] = coin_combinations[amount - coin].clone();
                coin_combinations[amount].push(coin);
            }
        }
    }

    // Retrieve the minimum number of coins needed and the combination for the target amount
    let min_number_of_coins = min_coins[target_amount];
    let coin_combination = &coin_combinations[target_amount];

    (min_number_of_coins, coin_combination.clone())
}

// fn find_coins(
//     coins: &[usize],
//     amounts: &mut HashMap<usize, usize>,
//     goal: usize,
//     mut best: Option<HashMap<usize, usize>>,
// ) -> Option<HashMap<usize, usize>> {
//     println!("{}", goal);
//     let current_sum: usize = amounts.iter().map(|(value, amount)| value * amount).sum();
//     if current_sum > goal {
//         return None;
//     }
// 
//     if calc_count(&amounts)
//         >= best
//             .clone()
//             .map(|best| calc_count(&best))
//             .unwrap_or(usize::MAX)
//     {
//         return None;
//     }
// 
//     if current_sum == goal {
//         return Some(amounts.clone());
//     }
// 
//     for coin in coins.iter().rev() {
//         let value = amounts.get(coin).cloned().unwrap_or_default();
//         amounts.insert(*coin, value + 1);
//         match find_coins(coins, amounts, goal, best.clone()) {
//             None => {}
//             Some(test_best) => {
//                 best = Some(test_best);
//             }
//         }
//         amounts.insert(*coin, value);
//     }
// 
//     best
// }
// 
// fn calc_count(a: &HashMap<usize, usize>) -> usize {
//     a.values().sum::<usize>()
// }
