use rand::Rng;
pub mod animals;

fn main() {
    let mut rng = rand::thread_rng();

    let mut herb = animals::Herbivore {
        animal: animals::Animal {
            weight: 5.0,
            age: 0,
            fitness: None,
        },
    };
    herb.aging();

    println!("{}", herb.fitness())
}
