pub struct WGS84 {
    pub lon: f64,
    pub lat: f64
}

pub const EARTH_RADIUS : f64 = 6372797.560856;

pub fn great_circle_distance(lhs: &WGS84, rhs: &WGS84) -> f64 {
    let lat1 = lhs.lat.to_radians();
    let lon1 = lhs.lon.to_radians();
    let lat2 = rhs.lat.to_radians();
    let lon2 = rhs.lon.to_radians();

    let x = (lon2 - lon1) * ((lat1 + lat2) / 2.0).cos();
    let y = lat2 - lat1;
    x.hypot(y) * EARTH_RADIUS
}

pub fn haversine(lhs: &WGS84, rhs: &WGS84) -> f64{
    let dlat1 = lhs.lat.to_radians();
    let dlong1 = lhs.lon.to_radians();
    let dlat2 = rhs.lat.to_radians();
    let dlong2 = rhs.lon.to_radians();

    let dlong = dlong1 - dlong2;
    let dlat = dlat1 - dlat2;

    let aharv = (dlat / 2.0).sin().powi(2) + (dlong / 2.0).sin().powi(2) * dlat1.cos() * dlat2.cos();
    let charv = 2.0 * aharv.sqrt().atan2((1.0 - aharv).sqrt());
    EARTH_RADIUS * charv
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn edge_cases() {
        assert!((haversine(&WGS84 {lon: 0.0, lat: 0.0},   &WGS84 {lon: 180.0, lat: 0.0})  - PI * EARTH_RADIUS).abs()  < 0.1);
        assert!((haversine(&WGS84 {lon: 0.0, lat: 0.0},   &WGS84 {lon: -180.0, lat: 0.0}) - PI * EARTH_RADIUS).abs()  < 0.1);
        assert!((haversine(&WGS84 {lon: 180.0, lat: 0.0}, &WGS84 {lon: -180.0, lat: 0.0}) - 0.0).abs()                < 0.1);
        assert!((haversine(&WGS84 {lon: 0.0, lat: 90.0},  &WGS84 {lon: 0.0, lat: -90.0})  -  PI * EARTH_RADIUS).abs() < 0.1);
    }
}
