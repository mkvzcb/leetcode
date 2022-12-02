
struct Solution {}
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        todo!()
    }
}

pub fn vowel_count(string_slice: String) -> usize {
    string_slice
        .chars()
        .filter(|&x| {
            ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
                .iter()
                .find(|&&vowel| x == vowel)
                .is_some()
        })
        .count()
}

fn main() -> std::io::Result<()> {
    println!("{}", vowel_count(String::from("dupakupasrakaptaka")));
    Ok(())
}

#[cfg(test)]
mod problem_1702 {
    use crate::*;
}
