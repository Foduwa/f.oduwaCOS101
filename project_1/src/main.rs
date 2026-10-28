struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    fn cost_purchase(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    let hp_cost = hp.cost_purchase(3);
    let ibm_cost = ibm.cost_purchase(3);
    let toshiba_cost = toshiba.cost_purchase(3);
    let dell_cost = dell.cost_purchase(3);

    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("Total cost for 3 laptops from each brand is N{}", total_cost);
}