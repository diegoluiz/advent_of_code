use helper::helper;
use std::{collections::HashSet, iter::FromIterator, time::Instant};

struct Entry<'a> {
    signals: Vec<&'a str>,
    digits: Vec<&'a str>,
}

impl Entry<'_> {
    fn create(input: &str) -> Entry {
        let parts = input.split("|").collect::<Vec<_>>();
        let signals = parts[0].trim().split(" ").collect::<Vec<_>>();
        let digits = parts[1].trim().split(" ").collect::<Vec<_>>();
        Entry { signals, digits }
    }
}

fn main() {
    let now = Instant::now();

    let input_file_path = helper::get_input_file(false);
    let input = helper::load_input(input_file_path);

    let entries = input
        .split("\n")
        .map(|x| Entry::create(x))
        .collect::<Vec<_>>();

    let now_p1 = Instant::now();
    let part_01_output = part_01(&entries);
    println!(
        "Part 1 result: {}   Took: {} microseconds",
        part_01_output,
        now_p1.elapsed().as_micros()
    );

    let now_p2 = Instant::now();
    let part_02_output = part_02(&entries);
    println!(
        "Part 2 result: {}   Took: {} microseconds",
        part_02_output,
        now_p2.elapsed().as_micros()
    );

    println!("Done in {} microseconds", now.elapsed().as_micros());
}

fn part_01(entries: &Vec<Entry>) -> String {
    let mut count = 0;
    for entry in entries {
        let digits = entry.digits.iter().map(|x| *x).collect::<Vec<_>>();
        for digit in digits {
            let len = (*digit).len();
            match len {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }

    count.to_string()
}

fn part_02(entries: &Vec<Entry>) -> String {
    let mut sum = 0;
    for entry in entries {
        let signals = entry
            .signals
            .iter()
            .map(|x| {
                let chars = x
                    .split("")
                    .filter(|x| *x != "")
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>();

                let hs: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
                hs
            })
            .collect::<Vec<_>>();

        let digits = entry
            .digits
            .iter()
            .map(|x| {
                let chars = x
                    .split("")
                    .filter(|x| *x != "")
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>();

                let hs: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
                hs
            })
            .collect::<Vec<_>>();

        let mut leds = [' '; 7];

        let n1 = signals.iter().filter(|x| x.len() == 2).next().unwrap();
        let n4 = signals.iter().filter(|x| x.len() == 4).next().unwrap();
        let n8 = signals.iter().filter(|x| x.len() == 7).next().unwrap();
        let n7 = signals.iter().filter(|x| x.len() == 3).next().unwrap();

        leds[0] = *(n7 - n1).iter().next().unwrap();

        let n6 = signals
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| {
                let y = *x - n1;
                x.len() - 1 == y.len()
            })
            .next()
            .unwrap();

        leds[2] = *(n8 - n6).iter().next().unwrap();

        let n5 = signals
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| {
                (*x - n6).len() == 0
            })
            .next()
            .unwrap();

        leds[4] = *(n6 - n5).iter().next().unwrap();

        let n2 = signals
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| x.contains(&leds[2]) && x.contains(&leds[4]))
            .next()
            .unwrap();

        let n3 = signals
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| *x != n2 && *x != n5)
            .next()
            .unwrap();

        let n0 = signals
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| x.contains(&leds[2]) && x.contains(&leds[4]))
            .next()
            .unwrap();

        let n9 = signals
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| *x != n0 && *x != n6)
            .next()
            .unwrap();

        let mut displayed = String::new();
        for digit in digits {
            if digit == *n0 { displayed.push('0'); }
            if digit == *n1 { displayed.push('1'); }
            if digit == *n2 { displayed.push('2'); }
            if digit == *n3 { displayed.push('3'); }
            if digit == *n4 { displayed.push('4'); }
            if digit == *n5 { displayed.push('5'); }
            if digit == *n6 { displayed.push('6'); }
            if digit == *n7 { displayed.push('7'); }
            if digit == *n8 { displayed.push('8'); }
            if digit == *n9 { displayed.push('9'); }
        }

        sum += &displayed.parse::<i32>().unwrap();
    }

    sum.to_string()
}
