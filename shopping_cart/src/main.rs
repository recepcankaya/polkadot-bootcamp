use std::io;
#[derive(Debug)]
struct Customer {
    items: Vec<String>,
}

impl Customer {
    fn add_items(&mut self, item: String) {
        self.items.push(item);
    }
}

fn main() {
    let mut customer_cart = Customer { items: vec![] };
    loop {
        let mut user_input = String::new();
        println!("Please, enter the item or write 'No' to exit");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read");
        if user_input.trim() == "No".to_string() {
            break;
        }
        customer_cart.add_items(user_input.clone());
        println!("The customer cart is {:?}", customer_cart);
    }
}
