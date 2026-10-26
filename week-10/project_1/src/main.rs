struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    let total_cost =
        hp.cost(3) + ibm.cost(3) + toshiba.cost(3) + dell.cost(3);

    println!("Total Cost: N{}", total_cost);
}
