use num::Complex;

pub fn mean(y: &[f64]) -> f64 {
    let n = y.len() as f64;
    let m = y.iter().sum::<f64>();
    m / n
}

pub fn absolute(y: &Vec<Complex<f32>>) -> Vec<f32> {
    y.iter().map(|cpx| cpx.norm()).collect()
}


pub fn centralize(y: &[f64]) -> Vec<f64> {
    let n = y.len() as f64;
    let m = mean(y);
    y.iter().map(|value| value - m).collect()
}
