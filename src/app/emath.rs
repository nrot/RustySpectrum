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

fn sqrt(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.sqrt().into())
}

fn pow(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(vl, v, 0, 2);
    get_arguments!(pw, v, 1, 2);
    get_f64!(vl, vl);
    get_f64!(pw, pw);
    Ok(vl.powf(pw).into())
}

fn abs(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.abs().into())
}

fn exp(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.exp().into())
}
fn exp2(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.exp2().into())
}
fn ln(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.ln().into())
}
fn log(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(vl, v, 0, 2);
    get_arguments!(base, v, 1, 2);
    get_f64!(vl, vl);
    get_f64!(base, base);
    Ok(vl.log(base).into())
}

fn cbrt(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.cbrt().into())
}

fn sin(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.sin().into())
}

fn cos(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.cos().into())
}

fn tan(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(v, v, 0, 1);
    get_f64!(f, v);
    Ok(f.tan().into())
}
fn hypot(v: Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error> {
    get_arguments!(vl, v, 0, 2);
    get_arguments!(base, v, 1, 2);
    get_f64!(vl, vl);
    get_f64!(base, base);
    Ok(vl.hypot(base).into())
}

type ResolverFunc = fn(Vec<resolver::Value>) -> Result<resolver::Value, resolver::Error>;

const MATH_FUNCTIONS: &[(&str, ResolverFunc)] = &[
    ("sqrt", sqrt),
    ("pow", pow),
    ("abs", abs),
    ("exp", exp),
    ("exp2", exp2),
    ("ln", ln),
    ("log", log),
    ("cbrt", cbrt),
    ("sin", sin),
    ("cos", cos),
    ("tan", tan),
    ("hypot", hypot),
];

pub fn add_math(e: resolver::Expr) -> resolver::Expr {
    let mut e = e;
    for (name, f) in MATH_FUNCTIONS {
        e = e.function(*name, f);
    }
    e
}
