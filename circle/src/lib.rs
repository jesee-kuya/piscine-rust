#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2)).sqrt()
    }
}

impl Circle {
    pub fn new(p1: f64, p2: f64, p3: f64) -> Self {
        Self {
            center: Point(p1, p2),
            radius: p3,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        let d = self.center.distance(other.center);
        self.radius - other.radius <= d && self.radius + other.radius >= d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {

        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);

        assert_eq!(point_a.distance(point_b), 1.4142135623730951);
    }

    fn test_new() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        assert_eq!(circle, Circle { center: Point(500.0, 500.0), radius: 150.0 });
    }

    fn test_diameter() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        assert_eq!(circle.diameter(), 300.0);
    }

    fn test_area() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        assert_eq!(circle.area(), 70685.83470577035);
    }

    fn test_intersect() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };

        assert_eq!(circle.intersect(circle1), false)
    }

}
