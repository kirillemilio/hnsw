mod my_sub;
mod animals;
mod houses;
use animals::animal::Animal;
use houses::doghouse::DogHouse;
use animals::dog::Dog;

mod my_add {
    pub fn add(a: f32, b: f32) -> f32 {
        return a + b;
    }
}

fn main() {
    println!("Hello, world!");
    let res = my_add::add(10.0, 11.0);
    println!("Result is {res}");
    let res = my_sub::sub(10.0, 11.0);
    println!("Result is {res}");

    let voice = animals::cat::Cat::new(10, "kitty".to_string()).voice();
    println!("Cat says: {}", voice);

    let voice = animals::dog::Dog::new(11, "doggy".to_string()).voice();
    println!("Dog says: {}", voice);

    let voice = animals::kitten::Kitten::new(5, "kitten".to_string()).voice();
    println!("Kitten says: {}", voice);

    let mut doghouse = DogHouse::new();

    doghouse.add(Dog::new(1, "dog-1".to_string())).add(Dog::new(2, "dog-2".to_string()));
    doghouse.print();
}
