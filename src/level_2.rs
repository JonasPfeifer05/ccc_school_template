use itertools::Itertools;

pub fn level_2(content: String) -> String {
    let lines = content.lines().skip(3);
    let coins: Vec<_> = lines
        .map(|line| {
            line.split(" ")
                .map(|element| element.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let lines = coins.chunks_exact(2).map(|coins| {
        println!("{:?}", coins);
        let given = &coins[0];
        let wanted = &coins[1];

        let mut pairs = vec![];
        'outer:
        for amount in wanted {
            for a in given {
                for b in given {
                    if a+b == *amount { 
                        pairs.push((*a,*b));
                        continue 'outer;
                    }
                }
            }
        }
        
        pairs.into_iter().map(|(a,b)| format!("{} {}\n", a,b)).join("")
    }).join("");
    
    lines
}
