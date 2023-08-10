/// The `ProjectivePlanePrimitive` trait defines the behavior of points and lines in a projective plane.
/// It requires two associated types: `Dual`, which represents the dual object (line or point) in the
/// projective plane, and `Self`, which represents the object implementing the trait.
pub trait ProjectivePlanePrimitive<Dual>: Eq {
    fn interact(&self, rhs: &Self) -> Dual; // join or meet
    fn incident(&self, dual: &Dual) -> bool; // incidence
}

/// The function `check_axiom` checks if certain axioms hold for points and lines in a projective plane.
///
/// Arguments:
///
/// * `pt_p`: A point in the projective plane.
/// * `pt_q`: The parameter `pt_q` represents a point in a projective plane.
/// * `ln_l`: The parameter `ln_l` represents a line in a projective plane.
pub fn check_axiom<Point, Line>(pt_p: &Point, pt_q: &Point, ln_l: &Line)
where
    Point: ProjectivePlanePrimitive<Line> + std::fmt::Debug,
    Line: ProjectivePlanePrimitive<Point> + std::fmt::Debug,
{
    // assert_eq!(pt_p, pt_p);
    // assert_eq!(pt_p == pt_q, pt_q == pt_p);
    assert_eq!(pt_p.incident(ln_l), ln_l.incident(pt_p));
    assert_eq!(pt_p.interact(pt_q), pt_q.interact(pt_p));
    let ln_m = pt_p.interact(pt_q);
    assert!(ln_m.incident(pt_p) && ln_m.incident(pt_q));
}

/// The `coincident` function checks if three points `pt_p`, `pt_q`, and `pt_r` are coincident in a projective
/// plane.
///
/// Arguments:
///
/// * `pt_p`: A point on the projection plane.
/// * `pt_q`: The parameter `pt_q` is of type `&Point`, which means it is a reference to an object of type `Point`.
/// * `pt_r`: The parameter `pt_r` is of type `&Point`, which means it is a reference to an object of type `Point`.
///
/// Returns:
///
/// The function `coincident` returns a boolean value.
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_plane::coincident;
/// use projgeom_rs::pg_object::PgPoint;
/// let pt_p = PgPoint::new([1, 2, 3]);
/// let pt_q = PgPoint::new([4, 5, 6]);
/// let pt_r = PgPoint::new([7, 8, 9]);
/// assert!(coincident(&pt_p, &pt_q, &pt_r));
/// ```
#[inline]
pub fn coincident<Point, Line>(pt_p: &Point, pt_q: &Point, pt_r: &Point) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    pt_p.interact(pt_q).incident(pt_r)
}

/// The function `check_pappus` checks if three points on a projective plane and three lines on another
/// projective plane satisfy Pappus' theorem.
///
/// Arguments:
///
/// * `coline_1`: The parameter `coline_1` is an array of length 3 containing elements of type `Point`.
/// * `coline_2`: The `coline_2` parameter is an array of three points `[pt_d, pt_e, pt_f]` in a projective plane. Each
/// point is represented by a type `Point` that implements the `ProjectivePlanePrimitive<Line>` trait, and `Line` is a type
/// that implements the `ProjectivePlane
///
/// Returns:
///
/// The function `check_pappus` returns a boolean value.
#[allow(dead_code)]
pub fn check_pappus<Point, Line>(coline_1: &[Point; 3], coline_2: &[Point; 3]) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let [pt_a, pt_b, pt_c] = coline_1;
    let [pt_d, pt_e, pt_f] = coline_2;
    let pt_g = (pt_a.interact(pt_e)).interact(&pt_b.interact(pt_d));
    let pt_h = (pt_a.interact(pt_f)).interact(&pt_c.interact(pt_d));
    let pt_i = (pt_b.interact(pt_f)).interact(&pt_c.interact(pt_e));
    coincident(&pt_g, &pt_h, &pt_i)
}

/// The `tri_dual` function takes a triangle and returns an array of lines that are dual to the
/// triangle's vertices.
///
/// Arguments:
///
/// * `triangle`: The `triangle` parameter is an array of length 3 containing points that define a
/// triangle in a projective plane. Each element of the array represents a vertex of the triangle.
///
/// Returns:
///
/// The function `tri_dual` returns an array of three elements, where each element is of type `Line`.
#[inline]
pub fn tri_dual<Point, Line>(triangle: &[Point; 3]) -> [Line; 3]
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    [a_2.interact(a_3), a_1.interact(a_3), a_1.interact(a_2)]
}

/// The function `persp` determines whether two triangles are perspective.
///
/// Arguments:
///
/// * `tri1`: An array of three points representing the vertices of the first triangle.
/// * `tri2`: tri2 is an array of three points representing the vertices of the second triangle.
///
/// Returns:
///
/// a boolean value, indicating whether two triangles are perspective or not.
#[inline]
pub fn persp<Point, Line>(tri1: &[Point; 3], tri2: &[Point; 3]) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let [pt_a, pt_b, pt_c] = tri1;
    let [pt_d, pt_e, pt_f] = tri2;
    let pt_o = pt_a.interact(pt_d).interact(&pt_b.interact(pt_e));
    pt_c.interact(pt_f).incident(&pt_o)
}

/// The function `check_desargue` checks if two triangles satisfy the Desargue's theorem in projective
/// geometry.
///
/// Arguments:
///
/// * `tri1`: An array of 3 points representing the first triangle in the projective plane.
/// * `tri2`: tri2 is an array of 3 points representing the vertices of a triangle in a projective
/// plane.
///
/// Returns:
///
/// The function `check_desargue` returns a boolean value.
#[allow(dead_code)]
pub fn check_desargue<Point, Line>(tri1: &[Point; 3], tri2: &[Point; 3]) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let trid1 = &tri_dual(tri1);
    let trid2 = &tri_dual(tri2);
    let b1 = persp(tri1, tri2);
    let b2 = persp(trid1, trid2);
    (b1 && b2) || (!b1 && !b2)
}

pub trait ProjectivePlane<Dual, Value: Default + Eq>: ProjectivePlanePrimitive<Dual> {
    fn aux(&self) -> Dual; // Dual not incident with Self
    fn dot(&self, dual: &Dual) -> Value; // for basic measurement
    fn plucker(&self, lambda: Value, other: &Self, mu: Value) -> Self;
}

/// The function `check_axiom2` checks if certain axioms hold true in a projective plane.
///
/// Arguments:
///
/// * `pt_p`: A point in the projective plane.
/// * `pt_q`: The parameter `pt_q` represents a point in a projective plane.
/// * `ln_l`: The parameter `ln_l` represents a line in a projective plane.
/// * `pt_a`: The parameter `pt_a` represents a point in the projective plane.
/// * `pt_b`: The parameter `pt_b` represents a value of type `Value` which is used as an argument in
/// the function `check_axiom2`. The specific meaning or purpose of `pt_b` would depend on the
/// implementation details of the `ProjectivePlane` trait and its associated types `Point`
#[allow(dead_code)]
pub fn check_axiom2<Point, Line, Value>(
    pt_p: &Point,
    pt_q: &Point,
    ln_l: &Line,
    pt_a: Value,
    pt_b: Value,
) where
    Value: Default + Eq,
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
{
    assert!(pt_p.dot(ln_l) == ln_l.dot(pt_p));
    assert!(!pt_p.aux().incident(pt_p));
    let ln_m = pt_p.interact(pt_q);
    assert!(ln_m.incident(&Point::plucker(pt_p, pt_a, pt_q, pt_b)));
}

/// The `harm_conj` function calculates the harmonic conjugate of three points in a projective plane.
///
/// Arguments:
///
/// * `pt_a`: The parameter `pt_a` represents a point on a projective plane.
/// * `pt_b`: The parameter `pt_b` represents a point in a projective plane.
/// * `pt_c`: The parameter `pt_c` represents a point in a projective plane.
///
/// Returns:
///
/// The function `harm_conj` returns a value of type `Point`, which is the harmonic conjugate of the points
/// `pt_a`, `pt_b`, and `pt_c`.
#[inline]
pub fn harm_conj<Point, Line, Value>(pt_a: &Point, pt_b: &Point, pt_c: &Point) -> Point
where
    Value: Default + Eq,
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
{
    assert!(coincident(pt_a, pt_b, pt_c));
    let ln_ab = pt_a.interact(pt_b);
    let ln_xc = ln_ab.aux().interact(pt_c);
    Point::plucker(pt_a, ln_xc.dot(pt_b), pt_b, ln_xc.dot(pt_a))
}

/// The function `involution` performs an involution transformation on a point `pt_p` with respect to an
/// origin point `origin` and a mirror line `mirror`.
///
/// Arguments:
///
/// * `origin`: The `origin` parameter represents the origin point in the projective plane.
/// * `mirror`: The `mirror` parameter represents a mirror line or mirror plane in a projective
/// geometry. It is used to perform an involution transformation on a point `pt_p` with respect to an
/// origin point `origin`.
/// * `pt_p`: The parameter `pt_p` represents a point in the projective plane.
///
/// Returns:
///
/// The function `involution` returns a value of type `Point`, which is the projected point on the
/// projection plane.
#[allow(dead_code)]
#[inline]
pub fn involution<Point, Line, Value>(origin: &Point, mirror: &Line, pt_p: &Point) -> Point
where
    Value: Default + Eq,
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
{
    let ln_po = pt_p.interact(origin);
    let pt_b = ln_po.interact(mirror);
    harm_conj(origin, &pt_b, pt_p)
}

#[cfg(test)]
mod tests {
    use crate::pg_plane::ProjectivePlanePrimitive;
    use crate::pg_plane::{check_axiom, coincident};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct PArch {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct LArch {}

    impl PArch {
        #[inline]
        fn new() -> Self {
            Self {}
        }
    }

    impl LArch {
        #[inline]
        fn new() -> Self {
            Self {}
        }
    }

    impl ProjectivePlanePrimitive<LArch> for PArch {
        #[inline]
        fn incident(&self, _rhs: &LArch) -> bool {
            true
        }

        #[inline]
        fn interact(&self, _rhs: &Self) -> LArch {
            LArch::new()
        }
    }

    impl ProjectivePlanePrimitive<PArch> for LArch {
        #[inline]
        fn incident(&self, _rhs: &PArch) -> bool {
            true
        }

        #[inline]
        fn interact(&self, _rhs: &Self) -> PArch {
            PArch::new()
        }
    }

    #[test]
    fn it_works() {
        let pt_p = PArch::new();
        let pt_q = PArch::new();
        let pt_r = PArch::new();
        let ln_l = LArch::new();
        println!("{}", pt_p == pt_q);
        println!("{}", pt_p.incident(&ln_l));
        println!("{}", coincident(&pt_p, &pt_q, &pt_r));
        check_axiom(&pt_p, &pt_q, &ln_l);
    }
}
