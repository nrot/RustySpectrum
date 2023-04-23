use clap::ValueEnum;
use rustfft::num_complex::Complex;


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum DisplayFun{
    #[default]
    Norm, 
    Real,
    Image,
    Custom
}

pub const DISPLAY_FN_NAME: &str = "convert";

pub fn get_display_fun(s: &DisplayFun, custom: String)->String{
    match s {
        DisplayFun::Norm => "fn convert(re, im) {hypot(re, im)}".into(),
        DisplayFun::Real => "fn convert(re, im) {re}".into(),
        DisplayFun::Image => "fm convert(re, im) {im}".into(),
        DisplayFun::Custom => custom,
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ComplexEval{
    pub re: f64,
    pub im: f64
}

impl From<Complex<f64>> for ComplexEval{
    fn from(v: Complex<f64>) -> Self {
        Self { re: v.re, im: v.im }
    }
}
