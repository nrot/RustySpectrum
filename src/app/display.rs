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

macro_rules! get_arguments {
    ($nw:ident, $v:ident, $index:expr, $mx:expr) => {
        let Some($nw) = $v.get($index) else {
            return Err(resolver::Error::ArgumentsLess($mx));            
        };
    };
}

macro_rules! get_f64 {
    ($nw:ident, $v:ident) => {
        let resolver::Value::Number(tmp_value_98312774) = $v else {
            return Err(resolver::Error::ExpectedNumber);
        };
        let Some($nw) = tmp_value_98312774.as_f64() else {
            return Err(resolver::Error::Custom("Number must have convert to f64".into()));
        };
    };
}



pub fn add_math(e: resolver::Expr)->resolver::Expr{
    e.function("sqrt", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.sqrt().into())
    }).function("pow", |v|{
        get_arguments!(vl, v, 0, 2);
        get_arguments!(pw, v, 1, 2);
        get_f64!(vl, vl);
        get_f64!(pw, pw);
        Ok(vl.powf(pw).into())
    }).function("abs", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.abs().into())
    }).function("exp", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.exp().into())
    }).function("exp2", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.exp2().into())
    }).function("ln", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.ln().into())
    }).function("log", |v|{
        get_arguments!(vl, v, 0, 2);
        get_arguments!(base, v, 1, 2);
        get_f64!(vl, vl);
        get_f64!(base, base);
        Ok(vl.log(base).into())
    }).function("cbrt", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.cbrt().into())
    }).function("sin", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.sin().into())
    }).function("cos", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.cos().into())
    }).function("tan", |v|{
        get_arguments!(v, v, 0, 1);
        get_f64!(f, v);
        Ok(f.tan().into())
    }).function("hypot", |v|{
        get_arguments!(vl, v, 0, 2);
        get_arguments!(base, v, 1, 2);
        get_f64!(vl, vl);
        get_f64!(base, base);
        Ok(vl.hypot(base).into())
    })
}