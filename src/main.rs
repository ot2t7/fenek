mod craft;
mod cruise;
mod world;
mod math;

use craft::Craft;
use cruise::intercept;
use world::World;
use world::tick;

fn main() {
    let mut world = World {
        crafts: vec![],
        contact: false
    };
    world.crafts.push(Craft {
        position: (0.0, 0.0),
        bearing: 0.0,
        velocity: 60
    });
    world.crafts.push(Craft {
        position: (10.0, 0.0),
        bearing: 180.0,
        velocity: 60
    });

    

    println!("{:?}", world);
    loop {
        tick(&mut world);
        println!("{:?}", world);
        if world.contact == true {
            println!("contact!");
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
