use rand::Rng;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, LogNormal};

pub struct Parameters {
    pub w_birth: f32,
    pub mu: f32,
    pub sigma_birth: f32,
    pub beta: f32,
    pub eta: f32,
    pub a_half: f32,
    pub phi_age: f32,
    pub w_half: f32,
    pub phi_weight: f32,
    pub gamma: f32,
    pub zeta: f32,
    pub xi: f32,
    pub omega: f32,
    pub hunger: f32,
    pub delta_phi_max: f32,
    pub stride: usize,
    pub procreate: f32,
    pub birth_mean: f32,
    pub birth_std: f32,
}

impl Parameters {
    pub const HERBIVORE: Parameters = Parameters {
        w_birth: 10.0,
        mu: 17.0,
        sigma_birth: 4.0,
        beta: 0.05,
        eta: 0.2,
        a_half: 2.5,
        phi_age: 5.0,
        w_half: 3.0,
        phi_weight: 0.09,
        gamma: 0.9,
        zeta: 0.22,
        xi: 0.42,
        omega: 0.4,
        hunger: 20.0,
        delta_phi_max: 10.0,

        stride: 1,

        procreate: 0.22 * (10.0 + 4.0),  // zeta * (w_birth + sigma_birth)

        birth_mean: 2.228375090434909,   // log((w_birth^2) / sqrt(w_birth^2 + sigma_birth^2))
        birth_std: 0.3852531701599264,   // sqrt(log(1 + (sigma_birth^2 / w_birth^2)))
    };
    pub const CARNIVORE: Parameters = Parameters {
        w_birth: 6.0,
        mu: 0.4,
        sigma_birth: 1.0,
        beta: 0.6,
        eta: 0.125,
        a_half: 40.0,
        phi_age: 0.45,
        w_half: 4.0,
        phi_weight: 0.28,
        gamma: 0.8,
        zeta: 3.5,
        xi: 1.1,
        omega: 0.3,
        hunger: 70.0,
        delta_phi_max: 10.0,

        stride: 3,

        procreate: 3.5 * (6.0 + 1.0),    // zeta * (w_birth + sigma_birth)

        birth_mean: 1.7780599821339977,  // log((w_birth^2) / sqrt(w_birth^2 + sigma_birth^2))
        birth_std: 0.16552635496534787,  // sqrt(log(1 + (sigma_birth^2 / w_birth^2)))
    };
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Species {
    Herbivore,
    Carnivore
}

pub fn birthweight(species: Species, rng: &mut ThreadRng) -> f32 {
    let (mean, std) = match species {
        Species::Herbivore => (Parameters::HERBIVORE.birth_mean, Parameters::HERBIVORE.birth_std),
        Species::Carnivore => (Parameters::CARNIVORE.birth_mean, Parameters::CARNIVORE.birth_std)
    };

    let log_normal = LogNormal::new(mean, std).unwrap();
    log_normal.sample(rng)
}

#[derive(Debug, Clone)]
pub struct Animal {
    pub species: Species,
    pub weight: f32,
    pub age: u32,
    pub fitness: f32,
}

impl Animal {
    pub fn birthweight(&self, rng: &mut ThreadRng) -> f32 {
        let (mean, std) = match self.species {
            Species::Herbivore => (Parameters::HERBIVORE.birth_mean, Parameters::HERBIVORE.birth_std),
            Species::Carnivore => (Parameters::CARNIVORE.birth_mean, Parameters::CARNIVORE.birth_std)
        };

        // TODO: Effektivisere.
        let log_normal = LogNormal::new(mean, std).unwrap();
        log_normal.sample(rng)
    }

    pub fn gain_weight(&mut self, food: f32) {
        match self.species {
            Species::Herbivore => self.weight += Parameters::HERBIVORE.beta * food,
            Species::Carnivore => self.weight += Parameters::CARNIVORE.beta * food,
        };
        self.calculate_fitness();
    }

    pub fn aging(&mut self) {
        self.age += 1;
    }

    pub fn lose_weight_year(&mut self) {
        match self.species {
            Species::Herbivore => self.weight -= Parameters::HERBIVORE.eta * self.weight,
            Species::Carnivore => self.weight -= Parameters::CARNIVORE.eta * self.weight,
        }
    }

    pub fn lose_weight_birth(&mut self, baby_weight: f32) -> bool {
        let xi: &f32 = match self.species {
            Species::Herbivore => &Parameters::HERBIVORE.xi,
            Species::Carnivore => &Parameters::CARNIVORE.xi,
        };
        if self.weight > xi * baby_weight {
            self.weight -= xi * baby_weight;
            self.calculate_fitness();
            true
        } else {
            false
        }
    }

    pub fn calculate_fitness(&mut self) {
        if self.weight <= 0.0 {
            self.fitness = 0.0;
            return
        }

        let (phi_age, a_half, phi_weight, w_half) = match self.species {
            Species::Herbivore => (
                Parameters::HERBIVORE.phi_age,
                Parameters::HERBIVORE.a_half,
                Parameters::HERBIVORE.phi_weight,
                Parameters::HERBIVORE.w_half,
            ),
            Species::Carnivore => (
                Parameters::CARNIVORE.phi_age,
                Parameters::CARNIVORE.a_half,
                Parameters::CARNIVORE.phi_weight,
                Parameters::CARNIVORE.w_half,
            ),
        };
        let q_pos = (1.0
            + f32::exp(phi_age * (self.age as f32 - a_half)))
        .powf(-1.0);

        let q_neg = (1.0
            + f32::exp(-phi_weight * (self.weight - w_half)))
        .powf(-1.0);

        self.fitness = q_pos * q_neg;
    }

    pub fn graze(&mut self, available_fodder: f32) -> f32 {
        match self.species {
            Species::Carnivore => panic!("Carnivores can't graze!"),
            _ => ()
        }

        if available_fodder >= Parameters::HERBIVORE.hunger {
            self.gain_weight(Parameters::HERBIVORE.hunger);
            Parameters::HERBIVORE.hunger
        } else {
            self.gain_weight(available_fodder);
            available_fodder
        }
    }

    pub fn predation(&mut self, rng: &mut ThreadRng, herbivores: &mut Vec<Animal>) {
        match self.species {
            Species::Herbivore => panic!("Herbivores can't hunt!"),
            _ => ()
        }

        let mut eaten: f32 = 0.0;
        let mut removing: Vec<usize> = Vec::new();

        'herbivores: for (idx, herbivore) in herbivores.iter_mut().enumerate() {
            let herbivore_fitness = herbivore.fitness;
            let carnivore_fitness = self.fitness;
            let difference = carnivore_fitness - herbivore_fitness;

            let prob: f32;
            if carnivore_fitness <= herbivore_fitness {
                prob = 0.0;
            } else if 0.0 < difference && difference < Parameters::CARNIVORE.delta_phi_max {
                prob = difference / Parameters::CARNIVORE.delta_phi_max;
            } else {
                prob = 1.0;
            }

            if rng.gen::<f32>() < prob {
                removing.push(idx);

                let rest = Parameters::CARNIVORE.hunger - eaten;
                let herbivore_weight = herbivore.weight;

                if herbivore_weight < rest {
                    eaten += herbivore_weight;
                    self.gain_weight(herbivore_weight);
                } else {
                    self.gain_weight(rest);
                    break 'herbivores;  // As a fail-safe I choose to name the loop, to be sure.
                }
            }
        }
        for idx in removing.iter().rev() {
            herbivores.remove(*idx);
        }
    }
}
