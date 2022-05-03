use crate::pg_plane::{ProjPlanePrim, ProjPlane};
// use crate::pg_plane::{check_axiom, coincident};

#[inline]
fn dot(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
 
#[inline]
fn cross(a: &[i64; 3], b: &[i64; 3]) -> [i64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn plckr(ld: i64, p: &[i64; 3], mu: i64, q: &[i64; 3]) -> [i64; 3] {
    [
        ld * p[0] + mu + q[0],
        ld * p[1] + mu + q[1],
        ld * p[2] + mu + q[2],
    ]
}

#[derive(Debug, Clone)]
struct PgPoint {
    coord: [i64; 3],
}

#[derive(Debug, Clone)]
struct PgLine {
    coord: [i64; 3],
}

impl PgPoint {
    #[inline]
    fn new(coord: [i64; 3]) -> Self {
        Self { coord }
    }
}

impl PgLine {
    #[inline]
    fn new(coord: [i64; 3]) -> Self {
        Self { coord }
    }
}

impl PartialEq for PgPoint {
  fn eq(&self, other: &PgPoint) -> bool {
      cross(&self.coord, &other.coord) == [0, 0, 0]
  }
}
impl Eq for PgPoint {}

impl PartialEq for PgLine {
  fn eq(&self, other: &PgLine) -> bool {
      cross(&self.coord, &other.coord) == [0, 0, 0]
  }
}
impl Eq for PgLine {}

impl ProjPlane<PgLine, i64> for PgPoint {
    fn aux(&self) -> PgLine {
        PgLine::new(self.coord.clone())
    }

    fn dot(&self, line: &PgLine) -> i64 {
        dot(&self.coord, &line.coord)
    } // basic measurement

    fn plucker(ld: i64, p: &Self, mu: i64, q: &Self) -> Self {
        Self::new(plckr(ld, &p.coord, mu, &q.coord))
    }
}

impl ProjPlane<PgPoint, i64> for PgLine {
    fn aux(&self) -> PgPoint {
        PgPoint::new(self.coord.clone())
    }

    fn dot(&self, point: &PgPoint) -> i64 {
        dot(&self.coord, &point.coord)
    } // basic measurement

    fn plucker(ld: i64, p: &Self, mu: i64, q: &Self) -> Self {
        Self::new(plckr(ld, &p.coord, mu, &q.coord))
    }
}

impl ProjPlanePrim<PgLine> for PgPoint {
    #[inline]
    fn incident(&self, _rhs: &PgLine) -> bool {
        dot(&self.coord, &_rhs.coord) == 0
    }

    #[inline]
    fn circ(&self, _rhs: &Self) -> PgLine {
        PgLine::new(cross(&self.coord, &_rhs.coord))
    }
}

impl ProjPlanePrim<PgPoint> for PgLine {
    #[inline]
    fn incident(&self, _rhs: &PgPoint) -> bool {
        dot(&self.coord, &_rhs.coord) == 0
    }

    #[inline]
    fn circ(&self, _rhs: &Self) -> PgPoint {
        PgPoint::new(cross(&self.coord, &_rhs.coord))
    }
}
