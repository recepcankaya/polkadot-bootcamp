use std::io;
#[derive(Debug)]
struct Customer {
    items: Vec<String>,
}

impl Customer {
    fn add_items(&mut self, item: String) {
        self.items.push(item);
    }
    fn remove_items(&mut self, item: String) {
        if self.items.contains(&item) == true {
            let index = self.items.iter().position(|x| x == &item).unwrap();
            self.items.remove(index);
        }
    }
}

fn main() {
    let mut customer_cart = Customer { items: vec![] };
    loop {
        let mut user_input = String::new();
        println!(
            "Write 'add' to add to cart or 'remove' to remove from the cart or 'exit' to exit "
        );
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read");
        if user_input.trim() == "exit".to_string() {
            break;
        }
        if user_input.trim() == "add".to_string() {
            println!("Write the item name to add to the cart or 'No' to exit");
            let mut added_item = String::new();
            io::stdin()
                .read_line(&mut added_item)
                .expect("Failed to read");
            if user_input.trim() == "No".to_string() {
                break;
            }
            customer_cart.add_items(added_item.clone());
        }
        if user_input.trim() == "remove".to_string() {
            println!("Write the item name to remove from the cart or 'No' to exit");
            let mut removed_item = String::new();
            io::stdin()
                .read_line(&mut removed_item)
                .expect("Failed to read");
            if user_input.trim() == "No".to_string() {
                break;
            }
            customer_cart.remove_items(removed_item.clone());
        }
        println!("The customer cart is {:?}", customer_cart);
    }
}
