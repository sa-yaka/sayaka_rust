#[derive(Debug)]
enum PokerSuit {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn main() {
    let card1 = PokerSuit::Clubs(12);
    let card2 = PokerSuit::Spades(12);
    let card3 = PokerSuit::Diamonds(12);
    let card4 = PokerSuit::Hearts(12);

    print_suit(card1);
    print_suit(card2);
    print_suit(card3);
    print_suit(card4);
    // println!("{:?}",card1);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}
