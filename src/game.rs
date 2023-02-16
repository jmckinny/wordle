use colored::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    solution: String,
    guess_left: u8,
    game_won: bool,
}

#[derive(Debug)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

type LetterScore = (char, Correctness);

#[derive(Debug)]
pub struct GuessScore {
    data: Vec<LetterScore>,
}

impl Game {
    pub fn new(solution: &str) -> Self {
        Game {
            solution: solution.to_string(),
            guess_left: 6,
            game_won: false,
        }
    }

    pub fn guess(&mut self, guess: &str) -> Result<GuessScore, &str> {
        if !is_valid_word(guess) {
            return Err("Invalid guess");
        }
        self.guess_left -= 1;

        let mut correctness = Vec::new();
        let mut letter_freq: HashMap<char, u8> = HashMap::new();

        for letter in self.solution.chars() {
            if let Some(freq) = letter_freq.get_mut(&letter) {
                *freq += 1;
            } else {
                letter_freq.insert(letter, 1);
            }
        }

        for (i, letter) in guess.chars().enumerate() {
            if let Some(freq) = letter_freq.get_mut(&letter) {
                if *freq > 0 {
                    if self.solution.chars().nth(i).unwrap() == letter {
                        correctness.push((letter, Correctness::Correct));
                    } else {
                        correctness.push((letter, Correctness::Misplaced));
                    }
                }
                *freq = freq.checked_sub(1).unwrap_or(0);
            } else {
                correctness.push((letter, Correctness::Wrong));
            }
        }

        if self.solution == guess {
            self.game_won = true;
        }

        Ok(GuessScore { data: correctness })
    }

    pub fn is_game_over(&self) -> bool {
        self.guess_left == 0 || self.game_won
    }

    pub fn game_won(&self) -> bool {
        self.game_won
    }

    pub fn guesses_left(&self) -> u8 {
        self.guess_left
    }
}

impl std::fmt::Display for GuessScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (letter, correctness) in self.data.iter() {
            match correctness {
                Correctness::Correct => write!(f, "{}", letter.to_string().green())?,
                Correctness::Misplaced => write!(f, "{}", letter.to_string().yellow())?,
                Correctness::Wrong => write!(f, "{}", letter.to_string().red())?,
            }
        }
        Ok(())
    }
}

pub fn is_valid_word(word: &str) -> bool {
    let words = std::fs::read_to_string("wordlist.txt").unwrap();
    for w in words.lines() {
        if word == w {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid() {
        assert!(is_valid_word("apple"));
    }
}
