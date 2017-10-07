struct AstroUtil {
    right_ascension: f32,
    declination: f32,
    time: f32,
    latitude: f32,
    longitude: f32,
}

impl AstroUtil {
    fn set_right_ascension(&mut self, hours: u32, minutes: u32, seconds: u32) {
        self.right_ascension =
            ((hours as f32) + ((minutes as f32) / 60.) + ((seconds as f32) / 60. / 60.)) * 15.;
    }
}

impl AstroUtil {
    fn set_declination(&mut self, degrees: u32, minutes: u32, seconds: u32, cardinal: String) {
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
    fn set_latitude(&mut self, degrees: u32, minutes: u32, seconds: u32, cardinal: String) {
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
    fn set_longitude(&mut self, degrees: u32, minutes: u32, seconds: u32, cardinal: String) {
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
    fn set_time(&mut self, hours: u32, minutes: u32) {
        self.time = (hours as f32) + ((minutes as f32) / 60.);
    }
}

fn main() {
    let mut astro_util = AstroUtil {
        right_ascension: 0.,
        declination: 0.,
        time: 0.,
        latitude: 0.,
        longitude: 0.,
    };

    // Test Right Ascension
    astro_util.set_right_ascension(3, 27, 30);
    println!("Right Ascension is {0}", astro_util.right_ascension);

    // Test Declination
    astro_util.set_declination(36, 3, 27, String::from("N"));
    println!("Declination is {0}", astro_util.declination);

    // Test Latitude
    astro_util.set_latitude(39, 22, 4, String::from("N"));
    println!("Latitude is {0}", astro_util.latitude);

    // Test Longitude
    astro_util.set_longitude(84, 12, 5, String::from("W"));
    println!("Longitude is {0}", astro_util.longitude);

    // Test Time
    astro_util.set_time(16, 30);
    println!("Time is {0}", astro_util.time);
}
