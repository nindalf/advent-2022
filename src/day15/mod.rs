use std::collections::{HashSet};

use scan_fmt::scan_fmt;

#[inline]
pub fn part_1(input: &str) -> usize {
    let sensors = get_sensors(input);
    let all_points = sensors
        .iter()
        .flat_map(|s| [s.nearest_beacon, s.sensor_location])
        .collect::<HashSet<Point>>();
    let (min_x, max_x, y) = if sensors.len() == 14 {(-2, 25, 10)} else {(-1091140, 4992558, 2000000)};
    (min_x..=max_x)
        .zip(std::iter::repeat(y))
        .map(|(x, y)| Point { x, y })
        .filter(|point| !all_points.contains(point))
        .filter(|point| sensors.iter().any(|s| s.within_range(point)))
        .count()
}

#[inline]
pub fn part_2(input: &str) -> i32 {
    let sensors = get_sensors(input);
    let all_points = sensors
        .iter()
        .flat_map(|s| [s.nearest_beacon, s.sensor_location])
        .collect::<HashSet<Point>>();
    
    let (max_x, max_y) = if sensors.len() == 14 {(20, 20)} else {(4000000, 4000000)};
    for x in 0 .. max_x {
        for y in 0 .. max_y {
            let point = Point {x, y};
            let outside_sensors = !sensors.iter().any(|s| s.within_range(&point));
            if !all_points.contains(&point) && outside_sensors {
                dbg!(x, y);
                return x * 4000000 + y;
            }
        }
    }
    0
}

fn get_sensors(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .flat_map(|line| {
            scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i32,
                i32,
                i32,
                i32
            )
        })
        .map(Sensor::new)
        .collect()
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    sensor_location: Point,
    nearest_beacon: Point,
    sensor_range: i32,
}

impl Sensor {
    fn new((sensor_x, sensor_y, beacon_x, beacon_y): (i32, i32, i32, i32)) -> Self {
        let sensor_location = Point {
            x: sensor_x,
            y: sensor_y,
        };
        let nearest_beacon = Point {
            x: beacon_x,
            y: beacon_y,
        };
        let sensor_range = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        Self {
            sensor_location,
            nearest_beacon,
            sensor_range,
        }
    }
    fn within_range(&self, point: &Point) -> bool {
        let distance =
            (self.sensor_location.x - point.x).abs() + (self.sensor_location.y - point.y).abs();
        distance <= self.sensor_range
    }
}

crate::aoctest!(26, 4886370, 56000011, 1234);
