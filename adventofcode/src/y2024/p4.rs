use crate::futils::read_file;

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut inp: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| {
        inp.push(l.chars().collect());
    });

    return inp;
}

fn is_xmas(word: &str) -> bool {
    word == "XMAS" || word == "SAMX"
}

fn xmax_counter(input: &str) -> i32 {
    let mut count = 0;
    let v: Vec<Vec<char>> = parse(input);

    println!("{}", input);

    // Check for horizontal vertical diagonal XMAS SAMX
    // This all could have been done in one loop but writing like this seemed faster (at first)

    // horizontal
    for i in 0..v.len() {
        for j in 0..(v[0].len() - 3) {
            let word = String::from_iter(v[i][j..(j + 4)].iter());
            if is_xmas(&word) {
                count += 1;
            }
        }
    }

    // vertical
    for i in 0..v[0].len() {
        for j in 0..(v.len() - 3) {
            let mut word = String::new();
            for k in 0..4 {
                word.push(v[j + k][i]);
            }
            if is_xmas(&word) {
                count += 1;
            }
        }
    }

    // diagonal (2 directions / \)
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            let mut word = String::new();
            for k in 0..4 {
                if i + k >= v.len() || j + k >= v[0].len() {
                    break;
                }
                word.push(v[i + k][j + k]);
            }
            let mut word2 = String::new();
            for k in 0..4 {
                if i.checked_sub(k) == None || j + k >= v[0].len() {
                    break;
                }
                word2.push(v[i - k][j + k]);
            }
            if is_xmas(&word) {
                count += 1;
            }
            if is_xmas(&word2) {
                count += 1;
            }
        }
    }
    count
}

// This problems context makes naming functions impossible
fn is_valid_x_mass(v: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    // M S
    //  A
    // M S
    // Check if char at position is A
    let (x, y) = pos;
    if v[x][y] != 'A' {
        return false;
    } else {
        if x >= 1 && y >= 1 && x < v.len() - 1 && y < v[0].len() - 1 {
            let tl = v[x - 1][y - 1];
            let tr = v[x - 1][y + 1];
            let bl = v[x + 1][y - 1];
            let br = v[x + 1][y + 1];

            // There are only 2 base variations only possible with swappable M & S
            if ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
                && ((bl == 'M' && tr == 'S') || (bl == 'S' && tr == 'M'))
            {
                return true;
            }
        }
    }
    false
}

fn mas_counter(input: &str) -> i32 {
    let mut count = 0;
    let v: Vec<Vec<char>> = parse(input);

    for i in 0..v.len() {
        for j in 0..v[0].len() {
            if is_valid_x_mass(&v, (i, j)) {
                count += 1;
            }
        }
    }

    count
}

pub fn solution() {
    let contents = read_file("src/y2024/p4.input.txt").expect("Failed to read input file");
    let a1 = xmax_counter(&contents);
    let a2 = mas_counter(&contents);
    println!("Answer 1: {}", a1);
    println!("Answer 2: {}", a2);
}
