use actix_web::{get, web, HttpResponse, Responder};
use crate::signals::transform::fft;

#[get("/transform")]
pub async fn route_transform_fft() -> impl Responder {
    //let mut data = req.into_inner();
    //let result = fft(&mut data, false);

    let mut y = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0];
    let spectrum = fft(&mut y, false);
    let abs_spectrum: Vec<f32> = spectrum.iter().map(|cpx| cpx.abs()).collect();
    println!("Abs spectrum: {:?}", abs_spectrum);
    let inverse = fft(&mut spectrum.clone(), true);
    let abs_inverse: Vec<f32> = inverse.iter().map(|cpx| cpx.abs()).collect();
    println!("Inverse: {:?}", abs_inverse);
    HttpResponse::Ok().json(abs_spectrum)
}
