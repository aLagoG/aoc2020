use regex::Regex;
use std::collections::HashMap;

const GOAL: &'static str = "shiny gold";

#[derive(Clone, Debug)]
pub struct Bag {
    name: String,
    children: HashMap<String, usize>,
}

type InputType = HashMap<String, Bag>;

pub fn input(input: &str) -> InputType {
    let re = Regex::new(r"(\w+ \w+) bags?").unwrap();
    let re2 = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    let read = input.lines().map(|l| {
        let name = re.captures(l).unwrap()[1].to_string();
        let mut contains = HashMap::new();
        for cap in re2.captures_iter(l) {
            let num: usize = cap[1].parse().unwrap();
            let bag = cap[2].to_string();
            contains.insert(bag, num);
        }
        Bag {
            name,
            children: contains,
        }
    });

    let mut res = HashMap::new();
    for r in read {
        res.insert(r.name.clone(), r);
    }
    res
}

struct Res {
    result: HashMap<String, bool>,
}

impl Res {
    fn search(&mut self, node: &Bag, graph: &InputType) -> bool {
        if node.name == GOAL {
            return true;
        }

        if self.result.contains_key(&node.name) {
            return self.result[&node.name];
        }

        let mut good = false;
        for (child, _) in node.children.iter() {
            good |= self.search(&graph[child], graph);
        }

        self.result.insert(node.name.clone(), good);
        good
    }
}

pub fn part1(input: InputType) -> usize {
    let mut res = Res {
        result: HashMap::new(),
    };

    for (_, bag) in input.iter() {
        res.search(bag, &input);
    }

    res.result.iter().filter(|&(_k, v)| *v).count()
}

struct Res2 {
    result: HashMap<String, usize>,
}

impl Res2 {
    fn search(&mut self, node: &Bag, graph: &InputType) -> usize {
        if self.result.contains_key(&node.name) {
            return self.result[&node.name];
        }

        let mut count = 1;
        for (child, num) in node.children.iter() {
            count += self.search(&graph[child], graph) * *num;
        }

        self.result.insert(node.name.clone(), count);
        count
    }
}

pub fn part2(input: InputType) -> usize {
    let mut res = Res2 {
        result: HashMap::new(),
    };

    res.search(&input[GOAL], &input);

    res.result[GOAL] - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input_str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        assert!(part1(input(input_str)) == 4);
    }

    #[test]
    fn p2() {
        let input_str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert!(part2(input(input_str)) == 126);
    }
}
