use rand::Rng;
use std::fmt;
use std::str;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Gene {
    dna: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum GeneError {
    NotPrintableAscii, // 32..=126 (0x20..=0x7E)
    MismatchedTargetDNALength,
    MismatchedPartnerDNALength,
}

// Chromosome > Gene > DNA
#[wasm_bindgen()]
impl Gene {
    #[wasm_bindgen(constructor)]
    pub fn new(dna_init: &str) -> Result</*Self*/ Gene, GeneError> {
        if !Self::is_printable_ascii(dna_init) {
            return Err(GeneError::NotPrintableAscii);
        }

        Ok(Self {
            dna: dna_init.as_bytes().to_vec(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn dna(&self) -> Vec<u8> {
        self.dna.clone()
    }

    #[wasm_bindgen(setter, js_name = setDNA)]
    pub fn set_dna(&mut self, dna_edit: &str) -> Result<(), GeneError> {
        if !Self::is_printable_ascii(dna_edit) {
            return Err(GeneError::NotPrintableAscii);
        }

        self.dna = dna_edit.as_bytes().to_vec();

        Ok(())
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        str::from_utf8(&self.dna)
            .unwrap_or("DNA contains non-utf8 characters.")
            .to_owned()
    }

    #[wasm_bindgen(js_name = calcFitness)]
    pub fn calc_fitness(&self, target: &Gene) -> Result<f32, GeneError> {
        if self.len() != target.len() {
            return Err(GeneError::MismatchedTargetDNALength);
        }

        let vis_ascii_len = (0x20u8..=0x7Eu8).len();

        // TODO: investigate...

        // https://stackoverflow.com/a/59167818/9851824
        let differences: usize = self
            .dna
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let idx_1 = *c as i32;
                let idx_2 = target.dna[i] as i32;

                let dist = i32::abs(idx_1 - idx_2) as usize;

                std::cmp::min(vis_ascii_len - dist, dist)
            })
            .sum();

        //
        let fitness: f32 = 1.0 - (differences as f32 / ((vis_ascii_len / 2) * target.len()) as f32);

        // let matches = self
        //     .dna
        //     .iter()
        //     .enumerate()
        //     .filter(|&(i, c)| *c == target.dna[i])
        //     .count();

        // let fitness: f32 = matches as f32 / target.len() as f32;

        Ok(fitness)
    }

    // TODO: returns Result<>
    // TODO: fix error handling
    #[wasm_bindgen]
    pub fn crossover(&self, partner: &Gene) -> Result</*Self*/ Gene, GeneError> {
        // TODO: two-point crossover?
        /*
         * parents:
         *          A: o|ooooooooo
         *          B: -|---------
         *      or
         *          A: oooooo|oooo
         *          B: ------|----
         *
         * child:
         *         C: o|---------
         *      or
         *         C: oooooo|----
         *
         */

        if self.len() != partner.len() {
            return Err(GeneError::MismatchedPartnerDNALength);
        }

        // crossover a minimum of 1 dna from each gene
        let cross_point: usize = rand::thread_rng().gen_range(1..self.len());

        let new_dna = {
            let mut sequence = Vec::<u8>::new();

            sequence.extend_from_slice(&self.dna[0..cross_point]);
            sequence.extend_from_slice(&partner.dna[cross_point..]);

            str::from_utf8(&sequence)
                .expect("Cannot crossover Genes.")
                .to_owned()
        };

        let child = Gene::new(&new_dna).expect("Crossover contained non-ASCII characters.");

        Ok(child)
    }

    #[wasm_bindgen]
    pub fn mutate(&mut self, rate: f32) {
        let mut rng = rand::thread_rng();

        // TODO: fix `as`'s
        let shift_char = |current: i32, shift: i32| {
            let max_value = 0x7Eu8;
            let min_value = 0x20u8;
            let range_size: i32 = (max_value - min_value + 1).into();

            let mut result = (current + shift - min_value as i32) % range_size;

            if result < 0 {
                result += range_size;
            }

            result += min_value as i32;

            result as u8
        };

        for character in self.dna.iter_mut() {
            let chance: f32 = rng.gen_range(0.0..1.0);

            let c = character.clone();

            if rate > chance {
                let shift_amount = rng.gen_range(-4..4);

                *character = shift_char(c as i32, shift_amount);
            }
        }
    }

    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.dna.len()
    }

    fn is_printable_ascii(input: &str) -> bool {
        let range = 0x20..=0x7E; // (0x20..=0x7E)
        for byte in input.as_bytes().into_iter() {
            if !range.contains(byte) {
                return false;
            }
        }
        return true;
    }
}

impl Default for Gene {
    fn default() -> Self {
        Self::new("").unwrap()
    }
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Gene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::error::Error for GeneError {}

// TODO: Find a way to prevent repeating error strings?

impl fmt::Display for GeneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match *self {
            GeneError::NotPrintableAscii => "Non-printable ASCII character(s) found in DNA.",
            GeneError::MismatchedTargetDNALength => "Target DNA length does not match.",
            GeneError::MismatchedPartnerDNALength => "Partner DNA length does not match.",
        };
        write!(f, "{}", error_message)
    }
}

impl Into<JsValue> for GeneError {
    fn into(self) -> JsValue {
        let error_message = match self {
            GeneError::NotPrintableAscii => "Non-printable ASCII character(s) found in DNA.",
            GeneError::MismatchedTargetDNALength => "Target DNA length does not match.",
            GeneError::MismatchedPartnerDNALength => "Partner DNA length does not match.",
        };
        JsValue::from_str(error_message)
    }
}
