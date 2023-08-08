use crate::pg_plane::{coincident, involution, tri_dual};
use crate::pg_plane::{ProjectivePlane, ProjectivePlanePrimitive};

/// The `CayleyKleinPlanePrimitive` trait is a trait that extends the `ProjectivePlanePrimitive` trait. It adds an additional
/// method `perp(&self) -> Line` to the trait. This method returns the perpendicular line to the given line
/// `self`. The `Line` type parameter represents a line in the projective plane.
pub trait CayleyKleinPlanePrimitive<Line>: ProjectivePlanePrimitive<Line> {
    // type Dual: ProjectivePlanePrimitive;
    fn perp(&self) -> Line;
}

/// The function `is_perpendicular` checks if two lines are perpendicular to each other.
///
/// Arguments:
///
/// * `m_1`: A reference to an object of type Line, which represents a line or a plane in a geometric space.
/// * `m_2`: m_2 is a reference to an object of type Line, which is a generic type parameter. The specific
/// type of Line is not specified in the function signature.
///
/// Returns:
///
/// a boolean value.
#[allow(dead_code)]
#[inline]
pub fn is_perpendicular<Point, Line>(m_1: &Line, m_2: &Line) -> bool
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    m_1.perp().incident(m_2)
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
#[allow(dead_code)]
#[inline]
pub fn altitude<Point, Line>(pt_p: &Point, ln_m: &Line) -> Line
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    ln_m.perp().interact(pt_p)
}

/// The `orthocenter` function calculates the orthocenter of a triangle given its three vertices.
///
/// Arguments:
///
/// * `triangle`: The `triangle` parameter is an array of three elements of type `Point`. Each element represents a
/// point in a triangle.
///
/// Returns:
///
/// The function `orthocenter` returns a value of type `Point`, which is the type parameter specified in the
/// function signature.
#[allow(dead_code)]
#[inline]
pub fn orthocenter<Point, Line>(triangle: &[Point; 3]) -> Point
where
    Point: CayleyKleinPlanePrimitive<Line>,
    Line: CayleyKleinPlanePrimitive<Point>,
{
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    let t_1 = altitude(a_1, &a_2.interact(a_3));
    let t_2 = altitude(a_2, &a_3.interact(a_1));
    t_1.interact(&t_2)
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

pub trait CayleyKleinPlane<Line, Value: Default + Eq>:
    ProjectivePlane<Line, Value> + CayleyKleinPlanePrimitive<Line>
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
#[allow(dead_code)]
#[inline]
pub fn reflect<Point, Line, Value>(mirror: &Line, pt_p: &Point) -> Point
where
    Value: Default + Eq,
    Point: CayleyKleinPlane<Line, Value>,
    Line: CayleyKleinPlane<Point, Value>,
{
    involution(&mirror.perp(), mirror, pt_p)
}
