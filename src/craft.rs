#[derive(Debug, PartialEq)]
pub struct Craft {
    pub position: (f32, f32), // (x, y)
    pub bearing: f32,
    pub velocity: u32 // KM per hour
}

pub fn update_craft(craft: &mut Craft, delta_distance: f32) {
    let mut c_pos = craft.position;
    c_pos.0 = c_pos.0 + (delta_distance * craft.bearing.to_radians().cos()); // cos(bearing) = x/delta_distance
    c_pos.1 = c_pos.1 + (delta_distance * craft.bearing.to_radians().sin()); // sin(bearing) = x/delta_distance
    craft.position = c_pos;
}

pub fn distance(craft: &Craft, other_craft: &Craft) -> f32 {
    return ((other_craft.position.0 - craft.position.0).powf(2.0) + (other_craft.position.1 - craft.position.1).powf(2.0)).sqrt();
}