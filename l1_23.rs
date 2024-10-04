pub struct Point{
    x: f64,
    y: f64
}

impl Point{
    pub fn new(x: f64, y: f64) -> Self{
        Point {
            x: x,
            y: y
        }
    }

    pub fn dunstance_calculate(&self, second_point: Point) -> f64{
        ((self.x - second_point.x) * (self.x - second_point.x) + (self.y - second_point.y)*(self.y - second_point.y)).sqrt()
    }
}
