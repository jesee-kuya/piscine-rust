#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    fn distance(&self, other: Point) -> f64 {
        ((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {

        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);

        assert_eq!(point_a.distance(point_b), 1.4142135623730951);
    }
}
