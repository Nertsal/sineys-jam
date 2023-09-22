use geng::prelude::*;

/// A position on a cylinder.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositionCylinder<T> {
    pos: vec2<T>,
    world_width: T,
}

impl<T: Num> PositionCylinder<T> {
    /// Normalize a position in the world to a torus.
    pub fn from_world(mut pos: vec2<T>, world_width: T) -> Self {
        while pos.x < T::ZERO {
            pos.x += world_width;
        }
        while pos.x > world_width {
            pos.x -= world_width;
        }

        Self { pos, world_width }
    }

    /// The zero position.
    pub fn zero(world_width: T) -> Self {
        Self::from_world(vec2::ZERO, world_width)
    }

    pub fn to_world(self) -> vec2<T> {
        self.pos
    }

    pub fn world_width(self) -> T {
        self.world_width
    }

    /// Returns a delta from zero to `self`.
    pub fn as_dir(self) -> vec2<T> {
        Self::zero(self.world_width).delta_to(self)
    }

    /// Shift a position in-place by the given delta.
    pub fn shift(&mut self, delta: vec2<T>) {
        *self = self.shifted(delta);
    }

    /// Return a position shifted by the given delta
    pub fn shifted(self, delta: vec2<T>) -> Self {
        Self::from_world(self.to_world() + delta, self.world_width)
    }

    /// Calculate the delta from `self` to `towards`.
    /// Practically a subtraction operator.
    ///
    /// Panics if two positions don't have the same `world_size`.
    pub fn delta_to(self, towards: Self) -> vec2<T> {
        assert_eq!(
            self.world_width, towards.world_width,
            "two positions are not from the same world"
        );

        let mut delta = towards.to_world() - self.to_world();

        // Normalize delta
        let two = T::ONE + T::ONE;
        if delta.x.abs() * two > self.world_width {
            let signum = delta.x.signum();
            delta.x -= self.world_width * signum;
        }

        delta
    }
}

impl<T: Float> PositionCylinder<T> {
    pub fn to_world_f32(self) -> vec2<f32> {
        self.pos.map(T::as_f32)
    }

    /// Calculate the distance between two points.
    ///
    /// Panics if two positions don't have the same `world_size`.
    pub fn distance(self, other: Self) -> T {
        self.delta_to(other).len()
    }
}
