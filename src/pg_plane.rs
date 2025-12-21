/// The `ProjectivePlanePrimitive` trait defines the behavior of points and lines in a projective plane.
/// It requires two associated types: `Dual`, which represents the dual object (line or point) in the
/// projective plane, and `Self`, which represents the object implementing the trait.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, PgLine, ProjectivePlanePrimitive};
///
/// let p1 = PgPoint::new([1, 2, 3]);
/// let p2 = PgPoint::new([4, 5, 6]);
/// let l1 = p1.meet(&p2); // The line joining p1 and p2
///
/// assert!(l1.incident(&p1));
/// assert!(l1.incident(&p2));
///
/// let l2 = PgLine::new([1, 1, -1]); // x + y - z = 0
/// let p3 = PgPoint::new([1, 0, 1]); // (1,0,1) is on x + y - z = 0
/// assert!(p3.incident(&l2));
/// ```
pub trait ProjectivePlanePrimitive<Dual>: Eq {
    fn meet(&self, rhs: &Self) -> Dual; // join or meet
    fn incident(&self, dual: &Dual) -> bool; // incidence
}

/// The function `check_axiom` checks if certain axioms hold for points and lines in a projective plane.
///
/// Arguments:
///
/// * `pt_p`: A point in the projective plane.
/// * `pt_q`: The parameter `pt_q` represents a point in a projective plane.
/// * `ln_l`: The parameter `ln_l` represents a line in a projective plane.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, PgLine, check_axiom};
///
/// let p1 = PgPoint::new([1, 2, 3]);
/// let p2 = PgPoint::new([4, 5, 6]);
/// let l1 = PgLine::new([1, 1, -1]);
/// check_axiom(&p1, &p2, &l1);
/// ```
pub fn check_axiom<Point, Line>(pt_p: &Point, pt_q: &Point, ln_l: &Line)
where
    Point: ProjectivePlanePrimitive<Line> + std::fmt::Debug,
    Line: ProjectivePlanePrimitive<Point> + std::fmt::Debug,
{
    // assert_eq!(pt_p, pt_p);
    // assert_eq!(pt_p == pt_q, pt_q == pt_p);
    assert_eq!(pt_p.incident(ln_l), ln_l.incident(pt_p));
    assert_eq!(pt_p.meet(pt_q), pt_q.meet(pt_p));
    let ln_m = pt_p.meet(pt_q);
    assert!(ln_m.incident(pt_p) && ln_m.incident(pt_q));
}

#[doc = svgbobdoc::transform!(
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
/// ```svgbob
///
///          |  /
///        \ | /       coincidence
///         \|/
///          o      -----o------o---o---
///         /|\           A      B   C
///        / | \
///       l  |  \
///          m   n
/// ```
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
)]
#[inline]
pub fn coincident<Point, Line>(pt_p: &Point, pt_q: &Point, pt_r: &Point) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    pt_p.meet(pt_q).incident(pt_r)
}

/// The function `check_pappus` checks if three points on a projective plane and three lines on another
/// projective plane satisfy Pappus' theorem.
///
/// Arguments:
///
/// * `coline_1`: The parameter `coline_1` is an array of length 3 containing elements of type `Point`.
/// * `coline_2`: The `coline_2` parameter is an array of three points `[pt_d, pt_e, pt_f]` in a projective plane. Each
///   point is represented by a type `Point` that implements the `ProjectivePlanePrimitive<Line>` trait, and `Line` is a type
///   that implements the `ProjectivePlane
///
/// Returns:
///
/// The function `check_pappus` returns a boolean value.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, check_pappus};
///
/// let a = PgPoint::new([0, 0, 1]);
/// let b = PgPoint::new([1, 0, 1]);
/// let c = PgPoint::new([2, 0, 1]);
///
/// let d = PgPoint::new([0, 1, 1]);
/// let e = PgPoint::new([1, 1, 1]);
/// let f = PgPoint::new([2, 1, 1]);
///
/// let coline_1 = [a, b, c];
/// let coline_2 = [d, e, f];
///
/// // These points are collinear, so Pappus' theorem should hold
/// assert!(check_pappus(&coline_1, &coline_2));
/// ```
#[allow(dead_code)]
pub fn check_pappus<Point, Line>(coline_1: &[Point; 3], coline_2: &[Point; 3]) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let [pt_a, pt_b, pt_c] = coline_1;
    let [pt_d, pt_e, pt_f] = coline_2;
    let pt_g = (pt_a.meet(pt_e)).meet(&pt_b.meet(pt_d));
    let pt_h = (pt_a.meet(pt_f)).meet(&pt_c.meet(pt_d));
    let pt_i = (pt_b.meet(pt_f)).meet(&pt_c.meet(pt_e));
    coincident(&pt_g, &pt_h, &pt_i)
}

#[doc = svgbobdoc::transform!(
/// The `tri_dual` function takes a triangle and returns an array of lines that are dual to the
/// triangle's vertices.
///
/// Arguments:
///
/// * `triangle`: The `triangle` parameter is an array of length 3 containing points that define a
///               triangle in a projective plane. Each element of the array represents a vertex of the triangle.
///
/// ```svgbob
///                       a
///            \         /
///             \ A     /
///      c ------o-----o--------
///               \   / B
///                \ /
///               C o    triangle,
///                / \     trilateral
///               /   \
///                    b
/// ```
///
/// Returns:
///
/// The function `tri_dual` returns an array of three elements, where each element is of type `Line`.
)]
#[inline]
pub fn tri_dual<Point, Line>(triangle: &[Point; 3]) -> [Line; 3]
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    [a_2.meet(a_3), a_1.meet(a_3), a_1.meet(a_2)]
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
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, persp};
///
/// let t1_p1 = PgPoint::new([0, 0, 1]);
/// let t1_p2 = PgPoint::new([1, 0, 1]);
/// let t1_p3 = PgPoint::new([0, 1, 1]);
/// let tri1 = [t1_p1, t1_p2, t1_p3];
///
/// let t2_p1 = PgPoint::new([0, 0, 2]);
/// let t2_p2 = PgPoint::new([2, 0, 2]);
/// let t2_p3 = PgPoint::new([0, 2, 2]);
/// let tri2 = [t2_p1, t2_p2, t2_p3];
///
/// // These two triangles are perspective from the origin (0,0,1)
/// assert!(persp(&tri1, &tri2));
/// ```
#[inline]
pub fn persp<Point, Line>(tri1: &[Point; 3], tri2: &[Point; 3]) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let [pt_a, pt_b, pt_c] = tri1;
    let [pt_d, pt_e, pt_f] = tri2;
    let pt_o = pt_a.meet(pt_d).meet(&pt_b.meet(pt_e));
    pt_c.meet(pt_f).incident(&pt_o)
}

/// The function `check_desargue` checks if two triangles satisfy the Desargue's theorem in projective
/// geometry.
///
/// Arguments:
///
/// * `tri1`: An array of 3 points representing the first triangle in the projective plane.
/// * `tri2`: tri2 is an array of 3 points representing the vertices of a triangle in a projective plane.
///
/// Returns:
///
/// The function `check_desargue` returns a boolean value.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, check_desargue};
///
/// let t1_p1 = PgPoint::new([0, 0, 1]);
/// let t1_p2 = PgPoint::new([1, 0, 1]);
/// let t1_p3 = PgPoint::new([0, 1, 1]);
/// let tri1 = [t1_p1, t1_p2, t1_p3];
///
/// let t2_p1 = PgPoint::new([0, 0, 2]);
/// let t2_p2 = PgPoint::new([2, 0, 2]);
/// let t2_p3 = PgPoint::new([0, 2, 2]);
/// let tri2 = [t2_p1, t2_p2, t2_p3];
///
/// // These two triangles are perspective, so Desargue's theorem should hold
/// assert!(check_desargue(&tri1, &tri2));
/// ```
#[allow(dead_code)]
pub fn check_desargue<Point, Line>(tri1: &[Point; 3], tri2: &[Point; 3]) -> bool
where
    Point: ProjectivePlanePrimitive<Line>,
    Line: ProjectivePlanePrimitive<Point>,
{
    let trid1 = &tri_dual(tri1);
    let trid2 = &tri_dual(tri2);
    let bool1 = persp(tri1, tri2);
    let bool2 = persp(trid1, trid2);
    (bool1 && bool2) || (!bool1 && !bool2)
}

/// The `ProjectivePlane` trait is a trait that extends the `ProjectivePlanePrimitive` trait. It adds an additional
/// `aux`, `dot`, and parametrize methods to the trait.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, PgLine, ProjectivePlane, ProjectivePlanePrimitive};
///
/// let p = PgPoint::new([1, 2, 3]);
/// let l = PgLine::new([1, 1, -1]);
///
/// // aux returns a line not incident with p
/// let aux_l = p.aux();
/// assert!(!p.incident(&aux_l));
///
/// // dot product
/// let dot_val = p.dot(&l);
/// assert_eq!(dot_val, 1 * 1 + 2 * 1 + 3 * -1); // 1 + 2 - 3 = 0
/// assert_eq!(dot_val, 0); // p is incident to l
///
/// // parametrize
/// let p1 = PgPoint::new([1, 0, 1]);
/// let p2 = PgPoint::new([0, 1, 1]);
/// let p_mid = p1.parametrize(1, &p2, 1);
/// assert_eq!(p_mid, PgPoint::new([1, 1, 2]));
/// ```
pub trait ProjectivePlane<Dual, Value: Default + Eq>: ProjectivePlanePrimitive<Dual> {
    fn aux(&self) -> Dual; // Dual not incident with Self
    fn dot(&self, dual: &Dual) -> Value; // for basic measurement
    fn parametrize(&self, lambda_val: Value, other: &Self, mu_val: Value) -> Self;
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
///   the function `check_axiom2`. The specific meaning or purpose of `pt_b` would depend on the
///   implementation details of the `ProjectivePlane` trait and its associated types `Point`
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, PgLine, check_axiom2};
///
/// let p1 = PgPoint::new([1, 2, 3]);
/// let p2 = PgPoint::new([4, 5, 6]);
/// let l1 = PgLine::new([1, 1, -1]);
/// check_axiom2(&p1, &p2, &l1, 1, 1);
/// ```
#[allow(dead_code)]
pub fn check_axiom2<Point, Line, Value>(
    pt_p: &Point,
    pt_q: &Point,
    ln_l: &Line,
    alpha: Value,
    beta: Value,
) where
    Value: Default + Eq,
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
{
    assert!(pt_p.dot(ln_l) == ln_l.dot(pt_p));
    assert!(!pt_p.aux().incident(pt_p));
    let ln_m = pt_p.meet(pt_q);
    assert!(ln_m.incident(&pt_p.parametrize(alpha, pt_q, beta)));
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
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, harm_conj};
///
/// let p1 = PgPoint::new([1, 0, 1]); // A
/// let p2 = PgPoint::new([0, 0, 1]); // B
/// let p3 = PgPoint::new([2, 0, 1]); // C
///
/// // Harmonic conjugate of C with respect to A and B
/// let p4 = harm_conj(&p1, &p2, &p3);
/// assert_eq!(p4, PgPoint::new([2, 0, 3]));
/// ```
#[inline]
pub fn harm_conj<Point, Line, Value>(pt_a: &Point, pt_b: &Point, pt_c: &Point) -> Point
where
    Value: Default + Eq,
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
{
    assert!(coincident(pt_a, pt_b, pt_c));
    let ln_ab = pt_a.meet(pt_b);
    let ln_xc = ln_ab.aux().meet(pt_c);
    pt_a.parametrize(ln_xc.dot(pt_b), pt_b, ln_xc.dot(pt_a))
}

/// The function `involution` performs an involution transformation on a point `pt_p` with respect to an
/// origin point `origin` and a mirror line `mirror`.
///
/// Arguments:
///
/// * `origin`: The `origin` parameter represents the origin point in the projective plane.
/// * `mirror`: The `mirror` parameter represents a mirror line or mirror plane in a projective
///   geometry. It is used to perform an involution transformation on a point `pt_p` with respect to an
///   origin point `origin`.
/// * `pt_p`: The parameter `pt_p` represents a point in the projective plane.
///
/// Returns:
///
/// The function `involution` returns a value of type `Point`, which is the projected point on the
/// projection plane.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, PgLine, involution};
///
/// let origin = PgPoint::new([0, 0, 1]); // Origin at (0,0)
/// let mirror = PgLine::new([1, 0, -1]); // Mirror line x = 1
/// let p = PgPoint::new([2, 0, 1]); // Point (2,0)
///
/// // Reflect (2,0) across x=1 with origin (0,0)
/// // The line from origin (0,0) to p (2,0) is the x-axis (0,1,0).
/// // The intersection of x-axis and mirror x=1 is (1,0).
/// // Harmonic conjugate of p (2,0) with respect to origin (0,0) and (1,0) is (inf, 0).
/// // In homogeneous coordinates, this would be (1,0,0).
/// let reflected_p = involution(&origin, &mirror, &p);
/// assert_eq!(reflected_p, PgPoint::new([-8, 0, -12]));
/// ```
#[allow(dead_code)]
#[inline]
pub fn involution<Point, Line, Value>(origin: &Point, mirror: &Line, pt_p: &Point) -> Point
where
    Value: Default + Eq,
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
{
    let ln_po = pt_p.meet(origin);
    let pt_b = ln_po.meet(mirror);
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
        fn meet(&self, _rhs: &Self) -> LArch {
            LArch::new()
        }
    }

    impl ProjectivePlanePrimitive<PArch> for LArch {
        #[inline]
        fn incident(&self, _rhs: &PArch) -> bool {
            true
        }

        #[inline]
        fn meet(&self, _rhs: &Self) -> PArch {
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
