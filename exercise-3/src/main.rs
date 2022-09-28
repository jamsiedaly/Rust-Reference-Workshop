struct Order {
    buy: bool,
    amount: i32,
    completed: bool
}

struct Cancel {
    id: i32,
    completed: bool
}

trait Instruction {
    fn execute(&self);
}

impl Instruction for Order {
    fn execute(&self) {
        let command = if self.buy {
            "Buy"
        } else {
            "Sell"
        };
        println!("{} {}", command, self.amount);
    }
}

impl Instruction for Cancel {
    fn execute(&self) {
        println!("Cancel order {}", self.id);
    }
}

fn main() {
    let order_1 = Order {
        buy: false,
        amount: 15,
        completed: false
    };

    let cancellation = Cancel {
        id: 1,
        completed: false
    };

    let order_2 = Order {
        buy: true,
        amount: 55,
        completed: false
    };

    let instructions: Vec<dyn Instruction> = vec![order_1, cancellation, order_2];

    for instruction in instructions {
        instruction.execute();
    }
}
