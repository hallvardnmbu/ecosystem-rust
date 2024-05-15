use std::collections::HashMap;
use ordered_float::OrderedFloat;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use super::animals::*;

pub struct Island<'a> {
    pub year: u16,
    pub geography: Vec<&'a [u8]>,

    pub cells: HashMap<(usize, usize), Cell>,
    pub inhabited: Vec<(usize, usize)>,

    pub rng: &'a mut ThreadRng,
}

impl Island<'_> {
    pub fn new<'a>(geography: Vec<&'a str>, rng: &'a mut ThreadRng) -> Island<'a> {

        // Change `geography` into vector of bytes, and check that edges are 'W'.
        let geography: Vec<&[u8]> = geography.iter()
            .map(|row| {
                let row = row.as_bytes();
                assert_eq!(*row.first().unwrap(), b'W', "Edges must be of 'W'");
                assert_eq!(*row.last().unwrap(), b'W', "Edges must be of 'W'");
                row
            })
            .collect();
        if !geography.first().unwrap().iter().all(|&b| b == b'W') {
            panic!("Edges must be of 'W'!")
        }
        if !geography.last().unwrap().iter().all(|&b| b == b'W') {
            panic!("Edges must be of 'W'!")
        }

        let cells: HashMap<(usize, usize), Cell> = geography
            .iter().enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate()
                    .map(move |(j, _el)| {
                        let f_max: f32 = match _el {
                            b'W' => 0.0,
                            b'H' => 300.0,
                            b'L' => 800.0,
                            b'M' => 0.0,
                            _ => panic!("Unknown terrain type!"),
                        };
                        ((i, j), Cell {
                            f_max,
                            fodder: f_max,
                            animals: HashMap::from([
                                (Species::Herbivore, Vec::new()),
                                (Species::Carnivore, Vec::new())
                            ])
                        })
                    })
            })
            .collect();
        let inhabited = Vec::new();

        Island {
            year: 0,
            geography, cells, inhabited,
            rng
        }
    }

    // Input: vec![((x, y), Species, n), ...]
    // Where (x, y) is the coordinate
    // and Species, n the Species and number of individuals.
    pub fn add_population(&mut self, population: Vec<((usize, usize), Species, u16)>) {
        for (coordinate, species, amount) in population {
            let cell = self.cells.get_mut(&coordinate).expect("Expected Cell.");
            for _ in 0..amount {
                let mut animal = Animal {
                    species: species,
                    age: 0,
                    weight: birthweight(species, self.rng),
                    fitness: 0.0,
                };
                animal.calculate_fitness();
                cell.animals.get_mut(&species).expect("Expected animals.").push(animal);
            }
        }
        self.update_inhabited();
    }

    fn procreate(&mut self) {
        let procreation = HashMap::from([
            (Species::Herbivore, Parameters::HERBIVORE.procreate),
            (Species::Carnivore, Parameters::CARNIVORE.procreate),
        ]);
        let species_keys = Vec::from([Species::Herbivore, Species::Carnivore]);

        for coordinate in self.inhabited.iter() {
            let mut babies = HashMap::from([
                (Species::Herbivore, Vec::new()),
                (Species::Carnivore, Vec::new()),
            ]);
            let cell = self.cells
                .get_mut(coordinate)
                .expect("Expected Cell.");

            for species in species_keys.iter() {
                let probability: f32 = match species {
                    Species::Herbivore => Parameters::HERBIVORE.gamma * cell.animals[species].len() as f32,
                    Species::Carnivore => Parameters::CARNIVORE.gamma * cell.animals[species].len() as f32,
                };

                for animal in cell.animals.get_mut(species).expect("Expected animals.") {
                    if animal.weight < procreation[species] {
                        continue
                    }
                    if self.rng.gen::<f32>() >= animal.fitness * probability {
                        continue
                    }

                    let babyweight = birthweight(*species, self.rng);
                    if !animal.lose_weight_birth(babyweight) {
                        continue
                    }
                    babies.get_mut(species).expect("Expected babies.").push({
                        let mut baby = Animal {
                            species: species.clone(),
                            age: 0,
                            weight: babyweight,
                            fitness: 0.0
                        };
                        baby.calculate_fitness();
                        baby
                    })
                }
            }
            for (species, babes) in babies.iter_mut() {
                cell.animals
                    .get_mut(species).expect("Expected animals")
                    .append(babes);
            }
        }
    }

    fn feed(&mut self) {
        for coordinate in self.inhabited.iter() {
            let cell = self.cells
                .get_mut(coordinate).expect("Expected Cell");

            if &cell.animals[&Species::Herbivore].len() == &0 {
                continue
            }
            cell.grow_fodder();

            // Herbivores:
            let mut herbivores = cell.animals
                .get_mut(&Species::Herbivore).expect("Expected Herbivores")
                .clone();
            herbivores.sort_by_key(|herbivore| OrderedFloat(herbivore.fitness));

            for herbivore in herbivores.iter_mut().rev() {
                cell.fodder -= herbivore.graze(cell.fodder);
                if cell.fodder == 0.0 {
                    break
                }
            }

            // Carnivores:
            let carnivores = cell.animals
                .get_mut(&Species::Carnivore).expect("Expected Carnivores");
            carnivores.shuffle(self.rng);
            for carnivore in carnivores.iter_mut() {
                if herbivores.len() == 0 {
                    break
                }
                carnivore.predation(self.rng, &mut herbivores);
            }
            let _ = cell.animals.insert(Species::Herbivore, herbivores);
        }
    }

    fn migrate(&mut self) {
        // animal idx, coordinate, species
        let mut migrating: Vec<(usize, (usize, usize), Species)> = Vec::new();

        for coordinate in self.inhabited.iter() {
            let cell = self.cells.get_mut(coordinate).expect("Expected Cell");

            for (species, animals) in cell.animals.iter_mut() {
                let mu = match species {
                    Species::Herbivore => Parameters::HERBIVORE.mu,
                    Species::Carnivore => Parameters::CARNIVORE.mu,
                };
                for (idx, animal) in animals.iter_mut().enumerate() {
                    if self.rng.gen::<f32>() > mu * animal.fitness {
                        continue
                    }

                    migrating.push((idx, *coordinate, *species));
                }
            }
        }
        // TODO: Group by cells.
        migrating.sort_by(|a, b| a.0.cmp(&b.0));
        'moving: for (idx, coordinate, species) in migrating.iter().rev() {
            let new_cell = self.new_cell(&coordinate, &species);
            match new_cell {
                None => continue 'moving,
                Some(new_coordinate) => {
                    let animal = self.cells.get_mut(&coordinate).unwrap().animals.get_mut(&species).unwrap().remove(*idx);
                    self.cells.get_mut(&new_coordinate).unwrap().animals.get_mut(&species).unwrap().push(animal);
                }
            }
        }
        self.update_inhabited();
    }

    fn new_cell(&mut self, (x, y): &(usize, usize), species: &Species) -> Option<(usize, usize)> {

        let (stride, hunger) = match species {
            Species::Herbivore => (Parameters::HERBIVORE.stride, Parameters::HERBIVORE.hunger as u128),
            Species::Carnivore => (Parameters::CARNIVORE.stride, Parameters::CARNIVORE.hunger as u128),
        };

        let mut possibilities: Vec<(usize, usize)> = Vec::new();

        for i in x.checked_sub(stride).unwrap_or(0)..x.checked_add(stride).unwrap_or(0)+1 {
            for j in y.checked_sub(stride).unwrap_or(0)..y.checked_add(stride).unwrap_or(0)+1 {
                if (i, j) == (*x, *y) {
                    continue
                }
                if i <= 0
                    || i >= self.geography.len() as u8 as usize
                    || j <= 0
                    || j >= self.geography[0].len() as u8 as usize {
                    continue
                }
                if self.geography[i][j].eq(&b'W') {
                    continue
                }
                let diff_i = i.checked_sub(*x).unwrap_or(0);
                let diff_j = j.checked_sub(*y).unwrap_or(0);
                if diff_i.pow(2) + diff_j.pow(2) > stride.pow(2) {
                    continue
                }
                possibilities.push((i, j));
            }
        }

        let mut propensity: Vec<f32> = Vec::new();
        for idx in possibilities.iter() {
            let fodder = match species {
                Species::Herbivore => {
                    self.cells[idx].fodder as f32
                },
                Species::Carnivore => {
                    let mut fodder = 0.0f32;
                    for animal in self.cells
                        .get_mut(idx).expect("Expected Cell")
                        .animals.get_mut(&Species::Herbivore).expect("Expected Herbivore") {
                        fodder += animal.weight;
                    }
                    fodder
                },
            };
            let population = self.cells[idx].animals[species].len() as u128;
            let abundance = fodder /
                ((population + 1) * hunger)
                    .max(population + 1)
                    .max(hunger)
                    .max(1) as f32;
            propensity.push(abundance);
        }
        if propensity.len() == 0 {
            return None
        }

        // Only consider the four best possibilities.
        if propensity.len() > 4 {
            propensity.sort_by_key(|&a| OrderedFloat(a));
            propensity.drain(4..);
        }

        let propensity_sum: f32 = propensity.iter().sum();
        let new_pos_idx = ((self.rng.gen::<f32>() * propensity.len() as f32).ceil() - 1.0) as usize;

        let probability: f32;
        if propensity_sum == 0.0f32 {
            probability = 0.5;
        } else {
            probability = propensity[new_pos_idx] / propensity_sum;
        }

        if self.rng.gen::<f32>() < probability {
            return Some(possibilities[new_pos_idx])
        };
        return None
    }

    fn update_inhabited(&mut self) {
        self.inhabited.clear();
        for (coordinates, cell) in self.cells.iter_mut() {
            if cell.animals[&Species::Herbivore].len() > 0
                || cell.animals[&Species::Carnivore].len() > 0 {
                self.inhabited.push(*coordinates);
            }
        }
    }

    fn aging(&mut self) {
        self.inhabited.iter()
            .for_each(|coordinate| {
                self.cells.get_mut(coordinate).expect("Expected Cell")
                    .animals.iter_mut()
                    .for_each(|(species, animals)| {
                        let omega = match species {
                            Species::Herbivore => Parameters::HERBIVORE.omega,
                            Species::Carnivore => Parameters::CARNIVORE.omega,
                        };
                        animals.retain_mut(|animal| {
                            animal.aging();
                            animal.lose_weight_year();
                            animal.calculate_fitness();
                            if animal.weight <= 0.0f32
                                ||
                                self.rng.gen::<f32>() < omega * (1.0f32 - animal.fitness) {
                                false
                            } else {
                                true
                            }
                        });
                    });
            });
    }

    pub fn yearly_cycle(&mut self) {
        self.procreate();
        self.feed();
        self.migrate();
        self.aging();

        self.year += 1;
    }

    pub fn animals(&mut self) -> (
        HashMap<Species, u32>, HashMap<(usize, usize), HashMap<Species, u32>>
    )  {
        let mut h: u32 = 0;
        let mut c: u32 = 0;
        let mut hc: HashMap<(usize, usize), HashMap<Species, u32>> = HashMap::new();

        for coordinate in self.inhabited.iter() {
            hc.insert(*coordinate, HashMap::new());
            let _hc = hc.get_mut(coordinate).expect("Expected coordinate");

            for (species, animals) in self.cells.get_mut(coordinate).expect("Expected Cell")
                .animals.iter() {
                let n = animals.len() as u32;
                match species {
                    Species::Herbivore => { h += n; },
                    Species::Carnivore => { c += n; },
                };
                _hc.insert(*species, n + _hc.get(species).unwrap_or(&0));
            }
        }
        (HashMap::from([
            (Species::Herbivore, h),
            (Species::Carnivore, c)
        ]), hc)
    }
}

#[derive(Debug)]
pub struct Cell {
    pub f_max: f32,
    pub fodder: f32,
    pub animals: HashMap<Species, Vec<Animal>>,
}

impl Cell {
    pub const ALPHA: f32 = 0.1;
    pub const V_MAX: f32 = 800.0;

    // Set to `pub` for testing purposes.
    pub fn grow_fodder(&mut self) {
        if self.f_max == 0.0 || self.fodder == self.f_max {
            return
        }
        let growth = Cell::V_MAX * (
                1.0 - Cell::ALPHA
                    * (self.f_max - self.fodder) / self.f_max
        );
        self.fodder = f32::min(self.f_max, self.fodder + growth);
    }
}