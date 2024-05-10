use rand::Rng;

pub mod animals;
mod island;

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

    println!("Fitness: {}", herb.fitness());

    let geography = vec![vec!['W', 'L'], vec!['W', 'W']];

    let isl = island::Island::new(geography, &mut rng);

    println!("Cells: {:?}", isl.cells);
    println!("Checking the rng of Island: {}", isl.rng.gen::<u8>());
}
