mod gene;
mod pool;
// mod utils;

// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// extern "C" {

//     #[wasm_bindgen(js_namespace = console)]
//     pub fn log(s: &str);
// }

// #[wasm_bindgen]
// pub fn big_computation() -> String {
//     // utils::set_panic_hook();
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

/*
 *
 * mod gene;
mod pool;

use gene::Gene;
use pool::Pool;

const GENERATIONS: u32 = 1024;

fn main() {
    let target = Gene::new("Nabeel Ahmed").unwrap();
    let mut pool = Pool::new(target.clone(), 512);

    // println!(
    //     "Gen 0: {:.2}",
    //     pool.get_best().calc_fitness(&target).unwrap()
    // );

    for i in 0..GENERATIONS {
        pool.natural_selection();

        println!("Result #{}: {}", i, pool.get_best());

        if pool.get_best().calc_fitness(&target).unwrap() == 1.0 {
            break;
        }
        // println!(
        //     "Gen {}: {:.2}",
        //     i + 1,
        //     pool.get_best().calc_fitness(&target).unwrap()
        // );
    }

    println!("Result: {}", pool.get_best());
}

 */
