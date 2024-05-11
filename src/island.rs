use rand::prelude::ThreadRng;
use rand::Rng;
use super::animals::*;

pub struct Island<'a> {
    pub year: u16,
    pub geography: Vec<Vec<char>>,

    pub cells: Vec<Vec<(Vec<Herbivore>, Vec<Carnivore>)>>,
    pub inhabited: Vec<(usize, usize)>,

    // 'a is a lifetime, meaning that rng lives as long as Island.
    pub rng: &'a mut ThreadRng,

    // geography : Contains the terrain types at each index-pair.
    // cells     : Contains the actual animals of each cell.
    // inhabited : Contains indices of inhabited cells.
}

impl Island<'_> {
    pub const H: u16 = 300;
    pub const L: u16 = 800;
    pub const M: u16 = 0;
    pub const W: u16 = 0;
    pub const ALPHA: f32 = 0.1;
    pub const V_MAX: u16 = 800;

    pub fn new(geography: Vec<Vec<char>>, rng: &mut ThreadRng) -> Island {
        for row in geography.iter() {
            assert_eq!(*row.first().unwrap(), 'W', "Edges must be 'W' (water)!");
            assert_eq!(*row.last().unwrap(), 'W', "Edges must be 'W' (water)!");
        }
        for element in geography.first().unwrap() {
            assert_eq!(*element, 'W', "Edges must be 'W' (water)!");
        }
        for element in geography.last().unwrap() {
            assert_eq!(*element, 'W', "Edges must be 'W' (water)!");
        }

        let mut cells = geography.iter()
            .map(|row| {
                row.iter()
                    .map(|_| (Vec::<Herbivore>::new(),
                              Vec::<Carnivore>::new())).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
        let mut inhabited = Vec::<(usize, usize)>::new();

        Island {
            year: 0,
            geography, cells, inhabited,
            rng
        }
    }

    // Input: vec![((x, y, 'X', Y)]
    // Where x, y is the coordinate
    // and 'X', Y the species and amount; 'h': Herbivore, 'c': Carnivore
    pub fn add_population(&mut self, population: Vec<(usize, usize, char, u16)>) {
        for item in population.iter() {

            // Update inhabited vector with current index-pair.
            self.inhabited.push((item.0, item.1));

            // Retrive the current cell for memory efficiency.
            let cell = &mut self.cells[item.0][item.1];

            // Create the number of specified animals and push to respective vector.
            for _ in 0..item.3 {
                match item.2 {
                    'h' => {
                        let herbivore = Herbivore {
                            weight: Herbivore::birthweight(self.rng),
                            age: 0,
                            fitness: None
                        };
                        cell.0.push(herbivore);
                    },
                    'c' => {
                        let carnivore = Carnivore {
                            weight: Carnivore::birthweight(self.rng),
                            age: 0,
                            fitness: None
                        };
                        cell.1.push(carnivore);
                    },
                    _ => panic!(
                        "Unknown species. \
                        Should be 'h' or 'c' for Herbivores and Carnivores, respectively."
                    ),
                };
            }
        }
    }

    pub fn procreate(&mut self) {
        for (x, y) in self.inhabited.iter() {

            // Loop through the Herbivoress of the cell:
            let p_herb = Herbivore::GAMMA *  self.cells[x][y][0].len();
            for herbivore in self.cells[x][y][0].iter() {
                if herbivore.weight < Herbivore::PROCREATE {
                    continue
                }
                if self.rng.gen::<f32>() >= herbivore.fitness * p_herb {
                    continue
                }
                let babyweight = Herbivore::birthweight(self.rng);
                if !herbivore.lose_weight_birth(babyweight) {
                    continue
                }
                self.cells[x][y][0].push(
                    Herbivore {
                        weight: babyweight,
                        age: 0,
                        fitness: None
                    }
                )
            }

            // Loop through the Carnivores of the cell:
            let p_carn = Carnivore::GAMMA *  self.cells[x][y][1].len();
            for carnivore in self.cells[x][y][1].iter() {
                if carnivore.weight < Carnivore::PROCREATE {
                    continue
                }
                if self.rng.gen::<f32>() >= carnivore.fitness * p_carn {
                    continue
                }
                let babyweight = Carnivore::birthweight(self.rng);
                if !carnivore.lose_weight_birth(babyweight) {
                    continue
                }
                self.cells[x][y][0].push(
                    Carnivore {
                        weight: babyweight,
                        age: 0,
                        fitness: None
                    }
                )
            }
        }
    }
}