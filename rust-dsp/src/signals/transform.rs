use std::f32::consts::PI;

// This function reverses the bits of an unsigned integer n
// of width 'width' and returns the result.
fn reverse_bits(mut n: usize, width: usize) -> usize {
    let mut res = 0;
    for _ in 0..width {
        // Extract the least significant bit of n using bitwise AND
        // with 1. Bitwise OR this bit with the result so far, which
        // has been shifted left by 1. Then shift n to the right by 1.
        res = (res << 1) | (n & 1);
        n >>= 1;
    }
    res
}

// This function computes the Fast Fourier Transform (FFT) or its inverse
// of a real-valued signal y of length n. The input signal is modified in place.
// If inverse is true, it computes the Inverse FFT (IFFT) instead.
pub fn fft(y: &mut [f32], inverse: bool) -> Vec<f32> {
    // Set the power factor according to the desired FFT direction.
    let power_factor = if inverse { 2.0 } else { -2.0 } * PI;
    let n = y.len();

    // Compute the width of the bit-reversed indices.
    let width = (n as f32).log2().ceil() as usize;

    // Allocate an array to hold the transformed signal.
    let mut spectrum = vec![0.0; n];

    // Perform the bit-reversal permutation to obtain the input
    // in the correct order for the Cooley-Tukey algorithm.
    for i in 0..n {
        spectrum[reverse_bits(i, width)] = y[i];
    }
    // Apply the Cooley-Tukey algorithm to compute the FFT or IFFT.
    for i in 1..=width {
        let m = 1 << i;
        // Compute the complex twiddle factor omega_m.
        let om_m = (power_factor / m as f32).exp();
        for j in (0..n).step_by(m) {
            let mut om = 1.0;
            let half_m = m >> 1;
            for k in 0..half_m {
                // Compute the butterfly operation between
                // spectrum[j + k] and spectrum[j + k + half_m].
                let t = om * spectrum[j + k + half_m];
                let u = spectrum[j + k];
                spectrum[j + k] = u + t;
                spectrum[j + k + half_m] = u - t;
                om *= om_m;
            }
        }
    }
    // Normalize the output if performing the IFFT.
    if inverse {
        let scale = 1.0 / n as f32;
        spectrum.iter().map(|cpx| cpx * scale).collect()
    } else {
        spectrum
    }
}
