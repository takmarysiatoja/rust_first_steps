fn main() {
    println!("Podaj numer ISBN-10:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Błąd odczytu");

    let isbn = input.trim().chars().filter(|&c| c!='-').collect::<String>();

    let length = isbn.len();
    if length != 10 {
        println!("Nieprawidłowa długość numeru ISBN-10");
        return;
    }

    let mut sum = 0;

    for elem in isbn.chars().enumerate() {
        let (i, c) = elem;
        let value = if c == 'X' && i == 9 {
            10
        } else if c.is_digit(10) {
            c.to_digit(10).unwrap()
        } else {
            println!("Nieprawidłowy znak w numerze ISBN-10");
            return;
        };
        sum += value * (i as u32 + 1);
    }

    if sum % 11 == 0{
        println!("Numer ISBN-10 jest poprawny");
    } else {
        println!("Numer ISBN-10 jest niepoprawny");
    }
}
