use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
pub struct Factor {
    prime: usize,
    cant: usize,
}

impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.prime, self.cant)
    }
}

impl Factor {
    fn new(p: usize) -> Factor {
        Factor { prime: p, cant: 1 }
    }

    fn add_factor(&mut self) {
        self.cant += 1;
    }

    fn de_factor(&self) -> usize {
        usize::pow(self.prime, self.cant as u32)
    }
}

pub struct Factors {
    factor_vec: Vec<Factor>,
}

impl Deref for Factors {
    type Target = [Factor];

    fn deref(&self) -> &[Factor] {
        &self.factor_vec[..]
    }
}

impl DerefMut for Factors {
    fn deref_mut(&mut self) -> &mut [Factor] {
        &mut self.factor_vec[..]
    }
}

impl std::fmt::Display for Factors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp: String = String::default();
        for x in self.factor_vec.iter() {
            if tmp == "" {
                tmp = format!("{}", x);
            } else {
                tmp = format!("{}, {}", tmp, x);
            }
        }
        write!(f, "[{}]", tmp)
    }
}

impl Factors {
    pub fn new() -> Factors {
        Factors { factor_vec: Vec::new() }
    }

    pub fn de_factor(&self) -> usize {
        let mut result: usize = 1;
        for f in self.factor_vec.iter() {
            result *= f.de_factor();
        }
        return result;
    }

    pub fn get_factor(&mut self, p: usize) -> Option<&mut Factor> {
        for f in self.factor_vec.iter_mut() {
            if f.prime == p {
                return Some(f);
            }
        }
        return None;
    }

    pub fn add_factor(&mut self, p: usize) {
        match self.get_factor(p) {
            Some(f) => f.add_factor(),
            None => {
                let f = Factor::new(p);
                self.factor_vec.push(f);
            }
        }
    }
}
