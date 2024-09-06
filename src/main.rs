use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::io::{self, Write};

use log::{info, warn, LevelFilter};
use simplelog::*;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "Z",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        let suit_str = match self.suit {
            Suit::Hearts => "C",
            Suit::Diamonds => "D",
            Suit::Clubs => "T",
            Suit::Spades => "E",
        };
        write!(f, "{}{}", rank_str, suit_str)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
                cards.push(Card::new(rank, suit));
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn imprimir_grid(grid: &[Vec<Option<Card>>]) {
    println!("Cartas:");
    for (_i, row) in grid.iter().enumerate() {
        for card in row {
            match card {
                Some(c) => print!("{} ", c),
                None => print!("None "),
            }
        }
        println!(""); 
    }
    println!(" 1  2  3  4  5  6  7  8 -> Columnas"); 
}

fn imprimir_cartas_extra(remaining_cards: &[Option<Card>]) {
    println!("\nCartas extra:");
    for (i, card) in remaining_cards.iter().enumerate() {
        match card {
            Some(c) => println!("{}: {}", i + 1, c),
            None => println!("{}: None", i + 1),
        }
    }
}

fn get_input(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Entrada invalida, por favor ingrese un número!"),
        }
    }
}

fn todo_vacio(grid: &[Vec<Option<Card>>], remaining_cards: &[Option<Card>]) -> bool {
    grid.iter().all(|row| row.iter().all(|&card| card.is_none())) &&
    remaining_cards.iter().all(|&card| card.is_none())
}

fn encontrar_carta(grid: &[Vec<Option<Card>>], col: usize) -> Option<(Card, usize)> {
    for row in (0..6).rev() {
        if let Some(card) = grid[row][col] {
            return Some((card, row));
        }
    }
    None
}

fn log_cartas(grid: &[Vec<Option<Card>>]) {
    info!("Cartas:");
    for (i, row) in grid.iter().enumerate() {
        let row_str: Vec<String> = row.iter().map(|card| match card {
            Some(c) => format!("{}", c),
            None => "None".to_string(),
        }).collect();
        info!("{} | {}", row_str.join(" "), i + 1);
    }
    info!(" 1  2  3  4  5  6  7  8 -> Columnas"); 
}

fn log_cartas_extra(remaining_cards: &[Option<Card>]) {
    info!("\nRemaining cards:");
    for (i, card) in remaining_cards.iter().enumerate() {
        match card {
            Some(c) => info!("{}: {}", i + 1, c),
            None => info!("{}: None", i + 1),
        }
    }
}

fn juego(mut grid: Vec<Vec<Option<Card>>>, mut remaining_cards: Vec<Option<Card>>) {
    loop {
        
        imprimir_grid(&grid);
        log_cartas(&grid);
        imprimir_cartas_extra(&remaining_cards);
        log_cartas_extra(&remaining_cards);
        

        let col1 = get_input("Ingrese el número de la columna de la carta 1 (9 en caso de usar las extra): ") - 1;
        info!("User selected column {} for the first card.", col1 + 1);
        let col2 = get_input("Ingrese el número de la columna de la carta 2 (9 en caso de usar las extra): ") - 1;
        info!("User selected column {} for the second card.", col2 + 1);

        if col1 < 9 && col2 < 9 && col1 != col2{
            let (card1, pos1) = if col1 < 8 {
                match encontrar_carta(&grid, col1) {
                    Some((card, pos)) => (Some(card), Some(pos)),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

            let (card2, pos2) = if col2 < 8 {
                match encontrar_carta(&grid, col2) {
                    Some((card, pos)) => (Some(card), Some(pos)),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

            if card1.is_none() && col1 < 8 || card2.is_none() && col2 < 8 {
                println!("Una o ambas posiciones no poseen ninguna carta, intentelo otra vez");
                continue;
            }

            let card1 = if col1 < 8 { card1.unwrap() } else { remaining_cards[col1 - 8].unwrap() };
            let card2 = if col2 < 8 { card2.unwrap() } else { remaining_cards[col2 - 8].unwrap() };



            if card1.rank == card2.rank {

                if col1 < 8 {
                    grid[pos1.unwrap()][col1] = None;
                } else {
                    remaining_cards[col1 - 8] = None;
                }
                if col2 < 8 {
                    grid[pos2.unwrap()][col2] = None;
                } else {
                    remaining_cards[col2 - 8] = None;
                }
            } else if col1 == 8 {
                let mut matched = false;
                for (index, rem_card) in remaining_cards.iter_mut().enumerate() {
                    if rem_card.is_some() && rem_card.unwrap().rank == card2.rank {

                        remaining_cards[index] = None;
                        grid[pos2.unwrap()][col2] = None;
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    println!("Las cartas no coinciden con ninguna carta extra. Intentelo de nuevo.");
                    warn!("Las cartas no coinciden con ninguna carta extra. Intentelo de nuevo.");
                }
            } else if col2 == 8 {
                let mut matched = false;
                for (index, rem_card) in remaining_cards.iter_mut().enumerate() {
                    if rem_card.is_some() && rem_card.unwrap().rank == card1.rank {

                        remaining_cards[index] = None;
                        grid[pos1.unwrap()][col1] = None;
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    println!("Las cartas no coinciden con ninguna carta extra. Intentelo de nuevo.");
                    warn!("Las cartas no coinciden con ninguna carta extra. Intentelo de nuevo.");
                }
            } else {
                println!("Las cartas no coinciden. Intentelo de nuevo.");
                warn!("Las cartas no coinciden. Intentelo de nuevo.");
            }
        } else {
            println!("Posición inválida. Intentelo de nuevo.");
            warn!("Posición inválida. Intentelo de nuevo.");
        }

        if todo_vacio(&grid, &remaining_cards) {
            println!("¡Felicidades! Has ganado el juego.");
            info!("¡Felicidades! Has ganado el juego.");
            break;
        }
    }
}

fn main() {
    WriteLogger::init(LevelFilter::Info, Config::default(), std::fs::File::create("nestor.log").unwrap()).unwrap();

    print!("SOLITARIO NESTOR \n");
    let mut deck = Deck::new();
    deck.shuffle();

    let mut grid: Vec<Vec<Option<Card>>> = vec![vec![None; 8]; 6];
    let mut remaining_cards: Vec<Option<Card>> = vec![None; 4];

    for (i, card) in deck.cards.iter().enumerate() {
        if i < 48 {
            let row = i / 8;
            let col = i % 8;
            grid[row][col] = Some(*card);
        } else {
            remaining_cards[i - 48] = Some(*card);
        }
    }

    juego(grid, remaining_cards);
}





