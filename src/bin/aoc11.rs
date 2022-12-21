use std::{fs, ops::Rem};

struct Monkey {
    items: Vec<i64>,
    test_div: i64,
    true_monkey: usize,
    false_monkey: usize,
    operation: Box<dyn Fn(i64) -> i64>,
    item_inspection_cnt: usize,
}

fn last_number_from_str(s: &str) -> Option<i64> {
    s.split(" ").last().unwrap().parse().ok()
}

fn monkey_around(monkeys: &mut Vec<Monkey>, do_div_3: bool) {
    let factor = if do_div_3 { 3 } else { 1 };
    let demon = common_denom(&monkeys);
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let mut thrown_items: Vec<(usize, i64)> = Vec::new();
        for item in &mut monkey.items {
            // Mod by common denominator to manage total size of the items
            *item = ((monkey.operation)(*item) / factor) % demon;

            monkey.item_inspection_cnt += 1;

            // Handle removal and addition of items after loop
            if item.rem(monkey.test_div) == 0 {
                thrown_items.push((monkey.true_monkey, *item));
            } else {
                thrown_items.push((monkey.false_monkey, *item));
            }
        }

        for item in &thrown_items {
            // This is not the right way to remove an item, since it can remove duplicates
            // For this problem that is okay, since they will go to the same place
            monkey.items.retain(|f| *f != item.1);
        }

        for item in &thrown_items {
            monkeys[item.0].items.push(item.1);
        }
    }
}

impl Monkey {
    pub fn from(raw: &str) -> Monkey {
        let mut iter = raw.lines().skip(1);

        // Parse "Starting items: ##, ##, ##..."
        let items = iter
            .next()
            .unwrap()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        // Parse 'Operation: new = old ? ##'
        let s = iter.next().unwrap();
        let operation_val = last_number_from_str(s);
        let is_add = s.contains("+");
        let operation = Box::new(move |x| {
            if is_add {
                operation_val.unwrap_or(x) + x
            } else {
                operation_val.unwrap_or(x) * x
            }
        });

        // Test: divisible by #
        let test_div: i64 = last_number_from_str(iter.next().unwrap()).unwrap();

        // If true: throw to monkey #
        let true_monkey = last_number_from_str(iter.next().unwrap()).unwrap() as usize;

        // If false: throw to monkey #
        let false_monkey = last_number_from_str(iter.next().unwrap()).unwrap() as usize;

        Monkey {
            items,
            test_div,
            true_monkey,
            false_monkey,
            operation,
            item_inspection_cnt: 0,
        }
    }
}

fn common_denom(monkeys: &Vec<Monkey>) -> i64 {
    monkeys.iter().fold(1, |sum, f| sum * f.test_div)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open file");
    let mut monkeys = input
        .split("\n\n")
        .enumerate()
        .map(|s| Monkey::from(s.1))
        .collect::<Vec<Monkey>>();

    for i in 0..10000 {
        monkey_around(&mut monkeys, false);
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            i + 1
        );

        for monkey in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", monkey.0, monkey.1.items);
        }
        for monkey in monkeys.iter().enumerate() {
            println!(
                "Monkey {}: inspected items {} times.",
                monkey.0, monkey.1.item_inspection_cnt
            );
        }

        println!("")
    }

    let mut sorted_inspections = monkeys
        .iter()
        .map(|f| f.item_inspection_cnt)
        .collect::<Vec<usize>>();
    sorted_inspections.sort();
    sorted_inspections.reverse();

    println!(
        "Result: {:?}",
        sorted_inspections[0] * sorted_inspections[1]
    );
}
