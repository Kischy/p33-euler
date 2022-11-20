mod fraction;
use fraction::fraction::Fraction;

fn main() {
    let mut curious_fractions = vec![];

    for nummerator in 10..100 {
        for denominator in 10..100 {
            if nummerator >= denominator {
                continue;
            }
            let frac = Fraction::new(nummerator, denominator);
            if frac.is_curious_non_trival_fraction() {
                curious_fractions.push(frac);
            }
        }
    }

    // println!("{:?}", curious_fractions);
}
