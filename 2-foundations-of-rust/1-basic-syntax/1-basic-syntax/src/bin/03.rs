fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut valoare_maxima = input[0];
    let mut valoare_minima = input[0];
    for i in 1..input.len() {
        if input[i] > valoare_maxima {
            valoare_maxima = input[i];
        }
        if input[i] < valoare_minima {
            valoare_minima = input[i];
        }

    }
    println!("{} is largest and {} is smallest", valoare_maxima, valoare_minima);
}


