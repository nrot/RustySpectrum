use rustfft::{FftPlanner, num_complex::Complex};

pub fn do_fft(src: &mut Vec<Complex<f32>>, size: usize){
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(size);
    fft.process(src);
}