use std::collections::VecDeque;
use nom::{
    IResult,
    multi::{many1, separated_list0},
    combinator::map,
    branch::alt,
    character::{complete::{line_ending, multispace0}, self},
    bytes::complete::tag, sequence::{preceded, delimited, pair},
};

type Item = u64;
fn item_parse(i: &str) -> IResult<&str, Item> {
    character::complete::u64(i)
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<Item>,
    op: Operation,
    test: Item,
    on_true: usize,
    on_false: usize,
    total_inspected: usize,
}

#[derive(Clone, Debug)]
enum Operation {
    Add(Item),
    Mul(Item),
    Square,
}

impl Operation {
    fn parse(i: &str) -> IResult<&str, Self> {
        let add = map(preceded(tag("+ "), item_parse), Operation::Add);
        let mul = map(preceded(tag("* "), item_parse), Operation::Mul);
        let square = map(tag("* old"), |_| Operation::Square);
        alt((add, mul, square))(i)
    }
}

#[derive(Clone, Default, Debug)]
struct Toss {
    toss_true: Vec<Item>,
    toss_false: Vec<Item>,
    target_true: usize,
    target_false: usize,
}

#[derive(Clone, Default, Debug)]
struct WorryConfig {
    worry_div: bool,
    monkey_modulo: Item,
}

impl Monkey {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, _monkey) = delimited(
            pair(multispace0, tag("Monkey ")),
            character::complete::u32,
            pair(tag(":"), line_ending)
        )(i)?;
        let (i, items) = delimited(
            tag("  Starting items: "),
            separated_list0(pair(tag(","), multispace0), item_parse),
            line_ending
        )(i)?;
        let (i, op) = delimited(
            tag("  Operation: new = old "),
            Operation::parse,
            line_ending
        )(i)?;
        let (i, test) = delimited(
            tag("  Test: divisible by "),
            item_parse,
            line_ending
        )(i)?;
        let (i, on_true) = delimited(
            tag("    If true: throw to monkey "),
            item_parse,
            line_ending
        )(i)?;
        let (i, on_false) = delimited(
            tag("    If false: throw to monkey "),
            item_parse,
            line_ending
        )(i)?;

        IResult::Ok((i, Self {
            items: items.into(),
            op,
            test,
            on_true: on_true as usize,
            on_false: on_false as usize,
            total_inspected: 0,
        }))
    }

    fn toss(&mut self, toss: &mut Toss, config: &WorryConfig) {

        // Toss setup
        toss.toss_true.clear();
        toss.toss_false.clear();
        toss.target_true = self.on_true;
        toss.target_false = self.on_false;

        // Run toss algorithm
        self.total_inspected += self.items.len();
        for item in self.items.drain(..) {
            let item = match self.op {
                Operation::Add(v) => item + v,
                Operation::Mul(v) => item * v,
                Operation::Square => item * item,
            };
            let item = if config.worry_div { item / 3 } else { item % config.monkey_modulo };
            if item % self.test == 0 {
                toss.toss_true.push(item);
            }
            else {
                toss.toss_false.push(item);
            };
        }
    }

    fn catch(&mut self, incoming: &[Item]) {
        self.items.extend(incoming.iter());
    }

}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: u32, config: &WorryConfig) -> usize {
    let mut toss = Toss::default();
    for _round in 0..rounds {
        for monkey in 0..monkeys.len() {
            monkeys.get_mut(monkey).unwrap().toss(&mut toss, config);
            monkeys.get_mut(toss.target_true).unwrap().catch(&toss.toss_true);
            monkeys.get_mut(toss.target_false).unwrap().catch(&toss.toss_false);
        }
    }

    let mut totals: Vec<usize> = monkeys.iter().map(|m| m.total_inspected).collect();
    for (i, total) in totals.iter().enumerate() {
        println!("Monkey {i} inspected items {} times", total);
    }

    totals.sort_unstable();
    totals.pop().unwrap() * totals.pop().unwrap()

}

fn main() {
    let input = include_str!("input");
    //let input = include_str!("sample");

    let (_, mut monkeys) = many1(Monkey::parse)(input).unwrap();
    let monkey_modulo = monkeys.iter().map(|m| m.test).product();
    let mut config = WorryConfig { worry_div: true, monkey_modulo };

    let part1 = monkey_business(&mut monkeys.clone(), 20, &config);
    config.worry_div = false;
    let part2 = monkey_business(&mut monkeys, 10000, &config);
    println!("Total monkey business after 20 rounds: {}", part1);
    println!("Total monkey business after 10000 rounds (high worry): {}", part2);
}
