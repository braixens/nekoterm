#![allow(unused)]
use rand::Rng;
use std::fmt;
use std::thread;
#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = match self {
            Suit::Clubs => "♣)",
            Suit::Diamonds => "♦)",
            Suit::Hearts => "♥)",
            Suit::Spades => "♠)",
        };
        write!(f, "{}", suit)
    }
}

#[derive(Debug)]
pub enum CardType{
    Numbered(u8),
    Face(Face),
    Ace,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Numbered(number) => write!(f, "({}", number),
            CardType::Face(face) => write!(f, "({}", face),
            CardType::Ace => write!(f, "(A"),

        }
    }
}
#[derive(Debug)]
pub enum Face {
    King,
    Queen,
    Jack,
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let face = match self {
            Face::King => "K",
            Face::Queen => "Q",
            Face::Jack => "J",
        };
        write!(f, "{}", face)
    }
}

#[derive(Debug)]
pub struct Card {
    card_type: CardType,
    suit: Suit,
}
impl Card {
    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for number in 2..=10 {
                cards.push(Card {
                    card_type: CardType::Numbered(number),
                    suit: *suit,
                })
            }
        }

        for suit in &[Suit::Spades, Suit::Hearts, Suit:: Diamonds, Suit::Clubs]{
            cards.push(Card{
                card_type: CardType::Face(Face::King),
                suit: *suit,
            });
            cards.push(Card{
                card_type: CardType::Face(Face::Queen),
                suit: *suit,
            });
            cards.push(Card{
                card_type: CardType::Face(Face::Jack),
                suit: *suit,
            });
        }

        for suit in &[Suit::Spades, Suit::Hearts, Suit:: Diamonds, Suit::Clubs] {
            cards.push(Card{
                card_type: CardType::Ace,
                suit: *suit,
            })
        }
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card>{
        if !self.cards.is_empty() {
            self.cards.pop()
        } else {
            println!("There are no cards left...");
            None
        }
    }

    pub fn shuffle(&mut self) {
        for i in (1..self.cards.len()).rev(){
            let mut rng = rand::rng();
            let j = rng.random_range(0..=i);
            self.cards.swap(i, j);
        }

    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
    hand_value: i32,
}
impl Player {
    pub fn new(name: String) -> Self{
        Player {
            name,
            hand: Vec::new(),
            hand_value: 0,
        }
    }

    pub fn draw_card(&mut self, deck: &mut Deck){
        if let Some(card) = deck.draw(){
            self.hand.push(card);
            self.calculate_hand();
            if self.is_busted() {

            }
        }
    }

    pub fn calculate_hand(&mut self) -> i32 {
        self.hand_value = 0;
        let mut ace_count = 0;
        for card in &self.hand {
            match card.card_type {
                CardType::Numbered(num) => self.hand_value += num as i32,
                CardType::Face(_) => self.hand_value += 10,
                CardType::Ace => {
                    self.hand_value += 11;
                    ace_count += 1;
                }
            }
        }
        while self.hand_value > 21 && ace_count > 0 {
            self.hand_value -= 10;
            ace_count -= 1
        }
        self.hand_value
    }

    pub fn is_busted(&self) -> bool {
        self.hand_value > 21
    }

    pub fn has_blackjack(&self) -> bool {
        self.hand_value == 21 && self.hand.len() == 2
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }
    pub fn hand_value(&self) -> i32 {
        self.hand_value
    }
}

#[derive(Debug)]
pub struct Game {
    dealer: Player,
    players: Vec<Player>,
    deck: Deck,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        Game {
            dealer: Player::new("Dealer".to_string()),
            players: Vec::new(),
            deck,
            game_over: false,
        }
    }
    pub fn single_player(name: String) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let mut game = Self {
            dealer: Player::new("Dealer".to_string()),
            players: vec![
                Player::new(name),
            ],
            deck,
            game_over: false,
        };
        game.dealer.draw_card(&mut game.deck);
        for player in &mut game.players {
            player.draw_card(&mut game.deck);
            player.draw_card(&mut game.deck);
            if player.has_blackjack() {
                game.game_over = true;
            }
        }
        return game
    }

    pub fn add_player(&mut self, name: String){
        let player = Player::new(name);
        self.players.push(player);
    }

    pub fn start(&mut self) {
        self.deck.shuffle();
        for player in &mut self.players{
            while player.hand.len() < 2{
                player.draw_card(&mut self.deck)
            }
        }
    }

    pub fn player_draw_card(&mut self, player_index: usize) {
        if !self.game_over {
            self.players[player_index].draw_card(&mut self.deck);
            self.players[player_index].hand_value = self.players[player_index].calculate_hand();
            if self.players[player_index].is_busted() {
                self.game_over = true;
            }
        }
    }



    pub fn dealer_turn(&mut self) {
        while self.dealer.hand_value < 17 && !self.dealer.has_blackjack() && !self.game_over {
            self.dealer.draw_card(&mut self.deck);
            self.dealer.calculate_hand();
            if self.dealer.has_blackjack() || self.dealer.is_busted() {
                self.game_over = true;
            }
        }
    }

    pub fn player_stand(&mut self) {
        self.dealer_turn();
        self.game_over = true;
    }
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }
    pub fn dealer(&self) -> &Player {
        &self.dealer
    }
    pub fn player_turn () {
        todo!()
    }


    pub fn finish(&mut self) {
        self.game_over = true;
    }
}
impl Default for Game {
    fn default() -> Self {
        Self::single_player("Dime".to_string())
    }
}