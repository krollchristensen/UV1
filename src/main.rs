use std::io; // Importér input/output funktioner fra standardbiblioteket

fn main() {
    // Opgave 1: Introduktion til variabler og datatyper
    // Opretter en immutable variabel til at gemme dit navn
    let name = String::from("Dit navn her"); // String::from bruges til dynamiske tekststrenge

    // Opretter en mutable variabel til at gemme din alder
    let mut age: i32 = 25; // i32 er standard heltalstypen i Rust

    // Udskriv navn og alder
    println!("Mit navn er {} og jeg er {} år gammel", name, age);

    // Opgave 2: Brugerinput
    let mut name_input = String::new(); // Opret en tom String til at gemme brugerens navn
    println!("Indtast dit navn:");

    // Læs brugerens input og håndter potentielle fejl
    io::stdin().read_line(&mut name_input).expect("Fejl ved læsning af navn");

    let mut age_input = String::new(); // Opret en String til brugerens alder
    println!("Indtast din alder:");
    io::stdin().read_line(&mut age_input).expect("Fejl ved læsning af alder");

    // Konverter alderen fra String til i32
    let age_input: i32 = age_input.trim().parse().expect("Indtast et gyldigt tal");

    // Udskriv navnet og alderen
    println!("Hej, {}! Du er {} år gammel.", name_input.trim(), age_input);

    // Opgave 3: Betingelser (if/else)
    if age_input >= 18 {
        println!("Du er voksen.");
    } else {
        println!("Du er mindreårig.");
    }

    // Opgave 4: Løkker (while og for)
    // While-løkke: tæller fra 1 til 10
    let mut count = 1;
    while count <= 10 {
        println!("Tæller: {}", count);
        count += 1;
    }

    // For-løkke: itererer over et array
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("For-løkke tal: {}", number);
    }

    // Opgave 5: Brug af match til kontrolflow
    let mut day = String::new();
    println!("Indtast et tal fra 1 til 7:");
    io::stdin().read_line(&mut day).expect("Fejl ved læsning");

    let day: i32 = day.trim().parse().expect("Indtast et gyldigt tal");

    match day {
        1 => println!("Mandag"),
        2 => println!("Tirsdag"),
        3 => println!("Onsdag"),
        4 => println!("Torsdag"),
        5 => println!("Fredag"),
        6 => println!("Lørdag"),
        7 => println!("Søndag"),
        _ => println!("Ugyldigt tal"), // Håndterer alle ikke-valide input
    }

    // Opgave 6: Brug af funktioner
    // Kalder funktionen sum
    let result = sum(5, 10);
    println!("Summen af 5 og 10 er: {}", result);

    // Opgave 7: Fejlhåndtering med Result type
    let mut input = String::new();
    println!("Indtast et tal:");

    io::stdin().read_line(&mut input).expect("Fejl ved læsning");

    let result: Result<i32, _> = input.trim().parse();

    match result {
        Ok(number) => println!("Du indtastede tallet: {}", number),
        Err(_) => println!("Det er ikke et gyldigt tal."),
    }

    // Opgave 8: Arrays og vektorer
    let numbers_array = [10, 20, 30, 40, 50];
    for number in numbers_array.iter() {
        println!("Array værdi: {}", number);
    }

    let mut numbers_vector = Vec::new();
    numbers_vector.push(10);
    numbers_vector.push(20);
    numbers_vector.push(30);

    for number in numbers_vector.iter() {
        println!("Vektor værdi: {}", number);
    }
}

// Opgave 6: Funktion der tager to heltal og returnerer deres sum
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
