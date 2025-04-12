pub fn score(s: &str) -> u64 {
    let mut tot_score: u64 = 0;
    for c in s.chars() {
        match c.to_uppercase().next(){
            Some(letter) => match letter {
               'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => tot_score += 1,
                'D' | 'G' => tot_score += 2,
                'B' | 'C' | 'M' | 'P' => tot_score += 3,
                'F' | 'H' | 'V' | 'W' | 'Y' => tot_score += 4,
                'K' => tot_score += 5,
                'J' | 'X' => tot_score += 8,
                'Q' | 'Z' => tot_score += 10,
                _ => {} // Ignore other characters 
            }
            None => {}
        }
    }
    tot_score
}