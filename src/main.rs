use rand::Rng;

pub mod animals;
mod island;

fn main() {
    let mut rng = rand::thread_rng();

    // let mut herb = animals::Herbivore {
    //         weight: 5.0,
    //         age: 0,
    //         fitness: None,
    // };
    // herb.aging();
    // println!("Fitness: {}", herb.fitness());

    let geography = vec![vec!['W', 'W', 'W'], vec!['W', 'L', 'W'], vec!['W', 'W', 'W']];

    let mut isl = island::Island::new(geography, &mut rng);

    // println!("Cells: {:#?}", isl.cells);

    // vec![(x, y, 'X', Y), ...]
    // Where x, y is the coordinate
    // and 'X', Y the species and amount; 'h': Herbivore, 'c': Carnivore
    isl.add_population(vec![(1, 1, 'h', 2), (1, 1, 'c', 1)]);

    println!("Cells: {:#?}", isl.cells);

    isl.yearly_cycle();

    println!("Cells: {:#?}", isl.cells);

    // println!("Checking the rng of Island: u8: {} f32: {}", isl.rng.gen::<u8>(), isl.rng.gen::<f32>());
}
