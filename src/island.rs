use rand::prelude::ThreadRng;
use crate::animals;

pub struct Island {
    pub year: u16,
    pub geography: Vec<Vec<char>>,
    pub cells: Vec<Vec<(Vec<animals::Herbivore>, Vec<animals::Carnivore>)>>,
    pub inhabited: Vec<(u8, u8)>,

    // geography : Contains the terrain types at each index-pair.
    // cells     : Contains the actual animals of each cell.
    // inhabited : Contains indices of inhabited cells.
}

impl Island {
    pub const H: u16 = 300;
    pub const L: u16 = 800;
    pub const M: u16 = 0;
    pub const W: u16 = 0;
    pub const ALPHA: f32 = 0.1;
    pub const V_MAX: u16 = 800;

    pub fn new(map: Vec<Vec<char>>) {

    }

    // Input: vec![((x, y, 'X', Y)]
    // Where x, y is the coordinate
    // and 'X', Y the species and amount; 'h': Herbivore, 'c': Carnivore
    pub fn add_population(&mut self, rng: &mut ThreadRng, population: Vec<(u8, u8, char, u16)>) {
        for item in population.iter() {

            // Update inhabited vector with current index-pair.
            self.inhabited.push((*item.0, *item.1));

            // Retrive the current cell for memory efficiency.
            let mut cell = self.cells[*item.0][*item.1];

            // Create the number of specified animals and push to respective vector.
            for i in 0..*item.3 {
                match *item.2 {
                    'h' => {
                        let herbivore = animals::Species::Herbivore(animals::Herbivore {
                            animal: animals::Animal {
                                weight: animals::Herbivore::birthweight(rng),
                                age: 0,
                                fitness: None
                            }
                        });
                        cell[0].push(herbivore);
                    },
                    'c' => {
                        let carnivore = animals::Species::Carnivore(animals::Carnivore {
                            animal: animals::Animal {
                                weight: animals::Carnivore::birthweight(rng),
                                age: 0,
                                fitness: None
                            }
                        });
                        cell[1].push(carnivore);
                    },
                    _ => panic!(
                        "Unknown species. \
                        Should be 'h' or 'c' for Herbivores and Carnivores, respectively."
                    ),
                };
            }
        }
    }
}