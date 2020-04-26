
use geojson::{GeoJson, Value};
use geo_booleanop::boolean::BooleanOp;
use geo_types::{LineString, Polygon};

fn main() {
    let poly1 = Polygon::new(LineString::from(vec![
      (0., 0.),
      (1., 1.),
      (1., 0.),
      (0., 0.),
    ]), vec![]);

    let poly2 = Polygon::new(LineString::from(vec![
      (0., 0.),
      (1., 1.),
      (1., 0.),
      (0., 0.),
    ]), vec![]);

    let union = poly1.union(&poly2);

    println!("Hello, world!");
}
