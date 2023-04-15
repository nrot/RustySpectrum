use clap::ValueEnum;
use rustfft::num_complex::Complex;


pub type ComplexDisplay = fn(&Complex<f32>)->f32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum DisplayFun{
    #[default]
    Norm, 
    Real,
    Image,
}

pub fn get_display_fun(s: &DisplayFun)->ComplexDisplay{
    match s {
        DisplayFun::Norm => norm_display,
        DisplayFun::Real => real_display,
        DisplayFun::Image => image_display,
    }
}

fn norm_display(s: &Complex<f32>)->f32{
    s.norm()
}

fn real_display(s: &Complex<f32>)->f32{
    s.re
}

fn image_display(s: &Complex<f32>)->f32{
    s.im
}