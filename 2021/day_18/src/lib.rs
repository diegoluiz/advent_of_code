use helper::helper;
use std::fmt;

#[derive(Debug, Clone)]
struct FishNumber {
    left: Option<Box<FishNumber>>,
    right: Option<Box<FishNumber>>,
    value: Option<i32>,
}

impl fmt::Display for FishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.is_some() {
            return write!(f, "{}", self.value.unwrap());
        } else {
            let l = self.left.clone().unwrap();
            let r = self.right.clone().unwrap();
            return write!(f, "[{},{}]", l, r);
        }
    }
}

fn parse_fish_number(str: &str, i: &mut usize) -> Box<FishNumber> {
    let mut fish_number = FishNumber {
        value: None,
        left: None,
        right: None,
    };

    let number = str[*i..*i + 1].parse::<i32>();
    match number {
        Ok(x) => {
            fish_number.value = Some(x);
            *i += 1;
            return Box::new(fish_number);
        }
        Err(_) => {}
    }

    if str[*i..*i + 1].eq("[") {
        *i += 1;
        let l = parse_fish_number(str, i);
        fish_number.left = Option::from(l);
    }

    if str[*i..*i + 1].eq(",") {
        *i += 1;
        let r = parse_fish_number(str, i);
        fish_number.right = Option::from(r);
    }

    if str[*i..*i + 1].eq("]") {
        *i += 1;
    }
    return Box::new(fish_number);
}

fn sum_fish_number(a: Box<FishNumber>, b: Box<FishNumber>) -> Box<FishNumber> {
    Box::new(FishNumber {
        left: Some(a),
        right: Some(b),
        value: None,
    })
}

fn split(a: &mut Box<FishNumber>) -> bool {
    if a.value.is_some() {
        let v = a.value.unwrap();
        if v >= 10 {
            let l = ((v as f64) / 2 as f64).floor() as i32;
            let r = ((v as f64) / 2 as f64).ceil() as i32;

            a.value = None;
            a.left = Some(Box::new(FishNumber {
                left: None,
                right: None,
                value: Some(l),
            }));
            a.right = Some(Box::new(FishNumber {
                left: None,
                right: None,
                value: Some(r),
            }));
            return true;
        }

        return false;
    }

    let mut l = a.left.clone().unwrap();
    let s_l = split(&mut l);
    a.left = Some(l);
    if s_l {
        return s_l;
    }

    let mut r = a.right.clone().unwrap();
    let s_r = split(&mut r);
    a.right = Some(r);

    return s_r;
}

fn sum_left_most(a: &mut Box<FishNumber>, v: i32) {
    if a.value.is_some() {
        a.value = Some(a.value.unwrap() + v);
        return;
    }

    let mut l = a.left.clone().unwrap();
    sum_left_most(&mut l, v);
    a.left = Some(l);
}

fn sum_right_most(a: &mut Box<FishNumber>, v: i32) {
    if a.value.is_some() {
        a.value = Some(a.value.unwrap() + v);
        return;
    }

    let mut r = a.right.clone().unwrap();
    sum_right_most(&mut r, v);
    a.right = Some(r);
}

fn explode(a: &mut Box<FishNumber>, depth: usize) -> Option<(bool, i32, i32)> {
    let d = depth + 1;

    let c = a.clone();
    if c.value.is_some() {
        return None;
    }

    let mut l = c.left.unwrap();
    let res_l = explode(&mut l, d);
    a.left = Some(l);
    match res_l {
        None => {}
        Some(x) => {
            if x.0 {
                a.left = Some(Box::new(FishNumber {
                    left: None,
                    right: None,
                    value: Some(0),
                }));
            }

            if x.2 != 0 {
                let mut r = a.right.clone().unwrap();
                sum_left_most(&mut r, x.2);
                a.right = Some(r);
            }

            return Some((false, x.1, 0));
        }
    }

    let mut r = c.right.unwrap();
    let res_r = explode(&mut r, d);
    a.right = Some(r);
    match res_r {
        None => {}
        Some(x) => {
            if x.0 {
                a.right = Some(Box::new(FishNumber {
                    left: None,
                    right: None,
                    value: Some(0),
                }));
            }

            if x.1 != 0 {
                let mut l = a.left.clone().unwrap();
                sum_right_most(&mut l, x.1);
                a.left = Some(l);
            }

            return Some((false, 0, x.2));
        }
    }

    if d > 4 {
        let c = a.clone();
        let l = c.left.unwrap();
        let r = c.right.unwrap();
        let l_n = l.value.unwrap();
        let r_n = r.value.unwrap();
        return Some((true, l_n, r_n));
    }

    return None;
}

fn reduce_fish_number(a: &mut Box<FishNumber>) {
    loop {
        let exploded = explode(a, 0).is_some();
        if exploded {
            continue;
        }

        let split = split(a);

        if !split {
            break;
        }
    }
}

fn magnitute(a: Box<FishNumber>) -> i32 {
    if a.value.is_some() {
        return a.value.unwrap();
    }

    let l = magnitute(a.left.unwrap());
    let r = magnitute(a.right.unwrap());

    return (3 * l) + (2 * r);
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: Vec<String>,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_18".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);

        let nodes = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();

        ChallengeInput { data: nodes }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let fish_numbers = lines
            .iter()
            .map(|x| {
                let mut idx = 0;
                parse_fish_number(x, &mut idx)
            })
            .collect::<Vec<_>>();

        let mut is_first = true;
        let mut total: Box<FishNumber> = Box::new(FishNumber {
            left: None,
            right: None,
            value: None,
        });
        for fish_number in fish_numbers {
            if is_first {
                total = fish_number;
                is_first = false;
            } else {
                total = sum_fish_number(total, fish_number);
            }

            reduce_fish_number(&mut total);
        }

        // println!("{}", total);
        magnitute(total).to_string()
    }

    fn part_02(&mut self, input: &ChallengeInput) -> String {
        let lines = input.data.clone();
        let fish_numbers = lines
            .iter()
            .map(|x| {
                let mut idx = 0;
                parse_fish_number(x, &mut idx)
            })
            .collect::<Vec<_>>();

        let mut m = 0;
        for i in 0..fish_numbers.len() {
            for j in i + 1..fish_numbers.len() {
                let n1 = fish_numbers.get(i).unwrap();
                let n2 = fish_numbers.get(j).unwrap();
                let mut r = sum_fish_number(n1.clone(), n2.clone());
                reduce_fish_number(&mut r);
                let n = magnitute(r);
                m = m.max(n);

                let mut r = sum_fish_number(n2.clone(), n1.clone());
                reduce_fish_number(&mut r);
                let n = magnitute(r);

                m = m.max(n);
            }
        }

        m.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut idx = 0;
        let n1 = parse_fish_number("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", &mut idx);
        let mut idx = 0;
        let n2 = parse_fish_number("[1,1]", &mut idx);

        let r = sum_fish_number(n1, n2);

        let r = format!("{}", r);
        assert_eq!(r, "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[1,1]]");
    }

    #[test]
    fn test_split() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[2,9]", &mut idx);
        n1.left = Some(Box::new(FishNumber {
            left: None,
            right: None,
            value: Some(15),
        }));

        let r = split(&mut n1);

        let n1 = format!("{}", n1);
        assert_eq!(r, true);
        assert_eq!(n1, "[[7,8],9]");
    }

    #[test]
    fn test_explode_1() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[[[[[9,8],1],2],3],4]", &mut idx);

        let idx = 0;
        let r = explode(&mut n1, idx);

        let n1 = format!("{}", n1);
        assert_eq!(r.unwrap(), (false, 9, 0));
        assert_eq!(n1, "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_explode_2() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[7,[6,[5,[4,[3,2]]]]]", &mut idx);

        let idx: usize = 0;
        let r = explode(&mut n1, idx);

        let n1 = format!("{}", n1);
        assert_eq!(r.unwrap(), (false, 0, 2));
        assert_eq!(n1, "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn test_explode_3() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[[6,[5,[4,[3,2]]]],1]", &mut idx);

        let idx: usize = 0;
        let r = explode(&mut n1, idx);

        let n1 = format!("{}", n1);
        assert_eq!(r.unwrap(), (false, 0, 0));
        assert_eq!(n1, "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn test_explode_4() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", &mut idx);

        let idx: usize = 0;
        let r = explode(&mut n1, idx);

        let n1 = format!("{}", n1);
        assert_eq!(r.unwrap(), (false, 0, 0));
        assert_eq!(n1, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    }

    #[test]
    fn test_explode_5() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", &mut idx);

        let idx: usize = 0;
        let r = explode(&mut n1, idx);

        let n1 = format!("{}", n1);
        assert_eq!(r.unwrap(), (false, 0, 2));
        assert_eq!(n1, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_reduce() {
        let mut idx = 0;
        let mut n1 = parse_fish_number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", &mut idx);

        reduce_fish_number(&mut n1);

        let n1 = format!("{}", n1);
        assert_eq!(n1, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_func_1() {
        let mut idx = 0;
        let n1 = parse_fish_number("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", &mut idx);

        let mut idx = 0;
        let n2 = parse_fish_number("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", &mut idx);

        let mut r = sum_fish_number(n1, n2);

        reduce_fish_number(&mut r);

        let r = format!("{}", r);
        assert_eq!(
            r,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
    }
}
