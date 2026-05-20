/// the different number of available sprites for planets.
/// if there are more planets types than sprites, different planet types
/// will share the same sprite.
pub(crate) const PLANET_SPRITE_NUM: usize = 7;

/// how fast each game "round" lasts, in seconds.
/// going under 0.2 seconds is not advisable.
pub(crate) const GAME_TICK: f32 = 0.25;

/// the different number of available sprites for explorers.
/// if there are more explorer types than sprites, different explorer types
/// will share the same sprite.
pub(crate) const EXP_SPRITE_NUM: usize = 2;

/// the radius of each planet sprite, in pixels.
//TODO turn this into a scalable unit according to the galaxy size at startup
pub(crate) const PLANET_RAD: f32 = 50.;

/// the radius of each celestial body (sunray and asteroids), in pixels.
pub(crate) const CELESTIAL_RAD: f32 = PLANET_RAD / 2.;

/// the radius of the total galaxy, in pixels.
pub(crate) const GALAXY_RADIUS: f32 = 250.;

/// the number of explorers to spawn in at startup
pub(crate) const EXPLORER_NUM: u32 = 2;

/// the size of the explorer sprite, in pixels.
/// scaled wrt the planet size so it looks semi consistent.
pub(crate) const EXPLORER_SIZE: f32 = 0.8 * PLANET_RAD;

/// where to place the tommy explorer graphically on the planet, wrt to the planet sprite
pub(crate) const EXP_TOMMY_OFFSET: (f32, f32) = (25., 25.);

/// where to place the mattia explorer graphically on the planet, wrt to the planet sprite
pub(crate) const EXP_MATTIA_OFFSET: (f32, f32) = (-25., -25.);

/// the standard sunray to asteroid ratio at the beginning of the game. 
/// higher number = more sunrays.
pub(crate) const DEFAULT_SUNRAY_RATIO: i32 = 80;