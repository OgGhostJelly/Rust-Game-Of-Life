const MAX_HEALTH: u8 = 255;

#[derive(Clone, Copy, Debug)]
pub struct Player {
    health: u8,
    strength: u8,
    luck: u8,
}

impl Player {
    pub const fn new(health: u8, strength: u8, luck: u8) -> Self {
        Self {
            health,
            strength,
            luck,
        }
    }
}

impl Player {
    pub fn health(&self) -> &u8 {
        &self.health
    }

    pub fn strength(&self) -> &u8 {
        &self.strength
    }

    pub fn luck(&self) -> &u8 {
        &self.luck
    }
}

impl Player {
    pub fn change_health_by(&mut self, amount: u8) {
        self.health = clamp(self.health - amount, 0, MAX_HEALTH)
    }
}

fn clamp<T>(value: T, minimum: T, maximum: T) -> T
where
    T: Ord,
{
    if value < minimum {
        minimum
    } else if value > maximum {
        maximum
    } else {
        value
    }
}
