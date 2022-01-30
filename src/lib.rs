/*
 * The idea here is to have 4 main projects
 *      1) Primes: A struct containing all the primes in order to query it
 *
 *      2) Factors: A struct for getting the factorization of a number
 *
 *      3) Provide extensions to all reasonable primitive types for factorization and primality check such as:
 *              let my_int: u32 = 123;
 *              let fact_a: Factors = my_int.factorize();
 *              if my_int.is_prime() {...}
 *
 *         also if possible:
 *              let my_primes: Primes = Primes::new(); //idealy some kind of global!
 *              let my_int: i32 = 1234;
 *              let fact_a: Factors = my_int.factorize(&mut my_primes); //share the use of a single prime struct!
 *              if my_int.is_prime(&mut my_primes) {...}
 *
 *      4) QckDivChk: Is just a small enum for checking divisibility rules, in order to avoid doing (x % 2 == 0)
 *                    wich for large numbers can be costly
 */

mod factors;
mod primes;
mod qck_div_chk;

#[cfg(test)]
mod tests {
    use crate::primes::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_extensions_a() {
        let a: u32 = 11;
        assert!(a.is_prime());

        assert!(!15.is_prime());
    }

    #[test]
    fn test_extensions_b() {
        let mut primes: Primes = Primes::new();

        let a: u32 = 127;
        assert!(a.is_prime_(&mut primes));

        assert!(!15.is_prime_(&mut primes));
    }

    #[test]
    fn test_factorisation_a() {
        let a: u32 = 127;
        let fact_a = a.get_factors();
        assert_eq!(format!("{}", fact_a), "[(127, 1)]");

        let b: u32 = 100;
        let fact_b = b.get_factors();
        assert_eq!(format!("{}", fact_b), "[(2, 2), (5, 2)]");
    }
    #[test]
    fn test_factorisation_b() {
        let mut primes: Primes = Primes::new();

        let a: u32 = 127;
        let fact_a = a.get_factors_(&mut primes);
        assert_eq!(format!("{}", fact_a), "[(127, 1)]");

        let b: u32 = 100;
        let fact_b = b.get_factors_(&mut primes);
        assert_eq!(format!("{}", fact_b), "[(2, 2), (5, 2)]");
    }

    #[test]
    fn test_primes_iter() {
        let mut primes: Primes = Primes::new();
        primes.primes_conf = PrimesConf::Constant;

        let x = 32165489;
        println!("{} is prime? {}", x, x.is_prime_(&mut primes));

        let y = 6548178;
        println!("{} is prime? {}", y, y.is_prime_(&mut primes));
        println!("{} Factors => {}", y, y.get_factors_(&mut primes));
        assert!(true);
/*
        let mut p_a: usize;
        let mut p_b: usize = 0;
        for p in primes.iter() {
            p_a = p_b;
            p_b = *p;
            let r: f64 = (p_a as f64) / (p_b as f64);
            println!("{} / {} = {}", p_a, p_b, r);
        }
*/
    }
}
