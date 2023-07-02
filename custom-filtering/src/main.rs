struct FilterCondition {
    data: u32,
}

impl FilterCondition {
    fn is_match(&self, item: &u32) -> bool {
        if self.data % item == 0 {
            true
        } else {
            false
        }
    }
}

fn custom_filter(x: Vec<u32>, y: &FilterCondition) -> Vec<u32> {
    x.into_iter().filter(|&a| a % y.data == 0).collect()
}

fn main() {
    let collec = vec![2131212, 908290, 376237462, 12788913];
    let condition = FilterCondition { data: 34 };
    let new_collec = custom_filter(collec, &condition);
    println!("The filtered result is: {:?}", new_collec);
}
