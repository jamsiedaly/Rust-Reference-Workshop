use std::cell::Cell;

struct Player {
    name: String,
    health: i32
}

impl Player {
    fn take_damage(&self, damage: i32) {
        self.health = self.health - damage;
    }
}

fn main() {
    let player = Player { name: "Jim bob".to_owned(), health: 100 };

    println!("{}'s health is {}", player.name, player.health);

    player.take_damage(42);

    println!("{}'s health is {}", player.name, player.health);
}
