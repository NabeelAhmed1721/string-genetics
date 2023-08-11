/* tslint:disable */
/* eslint-disable */
/**
*/
export class Gene {
  free(): void;
/**
* @param {string} dna_init
*/
  constructor(dna_init: string);
/**
* @returns {string}
*/
  toString(): string;
/**
* @param {Gene} target
* @returns {number}
*/
  calcFitness(target: Gene): number;
/**
* @param {Gene} partner
* @returns {Gene}
*/
  crossover(partner: Gene): Gene;
/**
* @param {number} rate
*/
  mutate(rate: number): void;
/**
* @returns {number}
*/
  len(): number;
/**
*/
  readonly dna: Uint8Array;
/**
*/
  setDNA: string;
}
/**
*/
export class Pool {
  free(): void;
/**
* @param {Gene} target
* @param {number} pool_size
*/
  constructor(target: Gene, pool_size: number);
/**
*/
  naturalSelection(): void;
/**
* @returns {Gene}
*/
  getBest(): Gene;
}
