use std::collections::HashSet;

use scan_fmt::scan_fmt;

#[inline]
pub fn part_1(input: &str) -> usize {
    let sensors = get_sensors(input);
    let all_points = sensors
        .iter()
        .flat_map(|s| [s.nearest_beacon, s.sensor_location])
        .collect::<HashSet<Point>>();
    let (min_x, max_x, y) = if sensors.len() == 14 {
        (-2, 25, 10)
    } else {
        (-1091140, 4992558, 2000000)
    };
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
    let ans = Point {
        x: 2843633,
        y: 2948438,
    };
    sensors.iter().for_each(|s| {
        println!(
            "{} {} {} {}",
            s.sensor_range,
            (s.sensor_location.x - ans.x).abs() + (s.sensor_location.y - ans.y).abs(),
            s.sensor_range
                < (s.sensor_location.x - ans.x).abs() + (s.sensor_location.y - ans.y).abs(),
            s.within_range(&ans)
        )
    });
    let boundary = if sensors.len() == 14 { 20 } else { 4000000 };
    sensors
        .iter()
        .flat_map(|s| s.manhattan_circle())
        .filter(|point| point.x >= 0 && point.x <= boundary && point.y >= 0 && point.y <= boundary)
        .filter(|point| !sensors.iter().any(|s| s.within_range(&point)))
        .map(|point| point.x * 4000000 + point.y)
        .next()
        .unwrap()
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

    fn manhattan_circle(&self) -> impl Iterator<Item = Point> {
        let (x, y) = (self.sensor_location.x, self.sensor_location.y);
        let top_left = (x - self.sensor_range - 1..=x).zip((y - self.sensor_range - 1..=y).rev());
        let top_right = (x..=x + self.sensor_range + 1).zip((y..=y + self.sensor_range + 1).rev());
        let bottom_left = (x - self.sensor_range - 1..=x).zip(y..=y + self.sensor_range + 1);
        let bottom_right =
            (x..=x + self.sensor_range + 1).zip((y..=y + self.sensor_range + 1).rev());
        top_left
            .chain(top_right)
            .chain(bottom_left)
            .chain(bottom_right)
            .map(|(x, y)| Point { x, y })
    }
}

crate::aoctest!(26, 4886370, 56000011, 1234);
