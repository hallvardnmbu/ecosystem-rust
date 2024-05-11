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
                let cell = Cell {
                    fodder, f_max: fodder,
                    animals: (
                        Vec::<Herbivore>::new(), Vec::<Carnivore>::new()
                    )
                };
                _cells.push(cell);
            }
            cells.push(_cells);
        }

        let inhabited = Vec::<(usize, usize)>::new();

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
        self.update_inhabited();
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
            let cell = &mut self.cells[*x][*y];

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

    fn migrate(&mut self) {
        let mut migrating: Vec<(usize, (usize, usize), (usize, usize))> = Vec::new();

        for (x, y) in self.inhabited.iter() {

            // Loop through the Herbivoress of the cell:
            for (animal_idx, herbivore) in self.cells[*x][*y].animals.0.iter().enumerate() {
                if self.rng.gen::<f32>() > Herbivore::MU * herbivore.fitness {
                    continue
                }
                let rndnr1 = self.rng.gen::<f32>();
                let rndnr2 = self.rng.gen::<f32>();
                let new_cell = self.migrate_herbivore(&rndnr1, &rndnr2, x, y);
                match new_cell {
                    (1000, 1000) => continue,
                    _ => migrating.push(
                        (animal_idx, (*x, *y), new_cell)
                    ),
                };
            }
        }

        migrating.sort_by(|a, b| a.0.cmp(&b.0));
        for (idx, from_cell, to_cell) in migrating.iter().rev() {
            let animal = self.cells[from_cell.0][from_cell.1].animals.0.remove(*idx);
            self.cells[to_cell.0][to_cell.1].animals.0.push(animal);
        }
    }

    fn migrate_herbivore(&self, randnr1: &f32, randnr2: &f32, x: &usize, y: &usize) -> (usize, usize) {
        let mut possibilities: Vec<(usize, usize)> = Vec::new();
        for i in x - Herbivore::STRIDE..x + Herbivore::STRIDE + 1 {
            for j in y - Herbivore::STRIDE..y + Herbivore::STRIDE + 1 {
                if (i, j) == (*x, *y) {
                    continue
                }
                if i <= 0
                    || i >= self.geography[0].len() as u8 as usize
                    || j <= 0
                    || j >= self.geography.len() as u8 as usize {
                    continue
                }
                if self.geography[i][j] == 'W' {
                    continue
                }
                if (i - x).pow(2) + (j - y).pow(2) > Herbivore::STRIDE.pow(2) {
                    continue
                }
                possibilities.push((i, j));
            }
        }

        let mut propensity: Vec<f32> = Vec::new();
        for (i, j) in possibilities.iter() {
            let cell: &Cell = &self.cells[*i][*j];
            let fodder = cell.fodder as f32;
            let population = cell.animals.0.len() as u16;
            let abundance = fodder /
                ((population + 1) * Herbivore::F)
                    .max(population + 1)
                    .max(Herbivore::F)
                    .max(1) as f32;
            propensity.push(abundance);
        }
        if propensity.len() == 0 {
            return (1000, 1000)
        }

        let propensity_sum: f32 = propensity.iter().sum();
        let new_pos_idx = ((randnr1 * propensity.len() as f32).ceil() - 1.0) as usize;

        let probability: f32;
        if propensity_sum == 0.0f32 {
            probability = 0.5;
        } else {
            probability = propensity[new_pos_idx] / propensity_sum;
        }

        if *randnr2 < probability {
            return possibilities[new_pos_idx]
        };
        return (1000, 1000)
    }

    fn update_inhabited(&mut self) {
        self.inhabited.clear();
        for (i, row) in self.cells.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if cell.animals.0.len() > 0 || cell.animals.1.len() > 0 {
                    self.inhabited.push((i, j));
                }
            }
        }
    }

    fn ageing(&mut self) {
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
        self.migrate();
        self.ageing();
        self.weight_loss();
        self.death();

        self.year += 1;
    }

    // pub fn animals(&mut self) {
    //
    // }
}

#[derive(Debug)]
pub struct Cell {
    pub f_max: u16,
    pub fodder: u16,
    pub animals:  (
        Vec<Herbivore>,
        Vec<Carnivore>
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