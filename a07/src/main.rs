use regex::Regex;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    value: char,
}

impl Card {
    fn to_int_value(&self) -> u32 {
        return match self.value.to_digit(10) {
            None => {
                match self.value {
                    'T' => 10,
                    // 'J' => 11,
                    'J' => 1, // for part 2
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("weird card {}", self.value),
                }
            }
            Some(v) => v,
        };
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int_value().cmp(&other.to_int_value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    value: Vec<Card>,
}

impl Hand {
    fn to_int_value(&self) -> u32 {
        let mut card_counts = self.value.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.value).or_insert(0) += 1;
            acc
        });

        // are we doing part 2 and do we have Jokers?
        let joker = Card { value: 'J' };
        if joker.to_int_value() == 1 && self.value.contains(&joker) {
            // replace jokers with the most frequest card (except J)
            let mut mfc = 'J';
            let mut most = 0;
            for (c, cnt) in card_counts.iter() {
                if c == &'J' {
                    continue;
                }

                if cnt > &most {
                    mfc = *c;
                    most = *cnt;
                } else if cnt == &most {
                    // if there are 2 with the same frequency, use the most valuable card
                    let most_card = Card { value: mfc };
                    let card = Card { value: *c };

                    if card.gt(&most_card) {
                        mfc = *c;
                    }
                }
            }

            // if joker is the only card (JJJJJ), do nothing
            if mfc != 'J' {
                let mut new_hand_value = self.value.iter().map(|c| c.value).collect::<String>();
                new_hand_value = new_hand_value.replace('J', mfc.to_string().as_str());
                let new_hand = Hand {
                    value: new_hand_value
                        .chars()
                        .map(|c| Card { value: c })
                        .collect::<Vec<Card>>(),
                };

                card_counts = new_hand.value.iter().fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c.value).or_insert(0) += 1;
                    acc
                });
            }
        }

        let unique_cnt = card_counts.len();

        // XXXXX
        if unique_cnt == 1 {
            return 6;
        }

        let cnts = card_counts.iter().map(|(_k, v)| *v).collect::<Vec<u32>>();

        // XXXXA
        // XXXYY
        if unique_cnt == 2 {
            if cnts[0] == 4 || cnts[1] == 4 {
                return 5;
            } else if cnts[0] == 3 || cnts[0] == 2 || cnts[1] == 3 || cnts[1] == 2 {
                return 4;
            }
        }

        // XXXAB
        // XXYYA
        if unique_cnt == 3 {
            if cnts[0] == 3 || cnts[1] == 3 || cnts[2] == 3 {
                return 3;
            }

            if cnts[0] == 2 || cnts[1] == 2 || cnts[2] == 2 {
                return 2;
            }
        }

        // XXABC
        if unique_cnt == 4 {
            return 1;
        }

        // ABCDE
        0
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = self.to_int_value();
        let other_value = other.to_int_value();

        if self_value.eq(&other_value) {
            for (i, c) in self.value.iter().enumerate() {
                if !c.eq(&other.value[i]) {
                    return c.cmp(&other.value[i]);
                }
            }

            return Equal;
        }

        self_value.cmp(&other_value)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re = Regex::new("([^\\s]+)").unwrap();

    let mut cards = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let cols = re
                    .find_iter(&line)
                    .map(|m| m.as_str())
                    .collect::<Vec<&str>>();

                cards.push((
                    Hand {
                        value: cols[0]
                            .chars()
                            .map(|c| Card { value: c })
                            .collect::<Vec<Card>>(),
                    },
                    cols[1].parse::<u32>().unwrap(),
                ));
            }
            Err(_) => break,
        }
    }

    cards.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = 0;
    for (i, (_c, bid)) in cards.iter().enumerate() {
        sum += (i as u32 + 1) * bid;
    }

    println!("Result: {}", sum);

    Ok(())
}
