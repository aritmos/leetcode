// Ransom Note
use crate::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        const NUM_LETTERS: usize = (b'z' - b'a' + 1) as usize;
        const FIRST_LETTER: u8 = b'a';

        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut magazine_letters = [0; NUM_LETTERS];
        magazine.bytes().for_each(|letter| {
            magazine_letters[(letter - FIRST_LETTER) as usize] += 1;
        });

        for letter in ransom_note.bytes() {
            let index = (letter - FIRST_LETTER) as usize;
            match magazine_letters[index] {
                0 => return false,
                _ => magazine_letters[index] -= 1,
            }
        }
        true
    }
}
