use crate::factors::*;
use crate::qck_div_chk::*;
use std::ops::{Deref, DerefMut};

pub struct Primes {
    pub primes_conf: PrimesConf,
    primes_vec: Vec<usize>,
}

pub enum PrimesConf {
    /*
     * This will eventualy die, when we can "predict" how many primes we would need
     */
    Constant,
    Linear,
    Exponential,
}

impl std::fmt::Display for Primes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.primes_vec)
    }
}

impl IntoIterator for Primes {
    type Item = usize;
    type IntoIter = <Vec<usize> as IntoIterator>::IntoIter; // so that you don't have to write std::vec::IntoIter, which nobody remembers anyway

    fn into_iter(self) -> Self::IntoIter {
        self.primes_vec.into_iter()
    }
}

// We deref to slice so that we can reuse the slice impls
impl Deref for Primes {
    type Target = [usize];

    fn deref(&self) -> &[usize] {
        &self.primes_vec[..]
    }
}
impl DerefMut for Primes {
    fn deref_mut(&mut self) -> &mut [usize] {
        &mut self.primes_vec[..]
    }
}
/*
pub struct PrimesIter<'a> {
    values: &'a Vec<usize>,
    index: usize,
}

impl<'a> Iterator for PrimesIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.values.len() {
            return None
        }

        self.index += 1;
        Some(&self.values[self.index - 1])
    }
}
*/

impl Primes {
    pub fn new() -> Primes {
        Primes {
            /*
             * creates a new primes vector, it is necessary that at least it has up to and including the 7 in the primes vec
             * ej: => primes_vec: vec![2, 3, 5, 7],
             */
            primes_vec: vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199,
            ],
            primes_conf: PrimesConf::Exponential,
        }
    }

    pub fn add_n_more_primes(&mut self, n: usize) {
        /*
         * as the names implies, adds n primes to the vec of primes.
         * this is used to extend the vec when we need more primes
         */
        for _ in 0..n {
            let mut possible_next_prime: usize = self.primes_vec.last().unwrap_or(&2) + 2;
            let mut remainder = 0;

            loop {
                /*
                 * we check if the number is divisible by 2,3,5,7 using the quick_div_chk()
                 * if it is divisible we automatically increment the new "possible next prime" and start again
                 * we only check for thease numbers as they are the most common
                 * we can always define more checks... in this particular case we are only intrested in divisibility by prime numbers
                 */
                if QckDiv::Two.check_div(possible_next_prime) {
                    possible_next_prime += 2;
                    continue;
                }
                if QckDiv::Three.check_div(possible_next_prime) {
                    possible_next_prime += 2;
                    continue;
                }
                if QckDiv::Five.check_div(possible_next_prime) {
                    possible_next_prime += 2;
                    continue;
                }
                if QckDiv::Seven.check_div(possible_next_prime) {
                    possible_next_prime += 2;
                    continue;
                }
                /*
                 * if none of the quick checks failed, we have to check manually (slow) for all other primes
                 */
                for &p in &self.primes_vec {
                    remainder = possible_next_prime % p;
                    if remainder == 0 {
                        /*
                         * if the reminder is 0, it is divisible so it is not a prime
                         */
                        break;
                    }
                    if p * p > possible_next_prime {
                        /*
                         * if we reach this point then the "possible_next_prime" it is indeed a prime
                         * as the only whole divisors(d) of a number(n) must be smaller than the scuared root of n
                         * this is the same that:
                         *      d < sqrt(n) <- squaring both sides
                         *      d * d > n
                         */
                        break;
                    }
                }

                if remainder != 0 {
                    /*
                     * if whe reach here then the number has no prime divisors... it is prime!
                     * we add it to our vec
                     */
                    self.primes_vec.push(possible_next_prime);
                    break;
                };

                possible_next_prime += 2;
            }
        }
    }

    pub fn one_more(&mut self) {
        /*
         * just a shortcut for when you only whant to add "one_more"
         */
        self.add_n_more_primes(1);
    }

    pub fn n_is_prime(&mut self, n: usize) -> bool {
        /*
         * we check for a number n if it is prime or not
         * the idea is to be fast when you preserve your prime variable, so that you dont have to recalculate all your primes each time
         * you can change this setting with the PConf enum
         * i have found that for very intensive use of large numbers, "Exponential" is the best
         * although for single use or very unfrecuent use, it generates an unnecesary overhead as it tends to use a lot more ram and cpu
         */
        let mut counter: usize = 1;

        // getting the biggest prime found yet
        let mut last_prime: usize = self.primes_vec.last().unwrap_or(&2) + 0;

        /*
         * if n is smaller than the las prime, then you only have to see if it is inside the vec
         * if not... we increment the vec untill it is smaler
         */
        while n > last_prime {
            self.add_n_more_primes(counter);
            counter = match self.primes_conf {
                PrimesConf::Constant => counter,
                PrimesConf::Linear => counter + 1,
                PrimesConf::Exponential => counter * 2,
            };
            last_prime = self.primes_vec.last().unwrap() + 0;
        }

        /*
         * when finaly it is smaller we just se if it is inside the vec
         * if it is... then its prime, if not, not
         */
        self.primes_vec.contains(&n)
    }

    pub fn factorice_number(&mut self, num: usize) -> Factors {
        let mut number = num;
        let mut result: Factors = Factors::new();

        if num <= 1 {
            return result;
        }
        if self.n_is_prime(num) {
            result.add_factor(num);
            return result;
        }

        for &p in &self.primes_vec {
            while number % p == 0 {
                result.add_factor(p);
                number = number / p;
            }
            if number <= 1 {
                break;
            }
        }
        return result;
    }
}

pub trait Prime {
    fn is_prime(&self) -> bool;
    fn is_prime_(&self, ps: &mut Primes) -> bool;
    fn get_factors(&self) -> Factors;
    fn get_factors_(&self, ps: &mut Primes) -> Factors;
}

extern crate num;
use num::Integer;

impl<T> Prime for T
where
    T: Integer,
    T: num::cast::ToPrimitive,
{
    fn is_prime(&self) -> bool {
        let mut primes: Primes = Primes::new();
        primes.primes_conf = PrimesConf::Constant;
        primes.n_is_prime(self.to_usize().unwrap_or_default())
    }

    fn is_prime_(&self, ps: &mut Primes) -> bool {
        ps.n_is_prime(self.to_usize().unwrap_or_default())
    }

    fn get_factors(&self) -> Factors {
        let mut primes: Primes = Primes::new();
        primes.primes_conf = PrimesConf::Constant;
        primes.factorice_number(self.to_usize().unwrap_or_default())
    }

    fn get_factors_(&self, ps: &mut Primes) -> Factors {
        ps.factorice_number(self.to_usize().unwrap_or_default())
    }
}
