use std::{cmp::max, fmt};

const SQRT_3: f64 = 1.73205080756888;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub(crate) fn new(x: f64, y: f64) -> Point {
        return Point { x, y };
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{};{}}}", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}
impl Hex {
    pub fn new_axial(q: i32, r: i32) -> Hex {
        return Hex { q, r, s: -q - r };
    }
    #[allow(dead_code)]
    pub fn new_cube(q: i32, r: i32, s: i32) -> Hex {
        return Hex { q, r, s };
    }
    #[allow(dead_code)]
    pub fn equals(&self, other: &Self) -> bool {
        return self.q == other.q && self.r == other.r && self.s == other.s;
    }
    pub fn hex_to_pixel(&self, layout_size: (f32, f32), scale: (f32, f32)) -> Point {
        let result = hex_to_pixel_pointy_layout(*self, layout_size, scale);

        return result;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FractionalHex {
    pub q: f64,
    pub r: f64,
    pub s: f64,
}
impl FractionalHex {
    #[allow(dead_code)]
    pub fn new_axial(q: f64, r: f64) -> FractionalHex {
        return FractionalHex { q, r, s: -q - r };
    }
    #[allow(dead_code)]
    pub fn new_cube(q: f64, r: f64, s: f64) -> FractionalHex {
        return FractionalHex { q, r, s };
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Orientation {
    pub f0: f64,
    pub f1: f64,
    pub f2: f64,
    pub f3: f64,
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
    pub start_angle: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Point,
    pub origin: Point,
}

#[allow(dead_code)]
pub fn hex_add(a: Hex, b: Hex) -> Hex {
    return Hex {
        q: a.q + b.q,
        r: a.r + b.r,
        s: a.s + b.s,
    };
}

#[allow(dead_code)]
pub fn hex_subtract(a: Hex, b: Hex) -> Hex {
    return Hex {
        q: a.q - b.q,
        r: a.r - b.r,
        s: a.s - b.s,
    };
}

#[allow(dead_code)]
pub fn hex_scale(a: Hex, k: i32) -> Hex {
    return Hex {
        q: a.q * k,
        r: a.r * k,
        s: a.s * k,
    };
}

#[allow(dead_code)]
pub fn hex_rotate_left(a: Hex) -> Hex {
    return Hex {
        q: -a.s,
        r: -a.q,
        s: -a.r,
    };
}

#[allow(dead_code)]
pub fn hex_rotate_right(a: Hex) -> Hex {
    return Hex {
        q: -a.r,
        r: -a.s,
        s: -a.q,
    };
}

static HEX_DIRECTIONS: [Hex; 6] = [
    Hex { q: 1, r: 0, s: -1 }, // east
    Hex { q: 1, r: -1, s: 0 }, // northeast
    Hex { q: 0, r: -1, s: 1 }, // northwest
    Hex { q: -1, r: 0, s: 1 }, // west
    Hex { q: -1, r: 1, s: 0 }, // southwest
    Hex { q: 0, r: 1, s: -1 }, // southeast
];

pub fn hex_direction(direction: i32) -> Hex {
    return HEX_DIRECTIONS[direction as usize];
}

pub fn hex_neighbor(hex: Hex, direction: i32) -> Hex {
    return hex_add(hex, hex_direction(direction));
}

pub fn all_hex_neighbors(hex: Hex) -> [Hex; 6] {
    let hex0 = hex_neighbor(hex, 0);
    let hex1 = hex_neighbor(hex, 1);
    let hex2 = hex_neighbor(hex, 2);
    let hex3 = hex_neighbor(hex, 3);
    let hex4 = hex_neighbor(hex, 4);
    let hex5 = hex_neighbor(hex, 5);
    let result = [ hex0, hex1, hex2, hex3, hex4, hex5 ];

    return result;
}

#[allow(dead_code)]
static HEX_DIAGONALS: [Hex; 6] = [
    Hex { q: 2, r: -1, s: -1 },
    Hex { q: 1, r: -2, s: 1 },
    Hex { q: -1, r: -1, s: 2 },
    Hex { q: -2, r: 1, s: 1 },
    Hex { q: -1, r: 2, s: -1 },
    Hex { q: 1, r: 1, s: -2 },
];

#[allow(dead_code)]
pub fn hex_diagonal_neighbor(hex: Hex, direction: i32) -> Hex {
    return hex_add(hex, HEX_DIAGONALS[direction as usize]);
}

#[allow(dead_code)]
pub fn hex_length(hex: Hex) -> i32 {
    return (hex.q.abs() + hex.r.abs() + hex.s.abs()) / 2 as i32;
}

#[allow(dead_code)]
pub fn hex_distance(a: Hex, b: Hex) -> i32 {
    return hex_length(hex_subtract(a, b));
}

pub fn hex_round(h: FractionalHex) -> Hex {
    let mut qi: i32 = h.q.round() as i32;
    let mut ri: i32 = h.r.round() as i32;
    let mut si: i32 = h.s.round() as i32;
    let q_diff: f64 = (qi as f64 - h.q).abs();
    let r_diff: f64 = (ri as f64 - h.r).abs();
    let s_diff: f64 = (si as f64 - h.s).abs();
    if q_diff > r_diff && q_diff > s_diff {
        qi = -ri - si;
    } else if r_diff > s_diff {
        ri = -qi - si;
    } else {
        si = -qi - ri;
    }

    return Hex {
        q: qi,
        r: ri,
        s: si,
    };
}

#[allow(dead_code)]
pub fn hex_lerp(a: FractionalHex, b: FractionalHex, t: f64) -> FractionalHex {
    return FractionalHex {
        q: a.q * (1.0 - t) + b.q * t,
        r: a.r * (1.0 - t) + b.r * t,
        s: a.s * (1.0 - t) + b.s * t,
    };
}

#[allow(dead_code)]
pub fn hex_linedraw(a: Hex, b: Hex) -> Vec<Hex> {
    let distance: i32 = hex_distance(a, b);
    let a_nudge: FractionalHex = FractionalHex {
        q: a.q as f64 + 1e-06,
        r: a.r as f64 + 1e-06,
        s: a.s as f64 - 2e-06,
    };
    let b_nudge: FractionalHex = FractionalHex {
        q: b.q as f64 + 1e-06,
        r: b.r as f64 + 1e-06,
        s: b.s as f64 - 2e-06,
    };
    let mut results: Vec<Hex> = vec![];
    let step: f64 = 1.0 / max(distance, 1) as f64;
    for i in 0..=distance {
        results.push(hex_round(hex_lerp(a_nudge, b_nudge, step * i as f64)));
    }

    return results;
}

pub const LAYOUT_POINTY: Orientation = Orientation {
    f0: SQRT_3,
    f1: SQRT_3 / 2.0,
    f2: 0.0,
    f3: 3.0 / 2.0,
    b0: SQRT_3 / 3.0,
    b1: -1.0 / 3.0,
    b2: 0.0,
    b3: 2.0 / 3.0,
    start_angle: 0.5,
};

#[allow(dead_code)]
pub const LAYOUT_FLAT: Orientation = Orientation {
    f0: 3.0 / 2.0,
    f1: 0.0,
    f2: SQRT_3 / 2.0,
    f3: SQRT_3,
    b0: 2.0 / 3.0,
    b1: 0.0,
    b2: -1.0 / 3.0,
    b3: SQRT_3 / 3.0,
    start_angle: 0.0,
};

pub fn hex_to_pixel_pointy_layout(h: Hex, layout_size: (f32, f32), scale: (f32, f32)) -> Point {
    let layout = Layout { orientation: LAYOUT_POINTY, size: Point::new(layout_size.0 as f64 * scale.0 as f64, -layout_size.1 as f64 * scale.1 as f64), origin: Point::new(0.0, 0.0) };
    let result = hex_to_pixel(layout, h);

    return result;
}

#[allow(dead_code)]
pub fn hex_to_pixel_flat_layout(h: Hex, layout_size: (f32, f32), scale: (f32, f32)) -> Point {
    let layout = Layout { orientation: LAYOUT_FLAT, size: Point::new(layout_size.0 as f64 * scale.0 as f64, -layout_size.1 as f64 * scale.1 as f64), origin: Point::new(0.0, 0.0) };
    let result = hex_to_pixel(layout, h);

    return result;
}

fn hex_to_pixel(layout: Layout, h: Hex) -> Point {
    let matrix: Orientation = layout.orientation;
    let size: Point = layout.size;
    let origin: Point = layout.origin;
    let x: f64 = (matrix.f0 * h.q as f64 + matrix.f1 * h.r as f64) * size.x;
    let y: f64 = (matrix.f2 * h.q as f64 + matrix.f3 * h.r as f64) * size.y;

    return Point {
        x: x + origin.x,
        y: y + origin.y,
    };
}

pub fn pixel_to_hex_pointy_layout(p: Point, layout_size: (f32, f32), scale: (f32, f32)) -> FractionalHex {
    let layout = Layout { orientation: LAYOUT_POINTY, size: Point::new(layout_size.0 as f64 * scale.0 as f64, -layout_size.1 as f64 * scale.1 as f64), origin: Point::new(0.0, 0.0) };
    let result = pixel_to_hex(layout, p);

    return result;
}

#[allow(dead_code)]
pub fn pixel_to_hex_flat_layout(p: Point, layout_size: (f32, f32), scale: (f32, f32)) -> FractionalHex {
    let layout = Layout { orientation: LAYOUT_FLAT, size: Point::new(layout_size.0 as f64 * scale.0 as f64, -layout_size.1 as f64 * scale.1 as f64), origin: Point::new(0.0, 0.0) };
    let result = pixel_to_hex(layout, p);

    return result;
}

pub fn pixel_to_hex(layout: Layout, p: Point) -> FractionalHex {
    let matrix: Orientation = layout.orientation;
    let size: Point = layout.size;
    let origin: Point = layout.origin;
    let pt: Point = Point {
        x: (p.x - origin.x) / size.x,
        y: (p.y - origin.y) / size.y,
    };
    let q: f64 = matrix.b0 * pt.x + matrix.b1 * pt.y;
    let r: f64 = matrix.b2 * pt.x + matrix.b3 * pt.y;

    return FractionalHex {
        q: q,
        r: r,
        s: -q - r,
    };
}
