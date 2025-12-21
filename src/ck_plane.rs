use crate::pg_plane::{coincident, involution, tri_dual};
use crate::pg_plane::{ProjectivePlane, ProjectivePlanePrimitive};

/// The `CayleyKleinPlanePrimitive` trait is a trait that extends the `ProjectivePlanePrimitive` trait. It adds an additional
/// method `perp(&self) -> Line` to the trait. This method returns the polar line to the given
/// point or the pole of a given line.
///
/// # Examples
///
/// ```text
/// // This example would require concrete implementations of Point and Line
/// // For instance, if you have `EuclidPoint` and `EuclidLine` that implement
/// // `CayleyKleinPlanePrimitive`:
/// // use projgeom_rs::{EuclidPoint, EuclidLine, CayleyKleinPlanePrimitive};
/// //
/// // let p = EuclidPoint::new(1, 2, 1);
/// // let l = p.perp(); // Get the polar line of point p
/// //
/// // let m = EuclidLine::new(1, 2, 3);
/// // let q = m.perp(); // Get the pole of line m
/// ```
pub trait CayleyKleinPlanePrimitive<Dual>: ProjectivePlanePrimitive<Dual> {
    // type Dual: ProjectivePlanePrimitive;
    fn perp(&self) -> Dual; // pole or polar
}

/// The function `is_perpendicular` checks if two lines are perpendicular to each other.
///
/// Arguments:
///
/// * `m_1`: A reference to an object of type Line, which represents a line or a plane in a geometric space.
/// * `m_2`: m_2 is a reference to an object of type Line, which is a generic type parameter. The specific
///   type of Line is not specified in the function signature.
///
/// Returns:
///
/// a boolean value.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, EuclidLine, is_perpendicular};
///
/// let l1 = EuclidLine::new([1, 0, -1]); // x = 1
/// let l2 = EuclidLine::new([0, 1, -1]); // y = 1
/// assert!(is_perpendicular(&l1, &l2));
///
/// let l3 = EuclidLine::new([1, 1, -1]); // x + y = 1
/// assert!(!is_perpendicular(&l1, &l3));
/// ```
#[allow(dead_code)]
#[inline]
pub fn is_perpendicular<Point, Line>(line_1: &Line, line_2: &Line) -> bool
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    line_1.perp().incident(line_2)
}

/// The `altitude` function calculates the altitude of a point `pt_p` with respect to a line `ln_m`.
///
/// Arguments:
///
/// * `pt_p`: A point on a plane.
/// * `ln_m`: The parameter `ln_m` represents a line in a two-dimensional plane.
///
/// Returns:
///
/// The function `altitude` returns a value of type `Line`.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, EuclidLine, altitude};
///
/// let p = EuclidPoint::new([1, 2, 1]);
/// let l = EuclidLine::new([1, 0, -1]); // x = 1
/// let alt = altitude(&p, &l);
/// // In Euclidean geometry, the altitude from a point to a line is perpendicular to the line
/// // and passes through the point. The line x=1 has normal (1,0). The line through (1,2) perpendicular
/// // to x=1 is y=2. So the altitude should be (0,1,-2).
/// assert_eq!(alt, EuclidLine::new([0, 1, -2]));
/// ```
#[allow(dead_code)]
#[inline]
pub fn altitude<Point, Line>(point_p: &Point, line_m: &Line) -> Line
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    line_m.perp().meet(point_p)
}

/// The `orthocenter` function calculates the orthocenter of a triangle given its three vertices.
///
/// Arguments:
///
/// * `triangle`: The `triangle` parameter is an array of three elements of type `Point`. Each element represents a
///   point in a triangle.
///
/// Returns:
///
/// The function `orthocenter` returns a value of type `Point`, which is the type parameter specified in the
/// function signature.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, orthocenter};
///
/// let p1 = EuclidPoint::new([0, 0, 1]);
/// let p2 = EuclidPoint::new([2, 0, 1]);
/// let p3 = EuclidPoint::new([1, 3, 1]);
/// let triangle = [p1, p2, p3];
/// let orthocenter_pt = orthocenter(&triangle);
/// // For a triangle with vertices (0,0), (2,0), (1,3), the orthocenter is (1, 1/3)
/// // In homogeneous coordinates, this would be (3, 1, 3)
/// assert_eq!(orthocenter_pt, EuclidPoint::new([3, 1, 3]));
/// ```
#[allow(dead_code)]
#[inline]
pub fn orthocenter<Point, Line>(triangle: &[Point; 3]) -> Point
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    let t_1 = altitude(a_1, &a_2.meet(a_3));
    let t_2 = altitude(a_2, &a_3.meet(a_1));
    t_1.meet(&t_2)
}

/// The function `tri_altitude` calculates the altitudes of a triangle given its three vertices and
/// three lines.
///
/// Arguments:
///
/// * `triangle`: triangle is an array of 3 elements of type Point, representing the vertices of a triangle.
///
/// Returns:
///
/// The function `tri_altitude` returns an array of three elements, where each element represents the
/// altitude of a vertex in a triangle.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, EuclidLine, tri_altitude};
///
/// let p1 = EuclidPoint::new([0, 0, 1]);
/// let p2 = EuclidPoint::new([2, 0, 1]);
/// let p3 = EuclidPoint::new([1, 3, 1]);
/// let triangle = [p1, p2, p3];
/// let altitudes = tri_altitude(&triangle);
/// // For p1(0,0), opposite side is line p2p3 (EuclidLine::new(3, -1, -6)). Perpendicular through (0,0) is (1,3,0).
/// assert_eq!(altitudes[0], EuclidLine::new([-1, 3, 0]));
/// // For p2(2,0), opposite side is line p1p3 (EuclidLine::new(3, -1, 0)). Perpendicular through (2,0) is (1,3,-2).
/// assert_eq!(altitudes[1], EuclidLine::new([1, 3, -2]));
/// // For p3(1,3), opposite side is line p1p2 (EuclidLine::new(0, 1, 0)). Perpendicular through (1,3) is (1,0,-1).
/// assert_eq!(altitudes[2], EuclidLine::new([2, 0, -2]));
/// ```
#[allow(dead_code)]
#[inline]
pub fn tri_altitude<Point, Line>(triangle: &[Point; 3]) -> [Line; 3]
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    let [l_1, l_2, l_3] = tri_dual(triangle);
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    let t_1 = altitude(a_1, &l_1);
    let t_2 = altitude(a_2, &l_2);
    let t_3 = altitude(a_3, &l_3);
    [t_1, t_2, t_3]
}

pub trait CayleyKleinPlane<Dual, Value: Default + Eq>:
    ProjectivePlane<Dual, Value> + CayleyKleinPlanePrimitive<Dual>
{
}

/// The `reflect` function in Rust reflects a point `pt_p` across a mirror plane `mirror`.
///
/// Arguments:
///
/// * `mirror`: A reference to an object of type Line, which represents a mirror or a plane.
/// * `pt_p`: The parameter `pt_p` represents a point in a geometric space.
///
/// Returns:
///
/// The function `reflect` returns a value of type `Point`, which is the same type as the input parameter
/// `pt_p`.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, EuclidLine, reflect};
///
/// let p = EuclidPoint::new([1, 2, 1]); // Point (1,2)
/// let mirror = EuclidLine::new([1, 0, 0]); // Line x = 0 (y-axis)
/// let reflected_p = reflect(&mirror, &p);
/// // Reflecting (1,2) across x=0 should give (-1,2)
/// assert_eq!(reflected_p, EuclidPoint::new([-1, 2, 1]));
///
/// let p2 = EuclidPoint::new([3, 3, 1]); // Point (3,3)
/// let mirror2 = EuclidLine::new([0, 1, -2]); // Line y = 2
/// let reflected_p2 = reflect(&mirror2, &p2);
/// // Reflecting (3,3) across y=2 should give (3,1)
/// assert_eq!(reflected_p2, EuclidPoint::new([3, 1, 1]));
/// ```
#[allow(dead_code)]
#[inline]
pub fn reflect<Point, Line, Value>(mirror: &Line, point_p: &Point) -> Point
where
    Value: Default + Eq,
    Point: CayleyKleinPlane<Line, Value>,
    Line: CayleyKleinPlane<Point, Value>,
{
    involution(&mirror.perp(), mirror, point_p)
}

/// Macro to implement the `CayleyKleinPlanePrimitive` and `CayleyKleinPlane` traits.
#[macro_export]
macro_rules! impl_cayley_klein_plane {
    ($point:ty, $line:ty, $perp_point:expr, $perp_line:expr) => {
        impl CayleyKleinPlanePrimitive<$line> for $point {
            #[inline]
            fn perp(&self) -> $line {
                $perp_point(self)
            }
        }

        impl CayleyKleinPlanePrimitive<$point> for $line {
            #[inline]
            fn perp(&self) -> $point {
                $perp_line(self)
            }
        }

        impl CayleyKleinPlane<$line, i64> for $point {}

        impl CayleyKleinPlane<$point, i64> for $line {}
    };
}
