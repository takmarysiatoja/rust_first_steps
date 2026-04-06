use std::io;

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut current_player = 'X';

    loop {

        println!("{:?}",board[0]);
        println!("{:?}",board[1]);
        println!("{:?}",board[2]);

        println!("\nGracz {}, Twój ruch (1-9):", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Błąd odczytu");
        
        let input = input.trim().parse::<usize>();
        
        let position = if let Ok(num) = input {
            if num >= 1 && num <= 9 { num } else { 
                println!("Liczba poza zakresem!"); 
                continue; 
            }
        } else {
            println!("To nie jest liczba!");
            continue;
        };

        let row = (position - 1) / 3;
        let col = (position - 1) % 3;

        if board[row][col] != ' ' {
            println!("Pole zajęte! Spróbuj ponownie.");
            continue;
        }

        board[row][col] = current_player;

        let mut win = false;

        // sprawdzenie wygranej

        for i in 0..3 {
            if board[i][0] == current_player && board[i][1] == current_player && board[i][2] == current_player { win = true; }
            if board[0][i] == current_player && board[1][i] == current_player && board[2][i] == current_player { win = true; }
        }

        if board[0][0] == current_player && board[1][1] == current_player && board[2][2] == current_player { win = true; }
        if board[0][2] == current_player && board[1][1] == current_player && board[2][0] == current_player { win = true; }

        if win {
            println!("\nGRACZ {} WYGRYWA!", current_player);
            break;
        }

        // sprawdzenie remisu
        let mut has_empty_space = false;
        for r in 0..3 {
            for c in 0..3 {
                if board[r][c] == ' ' { has_empty_space = true; }
            }
        }
        
        if !has_empty_space {
            println!("Remis!");
            break;
        }

        if current_player == 'X' {
            current_player = 'O';
        } else {
            current_player = 'X';
        }
    }
}