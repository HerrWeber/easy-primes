#[derive(Debug)]
pub enum QckDiv {
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
}

impl std::fmt::Display for QckDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            QckDiv::Two => '2',
            QckDiv::Three => '3',
            QckDiv::Five => '5',
            QckDiv::Seven => '7',
        };
        write!(f, "{}", printable)
    }
}

impl QckDiv {
    pub fn check_div(&self, n: usize) -> bool {
        let radix: u32 = 10;

        let mut n_aux: String = format!("{}", n);

        let result = match self {
            QckDiv::Two => {
                let digit = n_aux.pop().unwrap();
                vec![0, 2, 4, 6, 8].contains(&digit.to_digit(radix).unwrap())
            }
            QckDiv::Three => {
                while n_aux.len() > 1 {
                    n_aux = format!(
                        "{}",
                        n_aux
                            .chars()
                            .map(|c| c.to_digit(radix).unwrap())
                            .sum::<u32>()
                    );
                }
                vec![3, 6, 9].contains(&n_aux.parse::<i32>().unwrap())
            }
            QckDiv::Five => {
                let digit = n_aux.pop().unwrap();
                vec![0, 5].contains(&digit.to_digit(radix).unwrap())
            }
            QckDiv::Seven => {
                while n_aux.len() > 2 {
                    let digit = n_aux.pop().unwrap();
                    let right: u32 = &digit.to_digit(radix).unwrap() * 2;

                    let new: i64 = (n_aux.parse::<u32>().unwrap() as i64) - (right as i64);
                    n_aux = format!("{}", new.abs());
                }
                let x = &n_aux.parse::<u32>().unwrap();
                vec![7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91, 98].contains(x)
            }
        };
        return result;
    }
}
