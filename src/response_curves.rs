use std::fmt::{Debug, Display, Formatter};

/// The trait for implementing a Response Curve which transforms an arbitrary input.
/// Note that the output of transform will be capped between 0.0 and 1.0 by the framework.
pub trait InputTransform: Send + Sync + PartialEq {
    fn transform(&self, input: f32) -> f32;
}

#[derive(PartialEq, Clone)]
pub enum ResponseCurve {
    LinearCurve(Linear),
    PolynomialCurve(Polynomial),
    LogisticCurve(Logistic),
    PiecewiseLinear(PiecewiseLinear),
}

impl InputTransform for ResponseCurve {
    fn transform(&self, input: f32) -> f32 {
        match self {
            ResponseCurve::LinearCurve(x) => x.transform(input),
            ResponseCurve::PolynomialCurve(x) => x.transform(input),
            ResponseCurve::LogisticCurve(x) => x.transform(input),
            ResponseCurve::PiecewiseLinear(x) => x.transform(input),
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
    } else if x_shift.is_sign_positive() {
        write!(f, "(x - {:.})", x_shift)?;
    } else {
        write!(f, "(x + {:.})", x_shift.abs())?;
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
            ResponseCurve::PiecewiseLinear(r) => {
                write!(f, "PiecewiseLinear(")?;
                write!(f, "{:?}", r.points)?;
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

impl From<PiecewiseLinear> for ResponseCurve {
    fn from(value: PiecewiseLinear) -> Self {
        Self::PiecewiseLinear(value)
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

/// Custom curve defined by linear interpolation between the set of given points.
/// Out of bounds points to the left and right return the y value of the first or last
/// point respectively.
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseLinear {
    points: Vec<(f32, f32)>,
}

impl PiecewiseLinear {
    /// Creates a new [`PiecewiseLinear`] from the given Iterator.
    ///
    /// # Panics
    ///
    /// Panics if the iterator provides less than two points or the points are not
    /// strictly monotonically increasing in their x coordinates.
    pub fn new(points: impl IntoIterator<Item = (f32, f32)>) -> Self {
        let points = Vec::from_iter(points);
        if points.len() < 2 {
            panic!("You must provide at least two points to the PiecewiseLinear")
        }
        let prev_x = f32::NEG_INFINITY;
        for (x, _) in &points {
            if *x <= prev_x {
                panic!(
                    "Expected points which are strictly monotonically increasing in x. \
                    However, {x} is not greater than {prev_x}"
                )
            }
        }
        Self {
            points: Vec::from_iter(points),
        }
    }
}

impl InputTransform for PiecewiseLinear {
    fn transform(&self, input: f32) -> f32 {
        let mut index = 0;
        while index < self.points.len() {
            let (left_x, left_y) = self.points[index];
            let (right_x, right_y) = if index + 1 < self.points.len() {
                self.points[index + 1]
            } else {
                (f32::INFINITY, left_y)
            };

            if input >= left_x && input < right_x {
                return if left_y == right_y {
                    left_y
                } else {
                    // interpolate
                    let slope = (right_y - left_y) / (right_x - left_x);
                    left_y + slope * (input - left_x)
                };
            } else {
                index += 1;
            }
        }
        // we are out of bounds on the left, return the first y value
        self.points[0].1
    }
}

#[cfg(test)]
mod tests {
    use crate::response_curves::{InputTransform, PiecewiseLinear};

    #[test]
    fn test_piecewise_linear() {
        let piece_wise_linear = PiecewiseLinear::new(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(piece_wise_linear.transform(0.5), 0.5);
    }

    #[test]
    fn test_piecewise_multi() {
        let piece_wise_linear =
            PiecewiseLinear::new(vec![(0.0, 0.0), (0.1, 0.5), (0.3, 0.7), (1.0, 0.0)]);
        assert!(piece_wise_linear.transform(-0.1) - 0.0 <= 0.01);
        assert!(piece_wise_linear.transform(0.0) - 0.0 <= 0.01);
        assert!(piece_wise_linear.transform(0.1) - 0.5 <= 0.01);
        assert!(piece_wise_linear.transform(0.2) - 0.6 <= 0.01);
        assert!(piece_wise_linear.transform(0.3) - 0.7 <= 0.01);
        assert!(piece_wise_linear.transform(0.4) - 0.6 <= 0.01);
        assert!(piece_wise_linear.transform(0.5) - 0.5 <= 0.01);
        assert!(piece_wise_linear.transform(0.6) - 0.4 <= 0.01);
        assert!(piece_wise_linear.transform(0.7) - 0.3 <= 0.01);
        assert!(piece_wise_linear.transform(0.8) - 0.2 <= 0.01);
        assert!(piece_wise_linear.transform(0.9) - 0.1 <= 0.01);
        assert!(piece_wise_linear.transform(1.0) - 0.0 <= 0.01);
        assert!(piece_wise_linear.transform(1.1) - 0.0 <= 0.01);
    }

    #[test]
    fn test_piecewise_linear_out_of_bounds_left() {
        let piece_wise_linear = PiecewiseLinear::new(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(piece_wise_linear.transform(-0.1), 0.0);
    }

    #[test]
    fn test_piecewise_linear_out_of_bounds_right() {
        let piece_wise_linear = PiecewiseLinear::new(vec![(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(piece_wise_linear.transform(1.1), 1.0);
    }
}
