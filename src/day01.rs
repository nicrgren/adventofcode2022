//! --- Day 1: Calorie Counting ---

//! Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that,
//! their favorite snack is a special type of star fruit that only grows deep in the jungle.
//! The Elves have brought you on their annual expedition to the grove where the fruit grows.
//!
//! To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th.
//! Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
//! the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air;
//! the Elves' expedition traditionally goes on foot.
//! As your boats approach land, the Elves begin taking inventory of their supplies.
//! One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
//!
//! The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc.
//! that they've brought with them, one item per line.
//! Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
//!
//! For example, suppose the Elves finish writing their items' Calories and end up with the following list:

//! ```no_run
//! 1000
//! 2000
//! 3000
//!
//! 4000
//!
//! 5000
//! 6000
//!
//! 7000
//! 8000
//! 9000
//!
//! 10000
//! ```

//! This list represents the Calories of the food carried by five Elves:
//!
//!     The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
//!     The second Elf is carrying one food item with 4000 Calories.
//!     The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
//!     The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
//!     The fifth Elf is carrying one food item with 10000 Calories.
//!
//! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to
//! know how many Calories are being carried by the Elf carrying the most Calories. In the example above,
//! this is 24000 (carried by the fourth Elf).
//!
//! Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
//!
//! To play, please identify yourself via one of these services:

//! --- Part Two ---

//! By the time you calculate the answer to the Elves' question,
//! they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.
//!
//! To avoid this unacceptable situation, the Elves would instead like to know the total Calories
//! carried by the top three Elves carrying the most Calories.
//! That way, even if one of those Elves runs out of snacks, they still have two backups.
//!
//! In the example above, the top three Elves are the fourth Elf (with 24000 Calories),
//! then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories).
//! The sum of the Calories carried by these three elves is 45000.
//!
//! Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

#[derive(Debug)]
struct Elf {
    idx: usize,
    sum: i64,
}

pub fn solve(input: &str) {
    println!("\tPart1: {}", solve_part1(input).1);
    println!("\tPart2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> (usize, i64) {
    let elf = to_elves(input).max_by_key(|elf| elf.sum).unwrap();

    (elf.idx, elf.sum)
}

fn solve_part2(input: &str) -> i64 {
    let mut elves = to_elves(input).collect::<Vec<_>>();

    // Fuckit
    elves.sort_by(|elf1, elf2| elf1.sum.cmp(&elf2.sum).reverse());

    elves[0].sum + elves[1].sum + elves[2].sum
}

fn to_elves(input: &'_ str) -> impl Iterator<Item = Elf> + '_ {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|s| s.trim().parse::<i64>().expect("Parsing line"))
                .sum::<i64>()
        })
        .enumerate()
        .map(|(idx, sum)| Elf { idx: idx + 1, sum })
}

#[cfg(test)]
mod tests {

    static EXAMPLE_INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part1_example() {
        let (idx, sum) = super::solve_part1(EXAMPLE_INPUT);
        assert_eq!(idx, 4);
        assert_eq!(sum, 24000);
    }

    #[test]
    fn part2_example() {
        let sum = super::solve_part2(EXAMPLE_INPUT);
        assert_eq!(sum, 45000);
    }
}
