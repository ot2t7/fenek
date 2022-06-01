use crate::craft::Craft;
use crate::craft::distance;


pub fn intercept(intercepted_craft: &Craft, controlled_craft: &Craft) {
    let a = intercepted_craft.position.0;
    let b = intercepted_craft.position.1;
    let x = controlled_craft.position.0;
    let y = controlled_craft.position.1;
    let s = intercepted_craft.velocity as f32;
    let v = controlled_craft.velocity as f32;
    let t = intercepted_craft.bearing;

    let d = (f32::sqrt(4. * s.powf(2.0) * (a.powf(2.0) - 2. * a * x + b.powf(2.0) - 2. * b * y + x.powf(2.0) + y.powf(2.0)) * (-s.powf(2.0) * f32::sin(t).powf(2.0) - s.powf(2.0) * f32::cos(t).powf(2.0) + v.powf(2.0)) + s.powf(4.0) * (-2. * a * f32::cos(t) - 2. * b * f32::sin(t) + 2. * x * f32::cos(t) + 2. * y * f32::sin(t)).powf(2.0)) - s.powf(2.0) * (-2. * a * f32::cos(t) - 2. * b * f32::sin(t) + 2. * x * f32::cos(t) + 2. * y * f32::sin(t)))/(2. * (-s.powf(2.0) * f32::sin(t).powf(2.0) - s.powf(2.0) * f32::cos(t).powf(2.0) + v.powf(2.0)));
    let other_d = (s.powf(2.0) * (-(-2. * a * f32::cos(t) - 2. * b * f32::sin(t) + 2. * x * f32::cos(t) + 2. * y * f32::sin(t))) - f32::sqrt(4. * s.powf(2.0) * (a.powf(2.0) - 2. * a * x + b.powf(2.0) - 2. * b * y + x.powf(2.0) + y.powf(2.0)) * (-s.powf(2.0) * f32::sin(t).powf(2.0) - s.powf(2.0) * f32::cos(t).powf(2.0) + v.powf(2.0)) + s.powf(4.0) * (-2. * a * f32::cos(t) - 2. * b * f32::sin(t) + 2. * x * f32::cos(t) + 2. * y * f32::sin(t)).powf(2.0)))/(2. * (-s.powf(2.0) * f32::sin(t).powf(2.0) - s.powf(2.0) * f32::cos(t).powf(2.0) + v.powf(2.0)));    
    
    println!("intermediate is {}", (2. * (-s.powf(2.0) * f32::sin(t).powf(2.0) - s.powf(2.0) * f32::cos(t).powf(2.0) + v.powf(2.0))));

    println!("The other distance is {}", other_d);
    println!("The distance is {}", d);
}