// Finds the sum of name-scores of names in a vector, multiplied by position numbers
pub fn namescoresum(names: &Vec<String>) -> usize {
    let mut sum = 0;
    let orderednames = order(names);
    println!("foo");
    for n in 0..orderednames.len() {
        sum += (n + 1) * namescore(&orderednames[n]);
    }
    sum
}

// Orders a vector of Strings alphabetically
pub fn order(names: &Vec<String>) -> Vec<String> {
    let mut cloned = names.clone();
    let mut sorted: Vec<String> = Vec::new();
    while cloned.len() > 0 {
        let mut first = "ZZZ".to_string();
        let mut posit = 0;
        let doublecloned = cloned.clone();
        for i in 0..doublecloned.len() {
            if first > doublecloned[i] { 
                first = doublecloned[i].clone();
                posit = i;
            }
        }
        sorted.push(first);
        cloned.remove(posit);
    }
    sorted
}

// Finds the name score of a name
pub fn namescore(name: &str) -> usize {
    let mut sum = 0;
    let letterscores = name.chars()
        .map(|x| letterscore(x))
        .collect::<Vec<usize>>();
    for x in &letterscores {
        sum += *x;
    }
    sum
}

// Finds the letter score of a letter
pub fn letterscore(letter: char) -> usize {
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'J' => 10,
        'K' => 11,
        'L' => 12,
        'M' => 13,
        'N' => 14,
        'O' => 15,
        'P' => 16,
        'Q' => 17,
        'R' => 18,
        'S' => 19,
        'T' => 20,
        'U' => 21,
        'V' => 22,
        'W' => 23,
        'X' => 24,
        'Y' => 25,
        'Z' => 26,
        _ => 0,
    }
}
