# Spritesheet
### for rust lang piston

A more traditional style frame based spritesheet animation

## Example
running this example:
First, if you don't have rust installed you should install rust.
I recommend installing through [rustup](https://rustup.rs/).

- `git clone` this repo
- cd into `piston-spritesheet`
- `git fetch --all`
- `git checkout example/walking_sprite`
- `cargo run`

If you're machine doesn't support OpenGL v3.2 you can change the version
in `src/main.rs` to a lower version. This may or may not work.

## Usage

import and use spritesheet crate. Currently requires piston crate
```
extern crate spritesheet;

use spritesheet::{Animation};
```

create a new spritesheet with a path to your spritesheet image
```
let mut player_spritesheet = spritesheet::SpriteSheet::new(Path::new("assets/sprites/player.png"));
```

define frames (x, y coords on image)
This can be done directly in code, or preferably by creating a json file and loading that.

*code*
```
let walk_anim = Animation::new("walk".to_string(), 60, vec![
    (0.0, 0.0),
    (30.0, 0.0),
    (60.0, 0.0),
]);
```

*json*
```
let walk_anim = Animation::load_from_json("assets/animations/player.json");
```

example animation json (player.json)
```
{
    "walk": {
        "name": "walk",
        "frames": [
            [0.0, 0.0],
            [30.0, 0.0],
            [60.0, 0.0]
        ]
    },
    "jump": {
        "name": "jump",
        "frames": [
            [30.0, 0.0],
            [30.0, 30.0],
            [30.0, 60.0]
        ]
    }
}
```

finally add the animation to the spritesheet
```
player_spritesheet.add_animation(walk_anim);
```

from the variable holding the spritesheet you create you can `play` an animation by `name`
```
player_spritesheet.play("walk");
```

you can also cancel/stop the current animation frame
```
player_spritesheet.cancel(None);
```

you can flip the image or set the orientation using the `Orientation` type
```
use spritesheet::{Orientation};

player_spritesheet.set_orientation_h(Orientation::Flipped); // flip the image horizontally
player_spritesheet.set_orientation_v(Orientation::Flipped); // flip the image vertically
player_spritesheet.set_orientation(Orientation::Flipped, Orientation::Normal); // flip horizontally, normal vertically
```

during update you can set framesize, frameview, scaling, and position
```
player_spritesheet.set_frame_size(30.0, 30.0); // width, height
player_spritesheet.set_frame_view(0.0, 0.0);
player_spritesheet.set_scale(2.0, 2.0); // width, height
player_spritesheet.set_pos(player.x, 200.0, 0.0); // x,y, z
```

call render with context transform and graphics
```
// inside self.gl.draw(args.viewport(), | c, g | {});
player_spritesheet.render(c.transform, g);
```

This is a work in progress. I just wanted to quickly have sprite based frame animation. There are many improvements
to be made, but it does work more in a way that I would expect to do animation in 2D more than what I could 
currently find in piston.
