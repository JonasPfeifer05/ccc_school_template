use itertools::Itertools;

pub fn level_1(content: String) -> String {
    let lines = content.lines().skip(2);
    let map: Vec<_> = lines
        .map(|line| {
            line.split(" ")
                .map(|element| element.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut a = vec![];
    for line in map {
        let res = line.into_iter().enumerate().find_position(|x| x.0+1 != x.1);
        a.push(res.unwrap());
    }
    
    let mut string = String::new();
    for x in a {
        string.push_str(&format!("{}\n", x.0+1));
    }
    
    string
}
