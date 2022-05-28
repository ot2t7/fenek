use crate::craft::Craft;
use crate::craft::update_craft;
use crate::craft::distance;

#[derive(Debug)]
pub struct World {
    pub crafts: Vec<Craft>,
    pub contact: bool
}

pub fn tick(world: &mut World) {
    let delta: f32 = 1.0 / 60.0;
    for craft in world.crafts.iter_mut() {
        update_craft(craft, delta * craft.velocity as f32);
    }
    for craft in world.crafts.iter() { // This interception check is not very efficient
        for other_craft in world.crafts.iter() {
            let d = distance(craft, other_craft);
            if d < 1.0 && craft != other_craft {
                world.contact = true;
            }
        }
    }
}