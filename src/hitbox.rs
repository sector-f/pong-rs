use ncollide_geometry::bounding_volume::AABB;
use nalgebra::Point2;

pub trait Hitbox {
    fn aabb(&self) -> AABB<Point2<f64>> {
        AABB::new(
            self.top_left(),
            self.bottom_right(),
        )
    }

    fn top_left(&self) -> Point2<f64> {
        Point2::new(self.left() as f64, self.top() as f64)
    }

    fn top_right(&self) -> Point2<f64> {
        Point2::new(self.right() as f64, self.top() as f64)
    }

    fn bottom_left(&self) -> Point2<f64> {
        Point2::new(self.left() as f64, self.bottom() as f64)
    }

    fn bottom_right(&self) -> Point2<f64> {
        Point2::new(self.right() as f64, self.bottom() as f64)
    }

    fn top(&self) -> i32;
    fn bottom(&self) -> i32;
    fn left(&self) -> i32;
    fn right(&self) -> i32;

    fn half_height(&self) -> f64;
    fn half_width(&self) -> f64;
}
