pub struct AstroUtil {
    pub right_ascension: f32,
    pub declination: f32,
    pub time: f32,
    pub latitude: f32,
    pub longitude: f32,
}

impl AstroUtil {
    pub fn set_right_ascension(&mut self, hours: u32, minutes: u32, seconds: u32) {
        self.right_ascension = ((hours as f32) * 15.) + ((minutes as f32) / 4.) +
            ((seconds as f32) / 240.);
    }
}

impl AstroUtil {
    pub fn set_declination(&mut self, degrees: u32, minutes: u32, seconds: u32, cardinal: String) {
        let calc_value = (degrees as f32) + ((minutes as f32) / 60.) +
            ((seconds as f32) / 60. / 60.);

        match cardinal.as_ref() {
            "N" => self.declination = calc_value,
            "S" => self.declination = -(calc_value),
            _ => self.declination = calc_value,  // default to N
        }
    }
}

impl AstroUtil {
    pub fn set_latitude(&mut self, degrees: u32, minutes: u32, seconds: u32, cardinal: String) {
        let calc_value = (degrees as f32) + ((minutes as f32) / 60.) +
            ((seconds as f32) / 60. / 60.);

        match cardinal.as_ref() {
            "N" => self.latitude = calc_value,
            "S" => self.latitude = -(calc_value),
            _ => self.latitude = calc_value,  // default to N
        }
    }
}

impl AstroUtil {
    pub fn set_longitude(&mut self, degrees: u32, minutes: u32, seconds: u32, cardinal: String) {
        let calc_value = (degrees as f32) + ((minutes as f32) / 60.) +
            ((seconds as f32) / 60. / 60.);

        match cardinal.as_ref() {
            "E" => self.longitude = calc_value,
            "W" => self.longitude = -(calc_value),
            _ => self.longitude = calc_value,  // default to E
        }
    }
}

impl AstroUtil {
    pub fn set_time(&mut self, hours: u32, minutes: u32) {
        self.time = (hours as f32) + ((minutes as f32) / 60.);
    }
}
