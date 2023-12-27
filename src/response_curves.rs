use std::fmt::{Debug, Display, Formatter};

/// The trait for implementing a Response Curve which transforms an arbitrary input.
/// Note that the output of transform will be capped between 0.0 and 1.0 by the framework.
pub trait InputTransform: Send + Sync + PartialEq {
    fn transform(&self, input: f32) -> f32;
}

#[derive(PartialEq)]
pub enum ResponseCurve {
    LinearCurve(Linear),
    PolynomialCurve(Polynomial),
    LogisticCurve(Logistic),
}

impl InputTransform for ResponseCurve {
    fn transform(&self, input: f32) -> f32 {
        match self {
            ResponseCurve::LinearCurve(x) => x.transform(input),
            ResponseCurve::PolynomialCurve(x) => x.transform(input),
            ResponseCurve::LogisticCurve(x) => x.transform(input),
        }
    }
}

fn write_slope(f: &mut Formatter<'_>, slope: f32) -> std::fmt::Result {
    if slope == 1.0 {
        return Ok(());
    } else if slope == -1.0 {
        write!(f, "-")?;
    } else if slope.abs() < 0.0001 {
        write!(f, "{:+.2e}", slope)?;
    } else {
        write!(f, "{:.}", slope)?;
    }
    Ok(())
}

fn write_x(f: &mut Formatter<'_>, x_shift: f32) -> std::fmt::Result {
    if x_shift == 0.0 {
        write!(f, "x")?
    } else {
        if x_shift.is_sign_positive() {
            write!(f, "(x - {:.})", x_shift)?;
        } else {
            write!(f, "(x + {:.})", x_shift.abs())?;
        }
    }
    Ok(())
}

fn write_y_shift(f: &mut Formatter<'_>, y_shift: f32) -> std::fmt::Result {
    if y_shift != 0.0 {
        if y_shift.is_sign_positive() {
            write!(f, " + {:.}", y_shift)?;
        } else {
            write!(f, " - {:.}", y_shift.abs())?;
        }
    }
    Ok(())
}

fn write_float(f: &mut Formatter<'_>, val: f32) -> std::fmt::Result {
    if val.abs() < 0.0001 {
        write!(f, "{:+.2e}", val)?;
    } else {
        write!(f, "{:.}", val)?;
    }
    Ok(())
}

impl Display for ResponseCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCurve::LinearCurve(r) => {
                write!(f, "Linear(")?;
                write_slope(f, r.slope)?;
                write_x(f, r.x_shift)?;
                write_y_shift(f, r.y_shift)?;
                write!(f, ")")
            }
            ResponseCurve::PolynomialCurve(r) => {
                write!(f, "Poly(")?;
                write_slope(f, r.slope)?;
                write_x(f, r.x_shift)?;
                write!(f, "^{:.}", r.k)?;
                write_y_shift(f, r.y_shift)?;
                write!(f, ")")
            }
            ResponseCurve::LogisticCurve(r) => {
                write!(f, "Logistic(k=")?;
                write_float(f, r.k)?;
                write!(f, ",x_shift=")?;
                write_float(f, r.x_shift)?;
                write!(f, ",y_shift=")?;
                write_float(f, r.y_shift)?;
                write!(f, ")")
            }
        }
    }
}

impl From<Linear> for ResponseCurve {
    fn from(value: Linear) -> Self {
        Self::LinearCurve(value)
    }
}

impl From<Polynomial> for ResponseCurve {
    fn from(value: Polynomial) -> Self {
        Self::PolynomialCurve(value)
    }
}

impl From<Logistic> for ResponseCurve {
    fn from(value: Logistic) -> Self {
        Self::LogisticCurve(value)
    }
}

/// Implements the formula `y = slope * (x - x_shift) + y_shift`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Linear {
    pub slope: f32,
    pub x_shift: f32,
    pub y_shift: f32,
}

impl Linear {
    pub fn new(slope: f32) -> Self {
        Self {
            slope,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }

    pub fn shifted(self, x_shift: f32, y_shift: f32) -> Self {
        Self {
            slope: self.slope,
            x_shift,
            y_shift,
        }
    }
}

impl InputTransform for Linear {
    fn transform(&self, input: f32) -> f32 {
        self.slope * (input - self.x_shift) + self.y_shift
    }
}
/// Implements the formula `y = slope * (x - x_shift) ^ k + y_shift`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Polynomial {
    pub slope: f32,
    pub k: f32,
    pub x_shift: f32,
    pub y_shift: f32,
}

impl Polynomial {
    pub fn new(slope: f32, k: f32) -> Self {
        Self {
            slope,
            k,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }

    pub fn shifted(self, x_shift: f32, y_shift: f32) -> Self {
        Self {
            slope: self.slope,
            k: self.k,
            x_shift,
            y_shift,
        }
    }
}

impl InputTransform for Polynomial {
    fn transform(&self, input: f32) -> f32 {
        self.slope * (input - self.x_shift).powf(self.k) + self.y_shift
    }
}

/// Implements the formula `y = (1 / (1 + k ^ - (x - x_shift))) + y_shift`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Logistic {
    pub k: f32,
    pub x_shift: f32,
    pub y_shift: f32,
}

impl Logistic {
    pub fn new(k: f32) -> Self {
        Self {
            k,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }

    pub fn shifted(self, x_shift: f32, y_shift: f32) -> Self {
        Self {
            k: self.k,
            x_shift,
            y_shift,
        }
    }
}

impl InputTransform for Logistic {
    fn transform(&self, input: f32) -> f32 {
        1.0 / (1.0 + self.k.powf(-input + self.x_shift)) + self.y_shift
    }
}
