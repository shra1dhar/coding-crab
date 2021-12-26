use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let to_lowercase_s = |c: char| {
        if !c.is_lowercase() {c.to_lowercase().to_string()} else {c.to_string()}
    };


    let lw: Vec<_> = word.chars().map(to_lowercase_s).collect();
    let mut sc: Vec<_> = word.chars().map(to_lowercase_s).collect();
    sc.sort_unstable();


    let result = possible_anagrams.iter()
        .filter(|w| {
            let mut sci: Vec<_> = w.chars().map(to_lowercase_s).collect();
            let same_word = lw == sci;

            sci.sort_unstable();
            sci == sc && !same_word
        }).map(|&w| w);
    HashSet::from_iter(result)
}

