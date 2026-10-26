struct Laptop {
    brand:String,
    price:u32,
    quantity:u32
}

impl Laptop{
    fn cost_of_purchase(&self)->u32 {
        self.price * self.quantity
    }
}

fn main() {
    let total1 = Laptop {
        brand:String::from("HP"),
        price:650_000,
        quantity:10 
    };
    println!("Brand is {}\n, price is {}\n, quantity is {}\n, total is {}",total1.brand,total1.price,total1.quantity,total1.cost_of_purchase());

    let total2 = Laptop {
        brand:String::from("IBM"),
        price:755_000,
        quantity:6
    };
    println!("Brand is {}\n, price is {}\n, quantity is {}\n, total is {}",total2.brand,total2.price,total2.quantity,total2.cost_of_purchase());

    let total3 = Laptop {
        brand:String::from("Toshiba"),
        price:550_00,
        quantity:10
    };println!("Brand is {}\n, price is {}\n, quantity is {}\n, total is {}",total3.brand,total3.price,total3.quantity,total3.cost_of_purchase());

    let total4 = Laptop {
        brand:String::from("Dell"),
        price:850_000,
        quantity:4
    };
    println!("Brand is {}\n, price is {}\n, quantity is {}\n, total is {}",total4.brand,total4.price,total4.quantity,total4.cost_of_purchase());

    let purchased_units = 3;
    let hp_total = total1.price * purchased_units;
    let ibm_total = total2.price * purchased_units;
    let toshiba_total = total3.price * purchased_units;
    let dell_total = total4.price * purchased_units;
    let total_overall = hp_total + ibm_total + toshiba_total + dell_total;
    println!("Total of 3 Laptops bought is {}",total_overall);

}