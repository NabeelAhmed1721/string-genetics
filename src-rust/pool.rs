use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::gene::Gene;
use std::fmt;

// P -> PoolSize
// N -> GeneSize
// pub struct Pool<const P: usize> {
//     genes: [Gene; P],
// }

// P -> PoolSize
#[wasm_bindgen]
pub struct Pool {
    genes: Vec<Gene>,
    target: Gene,
}

#[derive(Debug)]
struct Mate {
    gene: Gene,
    fitness: f32,
}

#[wasm_bindgen]
impl Pool {
    #[wasm_bindgen(constructor)]
    pub fn new(target: Gene, pool_size: usize) -> Self {
        let mut genes = Vec::<Gene>::new();
        let mut rng = rand::thread_rng();

        for _ in 0..pool_size {
            // initialize all genes with random ascii character noise
            // printable ASCII list = 32..=126 (0x20..=0x7E)
            let dna_noise: String = (0..target.len())
                .map(|_| rng.gen_range::<u8, _>(0x20..=0x7E) as char)
                .collect();

            genes.push(Gene::new(&dna_noise).unwrap());
        }

        Self { genes, target }
    }

    #[wasm_bindgen(js_name = naturalSelection)]
    pub fn natural_selection(&mut self) {
        let pop_size = self.genes.len();

        // TODO: fix mating_pool spaghetti

        let mut mating_pool = Vec::<Mate>::new();

        for gene in self.genes.iter() {
            let fitness = gene.calc_fitness(&self.target).unwrap();

            mating_pool.push(Mate {
                gene: gene.clone(),
                fitness,
            });
        }

        // sort by highest fitness
        mating_pool.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        // select top 25%
        // let mating_pool = &mating_pool[0..pop_size / 4];
        // select top 10%
        let mating_pool = &mating_pool[0..pop_size / 10];

        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(
            mating_pool
                .iter()
                // weights clamp: [0.01 (MIN), 1 (MAX)]
                .map(|m| ((m.fitness * 100.).round() / 100.).clamp(0.01, 1.0)),
        )
        .unwrap();

        let mut children_pool = Vec::<Gene>::with_capacity(pop_size);

        // reproduce
        for _ in 0..pop_size {
            let partner_one = &mating_pool[dist.sample(&mut rng)].gene;
            let partner_two = &mating_pool[dist.sample(&mut rng)].gene;

            let mut child = partner_one.crossover(&partner_two).unwrap();

            child.mutate(0.1);

            children_pool.push(child);
        }

        self.genes = children_pool;
    }

    #[wasm_bindgen(js_name = getBest)]
    pub fn get_best(&self) -> Gene {
        // TODO: ouch, calculating fitness too much. Find a way to cache fitness values in `Pool`.
        let best_gene = self
            .genes
            .iter()
            .max_by(|a, b| {
                let a_fitness = a.calc_fitness(&self.target).unwrap();
                let b_fitness = b.calc_fitness(&self.target).unwrap();

                a_fitness.total_cmp(&b_fitness)
            })
            .unwrap();

        best_gene.to_owned()

        // let best_fitness = best_gene.calc_fitness(&self.target).unwrap();

        // (1.0 - best_fitness) / 1.0
    }

    // pub fn get_error(&self) -> f32 {
    //     // TODO: ouch, calculating fitness too much. Find a way to cache fitness values in `Pool`.
    //     let best_gene = self
    //         .genes
    //         .iter()
    //         .max_by(|a, b| {
    //             let a_fitness = a.calc_fitness(&self.target).unwrap();
    //             let b_fitness = b.calc_fitness(&self.target).unwrap();

    //             a_fitness.total_cmp(&b_fitness)
    //         })
    //         .unwrap();

    //     let best_fitness = best_gene.calc_fitness(&self.target).unwrap();

    //     (1.0 - best_fitness) / 1.0
    // }
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.genes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
