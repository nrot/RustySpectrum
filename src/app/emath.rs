fn sqrt(v: f64) -> f64 {
    v.sqrt()
}

fn pow(vl: f64, pw: f64) -> f64 {
    vl.powf(pw)
}

fn abs(f: f64) -> f64 {
    f.abs()
}

fn exp(f: f64) -> f64 {
    f.exp()
}
fn exp2(f: f64) -> f64 {
    f.exp2()
}
fn ln(f: f64) -> f64 {
    f.ln()
}
fn log(vl: f64, base: f64) -> f64 {
    vl.log(base)
}

fn cbrt(f: f64) -> f64 {
    f.cbrt()
}

fn sin(f: f64) -> f64 {
    f.sin()
}

fn cos(f: f64) -> f64 {
    f.cos()
}

fn tan(f: f64) -> f64 {
    f.tan()
}
fn hypot(vl: f64, base: f64) -> f64 {
    vl.hypot(base)
}

pub fn add_math(e: &mut rhai::Engine) {
    e.register_fn("sqrt", sqrt)
        .register_fn("pow", pow)
        .register_fn("abs", abs)
        .register_fn("exp", exp)
        .register_fn("exp2", exp2)
        .register_fn("ln", ln)
        .register_fn("log", log)
        .register_fn("cbrt", cbrt)
        .register_fn("sin", sin)
        .register_fn("cos", cos)
        .register_fn("tan", tan)
        .register_fn("hypot", hypot);
}
//Before const func: Take time: 42.08s
//After : Take time: 40.24s
//Rc after: Take time: 28.45s
//Rhai Hypot each iter call convert: Take time: 14.17s
//Rhai (re + im)/2 each iter call convert: Take time: 6.17s
