//! --- Day 3: Rucksack Reorganization ---
//!
//! One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey.
//! Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
//!
//! Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments.
//! The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack (your puzzle input),
//! but they need your help finding the errors.
//! Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single line.
//! A given rucksack always has the same number of items in each of its two compartments,
//! so the first half of the characters represent items in the first compartment,
//! while the second half of the characters represent items in the second compartment.
//!
//! For example, suppose you have the following list of contents from six rucksacks:
//!
//! * vJrwpWtwJgWrhcsFMMfFFhFp
//! * jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! * PmmdzqPrVvPwwTWBwg
//! * wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! * ttgJtRGJQctTZtZT
//! * CrZsJsPPZsGzwwsLwLmpwMDw
//!
//!     The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp,
//!       which means its first compartment contains the items vJrwpWtwJgWr,
//!       while the second compartment contains the items hcsFMMfFFhFp.
//!       The only item type that appears in both compartments is lowercase p.
//!
//!     The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL.
//!       The only item type that appears in both compartments is uppercase L.
//!
//!     The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg;
//!       the only common item type is uppercase P.
//!
//!     The fourth rucksack's compartments only share item type v.
//!
//!     The fifth rucksack's compartments only share item type t.
//!
//!     The sixth rucksack's compartments only share item type s.
//!
//! To help prioritize item rearrangement, every item type can be converted to a priority:
//!
//!     * Lowercase item types a through z have priorities 1 through 26.
//!     * Uppercase item types A through Z have priorities 27 through 52.
//!
//! In the above example, the priority of the item type that appears in both compartments of
//! each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
//!
//! Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
//!
//!

//! --- Part Two ---
//!
//! As you finish identifying the misplaced items, the Elves come to you with another issue.
//!
//! For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group.
//! For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves.
//! That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack,
//! and at most two of the Elves will be carrying any other item type.
//!
//! The problem is that someone forgot to put this year's updated authenticity sticker on the badges.
//! All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.
//!
//! Additionally, nobody wrote down which item type corresponds to each group's badges.
//! The only way to tell which item type is the right one is by finding the one item type that is common between all
//! three Elves in each group.
//!
//! Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type.
//! So, in the above example, the first group's rucksacks are the first three lines:
//!
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//!
//! And the second group's rucksacks are the next three lines:
//!
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//!
//! In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges.
//! In the second group, their badge item type must be Z.
//!
//! Priorities for these items must still be found to organize the sticker attachment efforts:
//! here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.
//!
//! Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?

struct Content<'a> {
    pub cmp1: &'a [u8],
    pub cmp2: &'a [u8],
}

impl<'a> Content<'a> {
    pub fn from_str(s: &'a str) -> Self {
        let (s1, s2) = s.trim().split_at(s.len() / 2);
        let (cmp1, cmp2) = (s1.as_bytes(), s2.as_bytes()); // working with ASCII
        Self { cmp1, cmp2 }
    }

    pub fn duplicate(&self) -> Option<u8> {
        self.cmp1
            .iter()
            .find(|&b| self.cmp2.contains(b))
            .or_else(|| self.cmp2.iter().find(|&b| self.cmp1.contains(b)))
            .copied()
    }
}

pub fn solve(s: &str) {
    println!("Day03");
    println!("\tPart1: {}", part01(s));
}

fn part01(s: &str) -> i64 {
    parse(s)
        .filter_map(|c| c.duplicate())
        .map(priority)
        .sum::<i64>()
}

fn parse(s: &str) -> impl Iterator<Item = Content<'_>> {
    s.trim().lines().map(Content::from_str)
}

fn priority(b: u8) -> i64 {
    // A = ASCII 65
    // a = ACSII 97

    // a-z = 1-26
    // A-Z = 27-52

    match b {
        64..=90 => (b as i64) - 65 + 27,
        97..=122 => (b as i64) - 96,

        _ => panic!("Invalid item `{}`", b as char),
    }
}

#[cfg(test)]
mod tests {

    static EXAMPLE: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn example() {
        assert_eq!(157, super::part01(EXAMPLE));
    }

    #[test]
    fn duplicates() {
        // In the above example, the priority of the item type that appears in both compartments of
        // each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157

        let contents = super::parse(EXAMPLE).collect::<Vec<_>>();

        assert_eq!(Some(b'p'), contents[0].duplicate());
        assert_eq!(Some(b'L'), contents[1].duplicate());
        assert_eq!(Some(b'P'), contents[2].duplicate());
        assert_eq!(Some(b'v'), contents[3].duplicate());
        assert_eq!(Some(b't'), contents[4].duplicate());
        assert_eq!(Some(b's'), contents[5].duplicate());
    }

    #[test]
    fn parsing() {
        let contents = super::parse(EXAMPLE).collect::<Vec<_>>();

        assert_eq!(contents[0].cmp1, b"vJrwpWtwJgWr", "content 1");
        assert_eq!(contents[0].cmp2, b"hcsFMMfFFhFp", "content 1");

        assert_eq!(contents[1].cmp1, b"jqHRNqRjqzjGDLGL", "content 2");
        assert_eq!(contents[1].cmp2, b"rsFMfFZSrLrFZsSL", "content 2");

        assert_eq!(contents[2].cmp1, b"PmmdzqPrV", "content 3");
        assert_eq!(contents[2].cmp2, b"vPwwTWBwg", "content 3");
    }

    #[test]
    fn priority() {
        // Lowercase item types a through z have priorities 1 through 26.
        // Uppercase item types A through Z have priorities 27 through 52.

        assert_eq!(1, super::priority(b'a'));
        assert_eq!(26, super::priority(b'z'));

        assert_eq!(27, super::priority(b'A'));
        assert_eq!(52, super::priority(b'Z'));
    }
}
