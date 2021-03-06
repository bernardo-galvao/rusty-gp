pub fn rmse(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    // for semantics vs targets
    if x.len() != y.len() {
        // use assert!() ?
        panic!("cannot rmse vectors of different length!");
    }
    let n = x.len() as f32;
    let mapper = x.into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| ((x - y) as f32).powi(2));
    let se: f32 = mapper.sum(); // or .reduce(|| 0, |a, b| a + b)
    (se / n).sqrt()
}

pub fn add(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {
        panic!("cannot sum element-wise vectors of different length!");
    } else {
        x.into_iter()
            .zip(y.into_iter())
            .map(|(x, y)| x + y)
            .collect()
    }
}

pub fn subtract(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {
        panic!("cannot subtract element-wise vectors of different length!");
    } else {
        x.into_iter()
            .zip(y.into_iter())
            .map(|(x, y)| x - y)
            .collect()
    }
}

pub fn multiply(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {
        panic!("cannot multiply element-wise vectors of different length!");
    } else {
        x.into_iter() // or rayon::prelude::par_iter()
            .zip(y.into_iter())
            .map(|(x, y)| x * y)
            .collect()
    }
}

/*
fn dot_product(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.par_iter()
        .zip(vec2)
        .map(|e1, e2| e1 * e2)
        .collect()
}
*/

pub fn cosine(x: Vec<f32>) -> Vec<f32> {
    x.into_iter().map(|x| x.cos()).collect()
}

/// Element-wise logistic function. Mainly for use of the Geometric Semantic Genetic Programming
pub fn logistic_function(x: Vec<f32>) -> Vec<f32> {
    let e = |y: f32| -> f32 { 1.0 / (1.0 + (-y).exp()) };
    x.into_iter().map(e).collect()
}


pub fn divide(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {
        panic!("cannot divide element-wise vectors of different length!");
    }
    let lower_limit = 0.00001; // this can totally influence semantics!
    let protected_division = |(n, d): (f32, f32)| -> f32 {
        // numerator and denominator
        if d.abs() > lower_limit {
            return n / d;
        } else {
            return n / 1.0;
        }
    };
    x.into_iter()
        .zip(y.into_iter())
        .map(protected_division)
        .collect()
}
