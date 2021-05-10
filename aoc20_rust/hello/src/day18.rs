use std::fs;

pub fn run() {
    let input_file = "data/18";
    let first = part1(input_file);
    println!("{}", first);
    let second = part2(input_file);
    println!("{}", second);
}

#[derive(PartialEq, Debug)]
enum Operation {
    Plus,
    Multiplication,
    Initial,
}

fn update_sum(op: &Operation, sum: &mut i64, n: i64) {
    println!("{} {:?} {}", sum, op, n);
    match op {
        Operation::Plus => *sum += n,
        Operation::Multiplication => *sum *= n,
        Operation::Initial => *sum = n,
    }
}

fn check_next_sign(i: &usize, list: &Vec<&str>) -> Operation {
    let mut index = i.clone();
    while index < list.len() {
        match list[index] {
            "+" => return Operation::Plus,
            "*" => return Operation::Multiplication,
            _ => index += 1,
        } 
    }
    Operation::Initial
}

fn get_sum(i: &mut usize, list: &Vec<&str>, addition_precidence: bool, one_op: bool) -> i64 {
    if *i == list.len() {
        return 0;
    }
    let mut sum = 0;
    let mut op = Operation::Initial;
    while *i < list.len() && (!one_op || (op != Operation::Multiplication && one_op)){
        println!("i: {}", *i);
        match list[*i] {
            "(" => {
                *i += 1;
                update_sum(&op, &mut sum, get_sum(i, &list, addition_precidence, false));
            }
            ")" => {println!("returning {}", sum); return sum},
            "+" => op = Operation::Plus,
            "*" => op = Operation::Multiplication,
            x => {
                if !addition_precidence {
                    update_sum(&op, &mut sum, x.parse::<i64>().unwrap())
                } else {
                    if op == Operation::Multiplication && check_next_sign(i, list) != Operation::Multiplication {
                        update_sum(&Operation::Multiplication, &mut sum, get_sum(i, &list, addition_precidence, true));
                        *i -= 1;
                    } else {
                        update_sum(&op, &mut sum, x.parse::<i64>().unwrap())
                    }
                }
            },
        }
        *i += 1;
    }
    println!("r {}", sum);
    sum
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    s.lines()
        .map(|l| {
            l.split("")
                .filter(|c| c != &" " && c != &"")
                .collect::<Vec<&str>>()
        })
        .map(|list| {
            let mut index = 0;
            get_sum(&mut index, &list, false, false)
        }).sum()
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    s.lines()
        .map(|l| {
            l.split("")
                .filter(|c| c != &" " && c != &"")
                .collect::<Vec<&str>>()
        })
        .map(|list| {
            let mut index = 0;
            get_sum(&mut index, &list, true, false)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_part1_easy() {
    //     assert_eq!(part1("data/18_easy"), 71 + 51);
    // }

    #[test]
    fn test_part2_easy() {
        assert_eq!(part2("data/18_easy"), 9666);//282);
    }
    // 51360880403854 to low
}
/*
use Token::*;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let result: u64 = input
    .trim_end()
    .split('\n')
    .map(|line| {
      let tokens = Token::parse(line);

      eval_tokens(&tokens)
    })
    .sum();

  Some(Box::new(result))
}

fn eval_tokens(tokens: &Vec<Token>) -> u64 {
  let mut ops_stack: Vec<Token> = vec![];
  let mut nums_stack: Vec<u64> = vec![];

  for token in tokens.iter() {
    match token {
      Num(x) => try_perform_op(&mut ops_stack, *x, &mut nums_stack),
      Parens(y) => {
        let x = eval_tokens(y);
        try_perform_op(&mut ops_stack, x, &mut nums_stack);
      }
      operation => ops_stack.push(operation.clone()),
    }
  }

  nums_stack.pop().unwrap()
}

fn try_perform_op(ops_stack: &mut Vec<Token>, value: u64, nums_stack: &mut Vec<u64>) {
  match ops_stack.pop() {
    None => nums_stack.push(value),
    Some(Plus) => {
      let prev = nums_stack.pop().unwrap();
      nums_stack.push(prev + value);
    }
    Some(Star) => {
      let prev = nums_stack.pop().unwrap();
      nums_stack.push(prev * value);
    }
    unexpected => panic!("ops_stack should only contain Plus & Star, got: {:?}", unexpected),
  }
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let result = input
    .trim_end()
    .split('\n')
    .map(|line| {
      let tokens = Token::parse(line);
      let tokens = add_plus_parens(tokens);

      eval_tokens(&tokens)
    })
    .collect::<Vec<u64>>();

    println!("{:?}", result);

  Some(Box::new(1))
}

/// Regroups `tokens` by adding parens around groups of tokens connected by `+`.
/// This emulates changed operator precedence in part 2, without affecting parsing.
fn add_plus_parens(tokens: Vec<Token>) -> Vec<Token> {
  let mut with_plus_parens = vec![];
  let mut curr_plus_group = vec![];

  for token in tokens.into_iter() {
    match token {
      num @ Num(_) => {
        if curr_plus_group.is_empty() {
          with_plus_parens.push(num);
        } else {
          curr_plus_group.push(num);
        }
      }
      Star => {
        if curr_plus_group.is_empty() {
          with_plus_parens.push(Star);
        } else {
          with_plus_parens.push(Parens(curr_plus_group));
          curr_plus_group = vec![];

          with_plus_parens.push(Star);
        }
      }
      Plus => {
        if curr_plus_group.is_empty() {
          let prev = with_plus_parens.pop().unwrap();
          curr_plus_group.push(prev);
        }

        curr_plus_group.push(Plus)
      }
      Parens(inner) => {
        let sub = Parens(add_plus_parens(inner));

        if curr_plus_group.is_empty() {
          with_plus_parens.push(sub);
        } else {
          curr_plus_group.push(sub);
        }
      }
    }
  }

  if !curr_plus_group.is_empty() {
    with_plus_parens.push(Parens(curr_plus_group));
  }

  with_plus_parens
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  Num(u64),
  Plus,
  Star,
  Parens(Vec<Token>),
}

impl Token {
  fn parse(input: &str) -> Vec<Token> {
    let mut lexems = input
      .trim_end()
      .split(' ')
      .flat_map(|lexem| {
        if lexem.starts_with("(") {
          let parens_count = lexem.chars().filter(|ch| *ch == '(').count();
          let mut num_and_parens = vec!["("; parens_count];

          let mut num = vec![lexem.trim_start_matches('(')];

          num_and_parens.append(&mut num);
          num_and_parens
        } else if lexem.ends_with(")") {
          let mut num_and_parens = vec![lexem.trim_end_matches(')')];

          let parens_count = lexem.chars().filter(|ch| *ch == ')').count();
          let mut parens = vec![")"; parens_count];

          num_and_parens.append(&mut parens);
          num_and_parens
        } else {
          vec![lexem]
        }
      })
      .rev()
      .collect::<Vec<_>>();

    parse_inner(&mut lexems)
  }
}

fn parse_inner(lexems: &mut Vec<&str>) -> Vec<Token> {
  let mut result = vec![];

  while let Some(lexem) = lexems.pop() {
    match lexem {
      "+" => result.push(Plus),
      "*" => result.push(Star),
      "(" => {
        let tokens_in_parens = parse_inner(lexems);
        result.push(Parens(tokens_in_parens))
      }
      ")" => return result,
      _ => {
        let num = parse_num(lexem);
        result.push(num)
      }
    }
  }

  result
}

fn parse_num(num: &str) -> Token {
  Num(num.parse::<u64>().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let tokens = Token::parse("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(
      tokens,
      vec![
        Num(1),
        Plus,
        Parens(vec![Num(2), Star, Num(3)]),
        Plus,
        Parens(vec![Num(4), Star, Parens(vec![Num(5), Plus, Num(6)])])
      ]
    );

    let tokens = Token::parse("1 + (((2 * 3) + 4) * 5)");
    assert_eq!(
      tokens,
      vec![
        Num(1),
        Plus,
        Parens(vec![
          Parens(vec![Parens(vec![Num(2), Star, Num(3)]), Plus, Num(4)]),
          Star,
          Num(5)
        ]),
      ]
    );
  }

  #[test]
  fn part_one_solved() {
    let sample = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(solve(sample), Some(Box::new(51)));

    let sample = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(solve(sample), Some(Box::new(71)));

    let sample = "2 * 3 + (4 * 5)";
    assert_eq!(solve(sample), Some(Box::new(26)));

    let sample = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(solve(sample), Some(Box::new(437)));

    let sample = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(solve(sample), Some(Box::new(12240)));

    let sample = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(solve(sample), Some(Box::new(13632)));

  }

  #[test]
  fn part_two_solved() {
    // let sample = "1 + 2 * 3 + 4 * 5 + 6";
    // assert_eq!(solve2(sample), Some(Box::new(231)));

    // let sample = "1 + (2 * 3) + (4 * (5 + 6))";
    // assert_eq!(solve2(sample), Some(Box::new(51)));

    // let sample = "2 * 3 + (4 * 5)";
    // assert_eq!(solve2(sample), Some(Box::new(46)));

    // let sample = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    // assert_eq!(solve2(sample), Some(Box::new(1445)));

    // let sample = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    // assert_eq!(solve2(sample), Some(Box::new(669060)));

    // let sample = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    // assert_eq!(solve2(sample), Some(Box::new(23340)));

    let input = fs::read_to_string("data/18").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(8952864356993)));
  }
}
[14538300, 8996944, 9666, 722400, 822, 2772, 127020, 8640, 2196, 2407860, 82950, 218792, 3927264, 10509, 351, 184340, 3048, 47, 918, 45276, 18440, 1143650, 2688930, 6804, 1995840, 36568896, 1540, 22, 1521, 2205, 962, 10275660, 2075, 1267, 40, 705600, 477162, 10575360, 254133, 19515, 693, 15333, 1102500, 22, 147, 189600, 20162, 3970270080, 13744080, 3240, 239, 5023296, 20795544, 462, 41515232, 34, 108, 3628, 1991152800, 8280, 624, 166440, 8000, 150, 57672, 17158901760, 22616, 30018, 18256, 125309850440, 610, 44712, 95760, 2949, 36, 3049632, 327, 6451200, 2751840, 484, 9205, 39038976, 2024141130, 1869885, 7200, 130536, 675, 21870, 2032, 7371, 14750, 790, 133886802, 2475, 40320, 522765467, 484273800, 94220, 2294001, 3982981464, 16, 15631, 38412685080, 6720, 10176, 8059, 4209, 837054, 1272, 10800, 950, 48114, 3352090, 22453317, 80660, 4788510727, 122706, 6761, 775428, 4478976, 360, 86256213321, 1304, 95618880, 6337931520, 5480, 1969, 84145152, 6636995820, 1111324, 72, 17108104, 1608, 532455, 16800, 18564, 63, 96, 4493021, 50328, 105000, 341780637744, 313632, 2388, 896, 17424, 8202, 602129430, 184320, 10064, 453, 7107903, 168840, 90, 342, 665280, 349, 693, 1404, 6192, 11890045462, 796250, 14742, 2431674, 4032, 1922364416, 3888, 140809, 4584, 189503040, 332928, 4890580608, 270400, 75, 190474986240, 4224, 6300, 3234, 269280, 46216, 144, 31680, 14, 19584, 1524, 771264, 366628680, 589, 218538, 6352080, 423678, 800, 4204440, 471771, 116678016, 2106, 3190, 325780, 10546, 3780, 25500, 116760, 46556370, 41614560, 4564, 54291, 5076480, 111767040, 18849915, 43540833336000, 765, 648, 70632, 387112, 4435, 132769395, 144, 69972, 262, 384, 7630, 29660488, 518400, 104, 43472, 110824, 1398715, 148104, 34803675, 471132288, 132579, 1980, 1924560, 11433, 75, 1724800, 53856, 54432, 451, 42768, 306, 48372503, 35136, 870, 6573, 403644384, 270004, 74019, 4723920, 21744, 2904, 11701, 504, 2696193, 442260, 41184, 138230400, 81729, 60912000, 1505, 3623520, 777, 456, 31788832320, 83171340, 8424000, 22406544, 1434720, 1448, 426888, 1214271240, 398824555, 15876, 5191571, 561600, 163096776, 5433120, 66042, 62, 19800, 13200, 1703457, 32760, 20096, 199000, 930204, 16636, 5880, 4003408281600, 728, 89, 34227, 816, 23750, 2326068036864, 48, 920, 122, 561600, 423000, 668742480, 40251420, 244238, 59220, 5397840, 8424, 27955616, 318852207000, 717444, 2082696, 28800000, 12267, 37, 67392, 1512336, 353702, 969570, 8530011, 1548, 1129032, 145, 23, 31538, 46899, 4549244, 7032, 784, 192, 839749680, 1386, 592, 37170, 75735, 8, 134372, 350, 940766400, 294368256, 70, 199584, 20616, 205974, 3628800, 9450, 106404627600, 3240, 3012, 520, 406, 262148, 12713148, 144, 3200, 309582000, 784, 50048, 4424, 67813935255, 5770240, 832320, 14399, 10302484608, 337094784, 152706608208, 21088512, 6000, 1788549, 193752, 72072, 6302562575, 9360028117440, 25308460800, 294, 105, 6272]
*/