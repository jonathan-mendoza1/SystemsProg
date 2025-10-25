#[derive(Debug)]
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}
struct Inventory {
    fruit: Vec<Fruit>,
}
impl Inventory {
    fn available_fruits(&self) {
        for f in &self.fruit {
            print!("{:?}: ", f);
            Self::tell_me_joke(f);
        }
    }
    fn tell_me_joke(fruit: &Fruit) {
        match fruit {
            Fruit::Tomato(msg) => println!("{}", msg),
            Fruit::Apple(msg) => println!("{}", msg),
            Fruit::Banana(msg) => println!("{}", msg),
        }
    }
}
fn main(){
    let a = "Apple pie is just a fruit salad that believed in itself.".to_string();
    let b = "Bananas are the only fruit that can start a conversation — they’re always appealing.".to_string();
    let t = "What did the father tomato say to the baby tomato who was falling behind? 'Come on, ketchup!'".to_string();
    let fruits = vec![Fruit::Apple(a),Fruit::Banana(b),Fruit::Tomato(t)];
    let grocery_store = Inventory {
        fruit:fruits,
    };
    grocery_store.available_fruits();
}