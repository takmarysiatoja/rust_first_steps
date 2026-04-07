use rand::{rng, Rng};
use std::io;


fn generate_password(length: usize, charsets: &[&str]) -> String {
    let mut pool: Vec<char> = Vec::new();

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digits = false;
    let mut has_special = false;

    for &wybor in charsets {
        match wybor {
            "lowercase" => has_lower = true,
            "uppercase" => has_upper = true,
            "digits" => has_digits = true,
            "special" => has_special = true,
            _ => {} 
        }
    }


    if !has_lower && !has_upper && !has_digits && !has_special {
        has_lower = true;
        has_upper = true;
        has_digits = true;
        has_special = true;
    }


    if has_lower {
        pool.extend("abcdefghijklmnopqrstuvwxyz".chars());
    }
    if has_upper {
        pool.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
    }
    if has_digits {
        pool.extend("0123456789".chars());
    }
    if has_special {
        pool.extend("!@#$%^&*()_+-=[]{}|;:,.<>?".chars());
    }


    let mut rng = rng();
    let mut password = String::new();


    for _ in 0..length {

        let index = rng.random_range(0..pool.len());

        password.push(pool[index]);
    }


    password
}

fn main() {
    println!("Podaj długość hasła:");
    

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu");
    
    let length: usize = input.trim().parse().expect("Proszę wpisać prawidłową liczbę!");

    println!("\nOto Twoje 5 wygenerowanych haseł:");
    for _ in 0..5 {
        let haslo = generate_password(length, &["lowercase", "uppercase", "digits", "special"]);
        println!("{}", haslo);
    }
}

//testy
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let haslo = generate_password(12, &[]);
        assert_eq!(haslo.len(), 12);
    }

    #[test]
    fn test_password_charsets() {
        let haslo = generate_password(50, &["digits"]);

        for znak in haslo.chars() {
            assert!(znak.is_ascii_digit(), "Znak {} nie jest cyfrą!", znak);
        }
    }
}