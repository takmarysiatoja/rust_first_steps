fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {
    // wiersze
    for r in 0..9 {
        let mut seen = [false; 10];
        for c in 0..9 {
            let val = board[r][c];
            if val == 0 { continue; }
            if val > 9 || seen[val as usize] {
                return false;
            }
            seen[val as usize] = true;
        }
    }

    // kolumny
    for c in 0..9 {
        let mut seen = [false; 10];
        for r in 0..9 {
            let val = board[r][c];
            if val == 0 { continue; }
            if val > 9 || seen[val as usize] {
                return false;
            }
            seen[val as usize] = true;
        }
    }

    // kwadraty 3x3
    for row_start in (0..9).step_by(3) {
        for col_start in (0..9).step_by(3) {
            
            let mut seen = [false; 10];

            for r in 0..3 {
                for c in 0..3 {
                    let val = board[row_start + r][col_start + c];
                    if val == 0 { continue; }
                    if val > 9 || seen[val as usize] {
                        return false; 
                    }
                    seen[val as usize] = true;
                }
            }
            
        }
    }

    true 
}

fn main() {
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];

    if check_sudoku_board(board) {
        println!("Plansza jest poprawna!");
    } else {
        println!("Plansza zawiera błędy.");
    }
}