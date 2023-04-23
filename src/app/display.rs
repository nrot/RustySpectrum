use clap::ValueEnum;
use rustfft::num_complex::Complex;

pub type ComplexDisplay = resolver::Expr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum DisplayFun{
    #[default]
    Norm, 
    Real,
    Image,
    Custom
}

pub fn get_display_fun(s: &DisplayFun, custom: String)->ComplexDisplay{
    match s {
        DisplayFun::Norm => resolver::Expr::new("hypot(c.re, c.im)"),
        DisplayFun::Real => resolver::Expr::new("c.re"),
        DisplayFun::Image => resolver::Expr::new("c.im"),
        DisplayFun::Custom => resolver::Expr::new(custom),
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
