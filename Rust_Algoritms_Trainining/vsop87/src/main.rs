fn calculate_var(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    var.iter()
        .fold(0_f64, |term, &(a, b, c)| term + a * (b + c * t).cos())
}

fn main() {
    println!(
        "calculate VSOP87 = {}",
        calculate_var(2.3232323, &[(3.456546545454, 32.54564564, 5.4564564)])
    );
}
