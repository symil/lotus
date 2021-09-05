use crate::Point;

const EPSILON : f64 = 0.0000001;

pub fn get_segments_intersection(p1: Point, p2: Point, p3: Point, p4: Point) -> Option<Point> {
    let (dx1, dy1) = p1.vector_to(p2).as_tuple();
    let (dx2, dy2) = p3.vector_to(p4).as_tuple();
    let (x1, y1) = p1.as_tuple();
    let (x2, y2) = p3.as_tuple();
    let (min_a, max_a) = (0., 1.);
    let (min_b, max_b) = (0., 1.);

    if dx1.abs() < EPSILON {
        if dx2.abs() < EPSILON {
            None
        } else {
            get_segments_intersection(p3, p4, p1, p2)
        }
    } else {
        // if (threshold !== 0) {
        //     let da = threshold / computeDistance(p1, p2);
        //     let db = threshold / computeDistance(p3, p4);

        //     minA += da;
        //     maxA -= da;
        //     minB += db;
        //     maxB -= db;
        // }

        let b = (y1 - y2 + (dy1 * x2 - x1 * dy1) / dx1) / (dy2 - (dx2 * dy1) / dx1);
        let a = (x2 + b * dx2 - x1) / dx1;

        if a >= min_a && b > min_b && a < max_a && b < max_b {
            Some(Point {
                x: p1.x + a * dx1,
                y: p1.y + a * dy1
            })
        } else {
            None
        }
    }
}