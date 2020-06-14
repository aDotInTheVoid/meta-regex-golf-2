use itertools::Itertools;
use std::borrow::Cow;
use std::str::pattern::{Pattern as _, Searcher};

const START: u8 = b'^';
const DOT: u8 = b'.';
const END: u8 = b'$';

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Regex {
    binds: Binds,
    pattern: Pattern,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
enum Binds {
    Front,
    Back,
    Both,
    Neither,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
enum Pattern {
    NoDots(String),
    Dots(String),
    // String is the litteral
    // First usize is leading dots
    // Secound is trailing dots
    //DotsLit(String, usize, usize),
}

impl Pattern {
    fn len(&self) -> usize {
        match self {
            Self::Dots(x) => x.len(),
            Self::NoDots(x) => x.len(),
            //Self::DotsLit(x, front, back) => x.len() + front + back,
        }
    }

    fn str(&self) -> &str {
        match self {
            Self::Dots(x) => x,
            Self::NoDots(x) => x
            //Self::DotsLit(x, front, back) => {
            //    Cow::Owned(format!("{}{}{}", ".".repeat(*front), x, ".".repeat(*back)))
            //}
        }
    }

    fn as_bytes<'a>(&'a self) -> &[u8] {
        match self {
            Self::Dots(x) => x.as_bytes(),
            Self::NoDots(x) => x.as_bytes(),
            _ => unreachable!()
        }
    }
}

impl Regex {

    pub fn new(input: String) -> Self {
        // TODO: Allow empty string to work

        // Check for ^ and $ in regex
        let has_start = input.as_bytes()[0] == START;
        let has_end = input.as_bytes().last().map(|&x| x == END).unwrap_or(false);

        // Get indexes to strip out anchors
        let start_idx = if has_start { 1 } else { 0 };
        let end_idx = if has_end {
            input.len() - 1
        } else {
            input.len()
        };

        // Calculate binds
        let binds = match (has_start, has_end) {
            (true, true) => Binds::Both,
            (true, false) => Binds::Front,
            (false, true) => Binds::Back,
            (false, false) => Binds::Neither,
        };

        // Remove anchors
        let pattern_range = &input[start_idx..end_idx];
        let pattern = if input.as_bytes().contains(&DOT) {
            // let lit_idx = pattern_range
            //     .as_bytes()
            //     .iter()
            //     .map(|x| *x != b'.')
            //     .enumerate()
            //     .filter(|(_, x)| *x) // Remove non lits
            //     .map(|(x, _)| x)
            //     .collect_vec();

            // if binds == Binds::Neither
            //     && !lit_idx.is_empty()
            //     && lit_idx
            //         .iter() // Get index's of lits
            //         .tuple_windows()
            //         .map(|(x, y)| y - x == 1)
            //         .all(|x| x)
            // {
            //     Pattern::DotsLit(
            //         pattern_range[lit_idx[0]..=*lit_idx.last().unwrap()].to_owned(),
            //         lit_idx[0],
            //         pattern_range.len() - lit_idx.last().unwrap() - 1,
            //     )
            //     // Pattern::Dots(pattern_range.to_owned())
            // } else {
            Pattern::Dots(pattern_range.to_owned())
        // }
        } else {
            Pattern::NoDots(pattern_range.to_owned())
        };

        Self { binds, pattern }
    }

    #[cfg(test)]

    pub fn new_clone(input: &str) -> Self {
        Self::new(input.to_owned())
    }

    pub fn is_match(&self, text: &str) -> bool {
        let (start, end) = match self.binds {
            // Front Bind's we match 0..pattern len
            // Eg with neadle `^abc` and haystack `xyx...`,
            // we only need to look at `xyz`
            Binds::Front => (0, self.pattern.len()),
            Binds::Back => (
                match text.len().checked_sub(self.pattern.len()) {
                    Some(x) => x,
                    None => return false,
                },
                text.len(),
            ),
            Binds::Both => {
                if text.len() == self.pattern.len() {
                    return self.match_knows_pos(text);
                } else {
                    return false;
                }
            }
            Binds::Neither => return self.match_unknown_pos(text),
        };
        text.get(start..end)
            .map(|x| self.match_knows_pos(x))
            .unwrap_or(false)
    }

    pub fn cost(&self) -> usize {
        (match self.binds {
            Binds::Front | Binds::Back => 1,
            Binds::Both => 2,
            Binds::Neither => 0,
        }) + self.pattern.len()
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            if matches!(self.binds, Binds::Front | Binds::Both) {
                "^"
            } else {
                ""
            },
            self.pattern.str(),
            if matches!(self.binds, Binds::Back | Binds::Both) {
                "$"
            } else {
                ""
            },
        )
    }

    fn match_knows_pos(&self, text: &str) -> bool {
        match &self.pattern {
            Pattern::NoDots(x) => x == text,
            Pattern::Dots(_) => self.match_dots_pos(text),
            //Pattern::DotsLit(_, _, _) => unreachable!(),
        }
    }

    fn match_unknown_pos(&self, text: &str) -> bool {
        match &self.pattern {
            Pattern::NoDots(x) => text.contains(x),
            Pattern::Dots(_) => self.match_dots_pos_unknown(text),
            //Pattern::DotsLit(lit, start, end) => Self::match_dots_lit(lit, *start, *end, text),
        }
    }

    fn match_dots_lit(lit: &str, start: usize, end: usize, text: &str) -> bool {
        panic!();
        let mut searcher = lit.into_searcher(text);
        while let Some((start_idx, end_idx)) = searcher.next_match() {
            if start_idx >= start && end_idx <= text.len() - end {
                return true;
            }
        }
        false
    }

    fn match_dots_pos(&self, text: &str) -> bool {
        debug_assert_eq!(self.pattern.len(), text.len());

        for (pat, txt) in self.pattern.as_bytes().iter().zip(text.as_bytes()) {
            if *pat == DOT {
                continue;
            } else if pat != txt {
                return false;
            }
        }
        true
    }

    fn match_dots_pos_unknown(&self, text: &str) -> bool {
        if text.len() < self.pattern.len() {
            return false;
        }

        for i in 0..=text.len() - self.pattern.len() {
            if self.match_dots_pos(&text[i..i + self.pattern.len()]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let test_1 = "^win$";
        let ans_1 = Regex {
            binds: Binds::Both,
            pattern: Pattern::NoDots("win".to_string()),
        };

        let test_2 = "^win";
        let ans_2 = Regex {
            binds: Binds::Front,
            pattern: Pattern::NoDots("win".to_string()),
        };

        let test_3 = "^wi.";
        let ans_3 = Regex {
            binds: Binds::Front,
            pattern: Pattern::Dots("wi.".to_string()),
        };

        let test_4 = "wi.";
        let ans_4 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("wi".to_string(), 0, 1),
        };

        let test_5 = "wi";
        let ans_5 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::NoDots("wi".to_string()),
        };

        let test_6 = "^wi";
        let ans_6 = Regex {
            binds: Binds::Front,
            pattern: Pattern::NoDots("wi".to_string()),
        };

        let test_7 = "win$";
        let ans_7 = Regex {
            binds: Binds::Back,
            pattern: Pattern::NoDots("win".to_string()),
        };

        let test_8 = "win";
        let ans_8 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::NoDots("win".to_string()),
        };

        let test_9 = "wi.$";
        let ans_9 = Regex {
            binds: Binds::Back,
            pattern: Pattern::Dots("wi.".to_string()),
        };

        let test_10 = "a";
        let ans_10 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::NoDots("a".to_string()),
        };

        let test_11 = ".x";
        let ans_11 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("x".to_string(), 1, 0),
        };

        let test_12 = "..x";
        let ans_12 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("x".to_string(), 2, 0),
        };

        let test_13 = ".x.";
        let ans_13 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("x".to_string(), 1, 1),
        };

        let test_14 = "abc..";
        let ans_14 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("abc".to_string(), 0, 2),
        };

        let test_15 = "..abc..";
        let ans_15 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("abc".to_string(), 2, 2),
        };

        let test_16 = "ab..";
        let ans_16 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::DotsLit("ab".to_string(), 0, 2),
        };

        let test_17 = "w..n";
        let ans_17 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::Dots("w..n".to_string()),
        };

        let test_18 = "...";
        let ans_18 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::Dots("...".to_string()),
        };

        assert_eq!(Regex::new_clone(test_1), ans_1);
        assert_eq!(Regex::new_clone(test_2), ans_2);
        assert_eq!(Regex::new_clone(test_3), ans_3);
        assert_eq!(Regex::new_clone(test_4), ans_4);
        assert_eq!(Regex::new_clone(test_5), ans_5);
        assert_eq!(Regex::new_clone(test_6), ans_6);
        assert_eq!(Regex::new_clone(test_7), ans_7);
        assert_eq!(Regex::new_clone(test_8), ans_8);
        assert_eq!(Regex::new_clone(test_9), ans_9);
        assert_eq!(Regex::new_clone(test_10), ans_10);
        assert_eq!(Regex::new_clone(test_11), ans_11);
        assert_eq!(Regex::new_clone(test_12), ans_12);
        assert_eq!(Regex::new_clone(test_13), ans_13);
        assert_eq!(Regex::new_clone(test_14), ans_14);
        assert_eq!(Regex::new_clone(test_15), ans_15);
        assert_eq!(Regex::new_clone(test_16), ans_16);
        assert_eq!(Regex::new_clone(test_17), ans_17);
        assert_eq!(Regex::new_clone(test_18), ans_18);
    }

    macro_rules! reg_text {
        ($regex:expr, [$($accpet:expr),*],  [$($regect:expr),*]) => {
            let re = Regex::new_clone($regex);
            $(
                assert!(re.is_match($accpet));
            )*
            $(
                assert!(!re.is_match($regect));
            )*
        };
    }

    #[test]
    fn cost_and_string() {
        for i in &[
            "^win$", "^win", "^wi.", "wi.", "wi", "^wi", "win$", "win", "wi.$",
        ] {
            let reg = Regex::new_clone(i);
            assert_eq!(reg.cost(), i.len());
            assert_eq!(&&reg.to_string(), i);
        }
    }

    #[test]
    fn no_dots() {
        reg_text!("^win$", ["win"], ["", "winn", "wwin", "wi", "in", "banana"]);
        reg_text!(
            "^win",
            ["win", "win ", "windows XP"],
            [" win", "xw", "wi", "in", ""]
        );
        reg_text!(
            "wi",
            ["wi", "win", "pinwion", "Mario Kart wii"],
            ["w i", "we", "w"]
        );
        reg_text!(
            "win$",
            ["win", "xd win", "how to win"],
            ["banh", "win95", "n"]
        );
    }

    #[test]
    fn dots_known_pos() {
        reg_text!("^wi.", ["win", "wix", "windows 2000"], ["wi", "won"]);
        reg_text!("^w.n$", ["win", "wan", "wxn"], ["winn", " win ", "xin"]);
        reg_text!("..n$", ["win", "..n", "pin", "impl pin", "xdin"], ["in"]);
    }

    #[test]
    fn dots_unknown_pos() {
        reg_text!(
            "w.n",
            [" win", "wxnxxx", "win", "winwinwin", "dsfawxndf"],
            ["wnwn", "dsasdf", "asdf", ""]
        );
        reg_text!(
            "..x..",
            ["  x  ", "xxxxx"],
            ["sfsdfdx", "vxdfs", "asdfdsxd"]
        );

        reg_text!(
            "x..",
            ["x  ", "xzy", "zxyy", "assdxddd", "x  x"],
            ["xx", "", "abcx", "ddxd"]
        );
        reg_text!(
            "...abc",
            ["abcabc", "xxxabc", "xxxabcxxx", "xxabcabc"],
            ["xxabcxxx", "x"]
        );
        reg_text!(
            "..abc...",
            ["xxabcxxx", "xxabcabc"],
            ["xabcxabc", "xabcxxx", "xxabcxx"]
        );
    }
}
