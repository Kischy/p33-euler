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

    let mut product = Fraction::new(1, 1);

    for (i, frac) in curious_fractions.iter().enumerate() {
        println!("Curious frac {}: {:?}", i, frac);
        product *= frac;
    }

    let answer_p33 = product.get_reduced_fraction().get_denominator();

    println!(
        "The answer to problem 33 of project Euler is {}.",
        answer_p33
    );
}
