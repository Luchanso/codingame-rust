use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let rounds = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let cash = parse_input!(input_line, i32);
    let mut play_rounds: Vec<Play> = Vec::new();

    for i in 0..rounds as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let play = input_line.trim_end().to_string();
        let mut play_iterator = play.split_ascii_whitespace();
        let ball: i32 = play_iterator.next().unwrap().parse().unwrap();
        let call: Call = match play_iterator.next().unwrap() {
            "EVEN" => Call::Even,
            "ODD" => Call::Odd,
            "PLAIN" => Call::Plain,
            _ => panic!(),
        };

        let target = match play_iterator.next() {
            Some(val) => Some(val.parse::<i32>().unwrap()),
            _ => None,
        };

        play_rounds.push(Play { ball, call, target })
    }

    eprintln!("{:?}", play_rounds);

    println!("{}", solve(cash, play_rounds));
}

#[derive(Debug)]
pub enum Call {
    Even,
    Odd,
    Plain,
}

#[derive(Debug)]
pub struct Play {
    ball: i32,
    call: Call,
    target: Option<i32>,
}

pub fn solve(cash: i32, rounds: Vec<Play>) -> i32 {
    let mut result = cash;
    const WIN_CASH: i32 = 100;
    const LOSE_CASH: i32 = -100;
    const STREAK_CASH: i32 = 3500;

    for round in rounds {
        result = result
            + match round.call {
                Call::Even => {
                    if round.ball % 2 == 0 {
                        WIN_CASH
                    } else {
                        LOSE_CASH
                    }
                }
                Call::Odd => {
                    if round.ball % 2 != 0 {
                        WIN_CASH
                    } else {
                        LOSE_CASH
                    }
                }
                Call::Plain => {
                    if round.ball == round.target.unwrap() {
                        STREAK_CASH
                    } else {
                        LOSE_CASH
                    }
                }
            }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_correct_work() {
        assert_eq!(
            solve(
                1000,
                vec![Play {
                    ball: 10,
                    call: Call::Even,
                    target: None
                }]
            ),
            1100
        );
        assert_eq!(
            solve(
                1000,
                vec![Play {
                    ball: 9,
                    call: Call::Even,
                    target: None
                }]
            ),
            900
        );
    }

    #[test]
    fn odd_correct_work() {
        assert_eq!(
            solve(
                1000,
                vec![Play {
                    ball: 10,
                    call: Call::Odd,
                    target: None
                }]
            ),
            900
        );
        assert_eq!(
            solve(
                1000,
                vec![Play {
                    ball: 9,
                    call: Call::Odd,
                    target: None
                }]
            ),
            1100
        );
    }

    #[test]
    fn plain_correct_work() {
        assert_eq!(
            solve(
                1000,
                vec![Play {
                    ball: 10,
                    call: Call::Plain,
                    target: Some(10)
                }]
            ),
            4500
        );
        assert_eq!(
            solve(
                1000,
                vec![Play {
                    ball: 9,
                    call: Call::Plain,
                    target: Some(10)
                }]
            ),
            900
        );
    }

    #[test]
    fn complex_test() {
        let rounds = vec![
            Play {
                ball: 31,
                call: Call::Plain,
                target: Some(30),
            },
            Play {
                ball: 18,
                call: Call::Plain,
                target: Some(35),
            },
            Play {
                ball: 14,
                call: Call::Plain,
                target: Some(32),
            },
            Play {
                ball: 25,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 13,
                call: Call::Plain,
                target: Some(9),
            },
            Play {
                ball: 14,
                call: Call::Plain,
                target: Some(34),
            },
            Play {
                ball: 32,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 26,
                call: Call::Plain,
                target: Some(9),
            },
            Play {
                ball: 29,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 7,
                call: Call::Plain,
                target: Some(21),
            },
            Play {
                ball: 32,
                call: Call::Plain,
                target: Some(29),
            },
            Play {
                ball: 0,
                call: Call::Plain,
                target: Some(7),
            },
            Play {
                ball: 7,
                call: Call::Plain,
                target: Some(34),
            },
            Play {
                ball: 13,
                call: Call::Plain,
                target: Some(14),
            },
            Play {
                ball: 22,
                call: Call::Plain,
                target: Some(8),
            },
            Play {
                ball: 25,
                call: Call::Plain,
                target: Some(28),
            },
            Play {
                ball: 11,
                call: Call::Plain,
                target: Some(20),
            },
            Play {
                ball: 14,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 23,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 13,
                call: Call::Plain,
                target: Some(22),
            },
            Play {
                ball: 2,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 23,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 17,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 30,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 28,
                call: Call::Plain,
                target: Some(28),
            },
            Play {
                ball: 5,
                call: Call::Plain,
                target: Some(36),
            },
            Play {
                ball: 13,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 22,
                call: Call::Plain,
                target: Some(11),
            },
            Play {
                ball: 5,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 32,
                call: Call::Plain,
                target: Some(25),
            },
            Play {
                ball: 13,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 10,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 28,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 15,
                call: Call::Plain,
                target: Some(2),
            },
            Play {
                ball: 33,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 29,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 1,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 19,
                call: Call::Plain,
                target: Some(12),
            },
            Play {
                ball: 0,
                call: Call::Plain,
                target: Some(34),
            },
            Play {
                ball: 24,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 16,
                call: Call::Plain,
                target: Some(36),
            },
            Play {
                ball: 4,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 35,
                call: Call::Plain,
                target: Some(13),
            },
            Play {
                ball: 14,
                call: Call::Plain,
                target: Some(34),
            },
            Play {
                ball: 30,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 13,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 29,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 7,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 18,
                call: Call::Plain,
                target: Some(20),
            },
            Play {
                ball: 33,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 24,
                call: Call::Plain,
                target: Some(28),
            },
            Play {
                ball: 34,
                call: Call::Plain,
                target: Some(34),
            },
            Play {
                ball: 33,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 32,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 10,
                call: Call::Even,
                target: None,
            },
            Play {
                ball: 13,
                call: Call::Odd,
                target: None,
            },
            Play {
                ball: 35,
                call: Call::Plain,
                target: Some(26),
            },
        ];

        assert_eq!(solve(70545, rounds), 1153)
    }
}
