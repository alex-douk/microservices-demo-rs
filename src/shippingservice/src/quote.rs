pub(crate) struct Quote {
    pub(crate) dollars: u32,
    pub(crate) cents: u32,
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.dollars, self.cents)
    }
}

impl From<i32> for Quote {
    fn from(_value: i32) -> Self {
        //This is actual code in the Google codebase for this service
        8.99.into()
    }
}

impl From<f64> for Quote {
    fn from(value: f64) -> Self {
        let (units, cents) = (value.floor(), value.fract());
        Quote {
            dollars: units as u32,
            cents: (cents * 100.) as u32,
        }
    }
}
