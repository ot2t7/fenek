use crate::craft::Craft;
use crate::craft::distance;
use crate::math::Circle;

use geo_types::coord;
use geo::{Line, Coordinate};
use geo::algorithm::line_intersection::{line_intersection, LineIntersection};

fn find_vesica_line(circle1: &Circle, circle2: &Circle, d_craft: f32) {
    // Following variable names may be taken from this graphic:
    // https://diego.assencio.com/images/mathematics/circle-circle-intersection.svg

    // Find d1 and d2 using a formula provided.
    let d1 = (circle1.radius.powf(2.0) - circle2.radius.powf(2.0) + d_craft.powf(2.0)) / (2.0 * d_craft);
    let d2 = d_craft - d1;

    let vesica_height_top = (circle1.radius.powf(2.0) - d1.powf(2.0)).sqrt();
}

pub fn intercept(intercepted_craft: &Craft, controlled_craft: &mut Craft) {
    // Some equations come from:
    // https://diego.assencio.com/?index=8d6ca3d82151bad815f78addf9b5c1c6

    // We create two circles with the center at each aircraft, with the 
    // radius equal to the distance between the two aircraft.
    let d_craft = distance(intercepted_craft, controlled_craft);
    let ic_circle = Circle {center: intercepted_craft.position, radius: d_craft};
    let cc_circle = Circle {center: controlled_craft.position, radius: d_craft};

    // Both circles will overlap with eachother, which forms a Vesica
    // piscis (https://en.wikipedia.org/wiki/Vesica_piscis). We need
    // to find the line which represents the height of this Vesica piscis.
    let vesica_line = find_vesica_line(&ic_circle, &cc_circle, d_craft);
}