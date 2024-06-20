# bevy_dodgy_2d

A port of the [dodgy](https://github.com/andriyDev/dodgy) crate developed 
by [andriyDev](https://github.com/andriyDev) for the [Bevy](https://github.com/bevyengine/bevy) game engine.
This crate use most of angriDev's code, but adapted to fit the ECS framework of Bevy.
The crate is used to compute local collision avoidance (specifically ORCA) for agents.

## Why local collision avoidance?

Characters in video games generally need to find paths to navigate around the
game world. Once this is done, the path needs to be followed. The trouble occurs
when characters start getting in the way of each other. As paths are not
generally regenerated every game frame, other characters cannot be taken into
account. Local collision avoidance provides cheap avoidance for characters even
in high-density situations.

## Which local collision avoidance?

There are several algorithms for local collision avoidance. This crate
implements [ORCA](https://gamma.cs.unc.edu/ORCA/).

This crate is essentially a port of [RVO2](https://gamma.cs.unc.edu/RVO2/) to
Rust. Several changes have been made: tests have been written, code more
commented, and the public API made more flexible.

## Example




## License

License under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Attribution

dodgy_2d contains code ported from RVO2. See
[original_license.txt](original_license.txt).
