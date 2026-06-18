/// the different number of available sprites for planets.
/// if there are more planets types than sprites, different planet types
/// will share the same sprite.
pub(crate) const PLANET_SPRITE_NUM: usize = 7;

/// how fast each game "round" lasts, in seconds.
/// going under 0.2 seconds is not advisable.
pub(crate) const GAME_TICK: f32 = 0.3;

/// the different number of available sprites for explorers.
/// if there are more explorer types than sprites, different explorer types
/// will share the same sprite.
pub(crate) const EXP_SPRITE_NUM: usize = 2;

/// the parameters to follow to scale the planet size.
/// planet sizing follows an exponential curve, derived from discrete point fits
pub(crate) const PLANET_SCALING_A: f32 = 54.65;
pub(crate) const PLANET_SCALING_B: f32 = -0.03042;
pub(crate) const PLANET_SCALING_C: f32 = 3.062;

/// the radius of the total galaxy, in pixels.
pub(crate) const GALAXY_RADIUS: f32 = 250.;

/// the number of explorers to spawn in at startup
pub(crate) const EXPLORER_NUM: u32 = 2;

/// the standard sunray to asteroid ratio at the beginning of the game.
/// higher number = more sunrays.
pub(crate) const DEFAULT_SUNRAY_RATIO: i32 = 80;

/// Reference height to compute scaled values for the galaxy, depending
/// on the user's display settings
pub(crate) const BASE_HEIGHT: f32 = 960.;

/// number of planets in a randomly generated galaxy
pub(crate) const DEFAULT_RANDOM_PLANETS: u32 = 7;
