pub struct Temperature {
    pub degrees: f32,
    pub scale: Scale,
}

pub enum Scale {
    Celsius,
    Fahrenheit,
}

impl Temperature {
    pub fn new(degrees: f32, scale: Scale) -> Self {
        Self { degrees, scale }
    }
    pub fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32.) * 5. / 9.,
        }
    }

    pub fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * 9.0 / 5. + 32.,
            Scale::Fahrenheit => self.degrees,
        }
    }
}
