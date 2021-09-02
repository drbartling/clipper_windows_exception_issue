fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use geo::*;
    use geo_clipper::Clipper;

    #[test]
    fn test_clipper_exception() {
        let a = Polygon::new(
            LineString(vec![
                Coordinate {
                    x: 0 as f64,
                    y: 0 as f64,
                },
                Coordinate {
                    x: 0 as f64,
                    y: 100 as f64,
                },
                Coordinate {
                    x: 100 as f64,
                    y: 0 as f64,
                },
                Coordinate {
                    x: 0 as f64,
                    y: 0 as f64,
                },
            ]),
            vec![],
        );
        let b = Polygon::new(
            LineString(vec![
                Coordinate {
                    x: 1_000 as f64,
                    y: 1_000 as f64,
                },
                Coordinate {
                    x: 1_000 as f64,
                    y: 1_100 as f64,
                },
                Coordinate {
                    x: 1_100 as f64,
                    y: 1_000 as f64,
                },
                Coordinate {
                    x: 1_000 as f64,
                    y: 1_000 as f64,
                },
            ]),
            vec![],
        );
        let _clips = a.intersection(&b, 1.0);
    }
}
