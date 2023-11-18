use std::cell::{RefCell};
use std::rc::Rc;

struct TotalCoins {
    coins: Vec<Rc<RefCell<i32>>>
}

impl TotalCoins {
    fn get_total(&self) -> i32 {
        let mut total = 0;
        for coin in &self.coins {
            total = total + coin.get().to_owned();
        }
        return total;
    }
}

struct Player {
    name: String,
    health: Cell<i32>,
    coins: Rc<Cell<i32>>
}

impl Player {
    fn take_damage(&self, damage: i32) {
        self.health.set(self.health.get() - damage);
    }

    fn add_coin(&self) {
        self.coins.set(self.coins.get() + 1)
    }
}

fn main() {
    let player_one = Player { name: "Mario".to_owned(), health: Cell::new(100), coins: Rc::new(Cell::new(0)) };
    let player_two = Player { name: "Luigi".to_owned(), health: Cell::new(100), coins: Rc::new(Cell::new(0)) };
    let combined_coins = TotalCoins{ coins: vec![player_one.coins, player_two.coins] };

    println!("{}'s health is {}", player_one.name, player_one.health.get());
    println!("{}'s health is {}", player_two.name, player_two.health.get());
    println!("Combined coins {}", combined_coins.get_total());

    player_one.take_damage(42);
    player_one.add_coin();

    player_two.take_damage(10);
    player_two.add_coin();

    println!("{}'s health is {}", player_one.name, player_one.health.get());
    println!("{}'s health is {}", player_two.name, player_two.health.get());
    println!("Combined coins {}", combined_coins.get_total());
}
