use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::shared::{Day, PartSolution};

#[derive(Default, Debug)]
pub struct Bag {
    pub name: String,
    pub parents: RefCell<Vec<Rc<Bag>>>,
    pub children: RefCell<Vec<(u32, Rc<Bag>)>>,
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let child_names: Vec<String> = self
            .children
            .borrow()
            .iter()
            .map(|(count, bag)| format!("Name: {}, count: {}", &bag.name, count))
            .collect();

        let parent_names: Vec<String> = self
            .parents
            .borrow()
            .iter()
            .map(|bag| format!("Name: {}", &bag.name))
            .collect();

        write!(
            f,
            "Name: {}, parents: {:?}, children: {:?} ",
            self.name, parent_names, child_names
        )
    }
}

pub fn map_bag_color_with_count(bag_color_with_count: &str) -> (u32, String) {
    let split = bag_color_with_count.trim().split_once(' ').unwrap();

    let _test: Vec<char> = bag_color_with_count.chars().collect();

    (split.0.parse().unwrap(), split.1.to_string())
}

pub fn parse_bag_line(bag_line: &str) -> (String, Vec<(u32, String)>) {
    let cleaned_up = bag_line
        .replace("bags", "")
        .replace("bag", "")
        .replace('.', "");

    let split: Vec<&str> = cleaned_up.split("contain").collect();

    let bag_name = split.first().unwrap().trim();

    let inside_bags = split.get(1).unwrap().trim();

    if inside_bags == "no other" {
        return (bag_name.to_string(), Vec::<(u32, String)>::new());
    }

    let inside_bags_with_count = inside_bags
        .split(',')
        .map(map_bag_color_with_count)
        .collect();

    (bag_name.to_string(), inside_bags_with_count)
}

pub fn parse_bags(bag_lines: &[String]) -> HashMap<String, Rc<Bag>> {
    let mut bag_parsed: HashMap<String, Rc<Bag>> = HashMap::new();

    for bag_line in bag_lines {
        let (bag_name, count_with_bag_name) = parse_bag_line(bag_line);

        let mut parsed_child_bags_current_line: Vec<(u32, Rc<Bag>)> = Vec::new();

        for (count, child_bag_name) in count_with_bag_name {
            let bag = bag_parsed
                .entry(child_bag_name.to_string())
                .or_insert_with(|| {
                    Rc::new(Bag {
                        name: child_bag_name.to_string(),
                        ..Bag::default()
                    })
                });

            parsed_child_bags_current_line.push((count, Rc::clone(bag)));
        }

        let bag: &Rc<Bag> = bag_parsed.entry(bag_name.to_string()).or_insert_with(|| {
            Rc::new(Bag {
                name: bag_name.to_string(),
                ..Bag::default()
            })
        });

        let mut bag_children = bag.children.borrow_mut();

        for (count, child_bag) in parsed_child_bags_current_line {
            let mut child_bag_parents = child_bag.parents.borrow_mut();

            child_bag_parents.push(Rc::clone(bag));

            bag_children.push((count, Rc::clone(&child_bag)));
        }
    }

    bag_parsed
}

fn get_parent_names_recursive(bag: &Rc<Bag>) -> Vec<String> {
    let mut my_parent_names: Vec<String> = bag
        .parents
        .borrow()
        .iter()
        .map(|b| b.name.clone())
        .collect();

    bag.parents
        .borrow()
        .iter()
        .for_each(|p| my_parent_names.append(&mut get_parent_names_recursive(p)));

    my_parent_names
}

fn count_parents(bag_parsed: &HashMap<String, Rc<Bag>>, start: &str) -> u32 {
    let start_bag = bag_parsed.get(start).unwrap();

    let mut parent_names = get_parent_names_recursive(start_bag);

    parent_names.sort();
    parent_names.dedup();

    u32::try_from(parent_names.len()).unwrap()
}

fn count_bags_recursive(bag: &Rc<Bag>) -> u32 {
    let children = bag.children.borrow();

    children
        .iter()
        .map(|(c, b)| {
            let sum_of_children = count_bags_recursive(b);

            c + c * sum_of_children
        })
        .sum()
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        const BAG_NAME: &str = "shiny gold";
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let bags = parse_bags(&lines);

        PartSolution::U32(count_parents(&bags, BAG_NAME))
    }

    fn part_2(&self) -> PartSolution {
        const BAG_NAME: &str = "shiny gold";
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let bags = parse_bags(&lines);

        PartSolution::U32(count_bags_recursive(bags.get(BAG_NAME).unwrap()))
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use crate::day_07::{Solution, count_parents, parse_bag_line, parse_bags};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(272), (Solution {}).part_1());
        }

        #[test]
        fn parse_bag_line_1() {
            let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "light red".to_string(),
                    vec![
                        (1, "bright white".to_string()),
                        (2, "muted yellow".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_2() {
            let input = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "dark orange".to_string(),
                    vec![
                        (3, "bright white".to_string()),
                        (4, "muted yellow".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_3() {
            let input = "bright white bags contain 1 shiny gold bag.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "bright white".to_string(),
                    vec![(1, "shiny gold".to_string())]
                ),
            );
        }

        #[test]
        fn parse_bag_line_4() {
            let input = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "muted yellow".to_string(),
                    vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())]
                ),
            );
        }

        #[test]
        fn parse_bag_line_5() {
            let input = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "shiny gold".to_string(),
                    vec![
                        (1, "dark olive".to_string()),
                        (2, "vibrant plum".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_6() {
            let input = "dark olive bags contain 3 faded blue bags, 4 dotted black bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "dark olive".to_string(),
                    vec![
                        (3, "faded blue".to_string()),
                        (4, "dotted black".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_7() {
            let input = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "vibrant plum".to_string(),
                    vec![
                        (5, "faded blue".to_string()),
                        (6, "dotted black".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_8() {
            let input = "faded blue bags contain no other bags.";

            let result = parse_bag_line(input);

            assert_eq!(result, ("faded blue".to_string(), vec![]));
        }

        #[test]
        fn parse_bag_line_9() {
            let input = "dotted black bags contain no other bags.";

            let result = parse_bag_line(input);

            assert_eq!(result, ("dotted black".to_string(), vec![]));
        }

        #[test]
        fn big_test() {
            let input = [
                "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                "bright white bags contain 1 shiny gold bag.",
                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                "faded blue bags contain no other bags.",
                "dotted black bags contain no other bags.",
            ];

            let lines: Vec<String> = input.map(Into::into).into();

            let rst = count_parents(&parse_bags(&lines), "shiny gold");

            assert_eq!(rst, 4);
        }
    }

    mod part_2 {
        use crate::day_07::{Solution, count_bags_recursive, parse_bag_line, parse_bags};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(172_246), (Solution {}).part_2());
        }

        #[test]
        fn parse_bag_line_1() {
            let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "light red".to_string(),
                    vec![
                        (1, "bright white".to_string()),
                        (2, "muted yellow".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_2() {
            let input = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "dark orange".to_string(),
                    vec![
                        (3, "bright white".to_string()),
                        (4, "muted yellow".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_3() {
            let input = "bright white bags contain 1 shiny gold bag.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "bright white".to_string(),
                    vec![(1, "shiny gold".to_string())]
                ),
            );
        }

        #[test]
        fn parse_bag_line_4() {
            let input = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "muted yellow".to_string(),
                    vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())]
                ),
            );
        }

        #[test]
        fn parse_bag_line_5() {
            let input = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "shiny gold".to_string(),
                    vec![
                        (1, "dark olive".to_string()),
                        (2, "vibrant plum".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_6() {
            let input = "dark olive bags contain 3 faded blue bags, 4 dotted black bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "dark olive".to_string(),
                    vec![
                        (3, "faded blue".to_string()),
                        (4, "dotted black".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_7() {
            let input = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";

            let result = parse_bag_line(input);

            assert_eq!(
                result,
                (
                    "vibrant plum".to_string(),
                    vec![
                        (5, "faded blue".to_string()),
                        (6, "dotted black".to_string())
                    ]
                ),
            );
        }

        #[test]
        fn parse_bag_line_8() {
            let input = "faded blue bags contain no other bags.";

            let result = parse_bag_line(input);

            assert_eq!(result, ("faded blue".to_string(), vec![]));
        }

        #[test]
        fn parse_bag_line_9() {
            let input = "dotted black bags contain no other bags.";

            let result = parse_bag_line(input);

            assert_eq!(result, ("dotted black".to_string(), vec![]));
        }

        #[test]
        fn big_test() {
            const BAG_NAME: &str = "shiny gold";

            let input = [
                "shiny gold bags contain 2 dark red bags.",
                "dark red bags contain 2 dark orange bags.",
                "dark orange bags contain 2 dark yellow bags.",
                "dark yellow bags contain 2 dark green bags.",
                "dark green bags contain 2 dark blue bags.",
                "dark blue bags contain 2 dark violet bags.",
                "dark violet bags contain no other bags.",
            ];

            let lines: Vec<String> = input.map(Into::into).into();

            let bags = parse_bags(&lines);

            let rst = count_bags_recursive(bags.get(BAG_NAME).unwrap());

            assert_eq!(rst, 126);
        }
    }
}
