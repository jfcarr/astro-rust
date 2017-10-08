mod astroutil;

fn main() {
    let mut astro_util = astroutil::AstroUtil {
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
