use ordered_float::OrderedFloat;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use super::animals::*;

pub struct Island<'a> {
    pub year: u16,
    pub geography: Vec<Vec<char>>,

    pub cells: Vec<Vec<Cell>>,
    pub inhabited: Vec<(usize, usize)>,

    // 'a is a lifetime, meaning that rng lives as long as Island.
    pub rng: &'a mut ThreadRng,

    // geography : Contains the terrain types at each index-pair.
    // cells     : Contains the actual animals of each cell.
    // inhabited : Contains indices of inhabited cells.
}

impl Island<'_> {
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

        let mut cells: Vec<Vec<Cell>> = Vec::new();
        for row in geography.iter() {
            let mut _cells: Vec<Cell> = Vec::new();
            for landscape in row.iter() {
                let fodder = match landscape {
                    'H' => 300u16,
                    'L' => 800u16,
                    'M' => 0u16,
                    'W' => 0u16,
                    _ => panic!("Unknown landscape type!")
                };
                let mut cell = Cell {
                    fodder, f_max: fodder,
                    animals: (
                        Vec::<Herbivore>::new(), Vec::<Carnivore>::new()
                    )
                };
                _cells.push(cell);
            }
            cells.push(_cells);
        }

        let mut inhabited = Vec::<(usize, usize)>::new();

        Island {
            year: 0,
            geography, cells, inhabited,
            rng
        }
    }

    // Input: vec![(x, y, 'X', Y)]
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
                        let mut herbivore = Herbivore {
                            weight: Herbivore::birthweight(self.rng),
                            age: 0,
                            fitness: 0.0
                        };
                        herbivore.calculate_fitness();
                        cell.animals.0.push(herbivore);
                    },
                    'c' => {
                        let mut carnivore = Carnivore {
                            weight: Carnivore::birthweight(self.rng),
                            age: 0,
                            fitness: 0.0
                        };
                        carnivore.calculate_fitness();
                        cell.animals.1.push(carnivore);
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
            let p_herb = Herbivore::GAMMA *  self.cells[*x][*y].animals.0.len() as f32;
            let mut b_herb: Vec<Herbivore> = Vec::new();
            for herbivore in self.cells[*x][*y].animals.0.iter_mut() {
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
                b_herb.push({
                    let mut baby = Herbivore {
                        weight: babyweight,
                        age: 0,
                        fitness: 0.0
                    };
                    baby.calculate_fitness();
                    baby
                })
            }
            self.cells[*x][*y].animals.0.append(&mut b_herb);

            // Loop through the Carnivores of the cell:
            let p_carn = Carnivore::GAMMA *  self.cells[*x][*y].animals.1.len() as f32;
            let mut b_carn: Vec<Carnivore> = Vec::new();
            for carnivore in self.cells[*x][*y].animals.1.iter_mut() {
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
                b_carn.push({
                    let mut baby = Carnivore {
                        weight: babyweight,
                        age: 0,
                        fitness: 0.0
                    };
                    baby.calculate_fitness();
                    baby
                })
            }
            self.cells[*x][*y].animals.1.append(&mut b_carn);
        }
    }

    fn feed(&mut self) {
        for (x, y) in self.inhabited.iter() {
            let mut cell = &mut self.cells[*x][*y];

            // Herbivores:
            cell.grow_fodder();
            cell.animals.0.sort_by_key(|herbivore| {
                OrderedFloat(herbivore.fitness)
            });
            for herbivore in cell.animals.0.iter_mut().rev() {
                cell.fodder -= herbivore.graze(cell.fodder);
            }

            // Carnivores:
            cell.animals.1.shuffle(self.rng);
            for carnivore in cell.animals.1.iter_mut() {
                carnivore.predation(self.rng, &mut cell.animals.0);
            }
        }
    }

    // fn migrate(&mut self) {
    //
    // }

    fn update_inhabitet(&mut self) {
        self.inhabited.clear();
        for (i, row) in self.cells.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if cell.animals.0.len() > 0 || cell.animals.1.len() > 0 {
                    self.inhabited.push((i, j));
                }
            }
        }
    }

    fn aging(&mut self) {
        for (x, y) in self.inhabited.iter() {
            for herbivore in self.cells[*x][*y].animals.0.iter_mut() {
                herbivore.aging();
            }
            for carnivore in self.cells[*x][*y].animals.1.iter_mut() {
                carnivore.aging();
            }
        }
    }

    fn weight_loss(&mut self) {
        for (x, y) in self.inhabited.iter() {
            for herbivore in self.cells[*x][*y].animals.0.iter_mut() {
                herbivore.lose_weight_year();
            }
            for carnivore in self.cells[*x][*y].animals.1.iter_mut() {
                carnivore.lose_weight_year();
            }
        }
    }

    fn death(&mut self) {
        for (x, y) in self.inhabited.iter() {

            for herbivore in &mut self.cells[*x][*y].animals.0 {
                herbivore.calculate_fitness();
            }
            self.cells[*x][*y].animals.0.retain(|herbivore| {
                herbivore.weight > 0.0f32 && self.rng.gen::<f32>() >= Herbivore::OMEGA * (1.0f32 -
                    &herbivore.fitness)
            });

            for carnivore in &mut self.cells[*x][*y].animals.1 {
                carnivore.calculate_fitness();
            }
            self.cells[*x][*y].animals.1.retain(|carnivore| {
                carnivore.weight > 0.0f32 && self.rng.gen::<f32>() >= Herbivore::OMEGA * (1.0f32 -
                    &carnivore.fitness)
            });
        }
    }

    pub fn yearly_cycle(&mut self) {
        self.procreate();
        self.feed();
        // self.migrate();
        self.aging();
        self.weight_loss();
        self.death();
    }

    // pub fn animals(&mut self) {
    //
    // }
    //
    // fn grow_fodder(&mut self) {
    //
    // }
}

#[derive(Debug)]
pub struct Cell {
    pub f_max: u16,
    pub fodder: u16,
    pub animals:  (
        Vec::<Herbivore>,
        Vec::<Carnivore>
    ),
}

impl Cell {
    pub const ALPHA: f32 = 0.1;
    pub const V_MAX: u16 = 800;

    pub fn grow_fodder(&mut self) {
        if self.f_max == 0 {
            return
        }
        self.fodder = self.fodder.min(
            self.fodder + Cell::V_MAX * (
                1.0
                    - Cell::ALPHA
                    * (self.f_max - self.fodder) as f32
                    / self.f_max as f32
            ) as u16);
    }
}