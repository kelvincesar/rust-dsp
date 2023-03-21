
// from folder signals, import frequency
mod signals {
    pub mod transform;
    pub mod basics;
}
use signals::transform::fft;

fn main() {
    let mut y = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0];
    let spectrum = fft(&mut y, false);
    let abs_spectrum: Vec<f32> = spectrum.iter().map(|cpx| cpx.abs()).collect();
    println!("Abs spectrum: {:?}", abs_spectrum);
    let inverse = fft(&mut spectrum.clone(), true);
    let abs_inverse: Vec<f32> = inverse.iter().map(|cpx| cpx.abs()).collect();
    println!("Inverse: {:?}", abs_inverse);
}
// https://dev.to/dbanty/replacing-fastapi-with-rust-part-3-trying-actix-32lp
// https://github.com/kelvincesar/rust_learning/blob/cefb2b0197062e5f42715fd519018c7dfc20fbd1/rust_book/ch11_tests/adder/src/lib.rs