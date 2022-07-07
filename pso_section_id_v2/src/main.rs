use rand::seq::SliceRandom;

// quick test to see if i can add stuff outside funcitons
//let my_lst = ["item1", "item2", "item3"];

fn main() {
    let my_lst = ["item1", "item2", "item3"];

    println!("{:?}", my_lst.choose(&mut rand::thread_rng()));
}
