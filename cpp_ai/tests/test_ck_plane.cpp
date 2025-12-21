#include <doctest/doctest.h>
#include "projgeom.hpp"

template<typename Point, typename Line>
static void check_ck_plane(const Point& a_1, const Point& a_2, const Point& a_3) {
    const std::array triangle = {a_1, a_2, a_3};
    const auto trilateral = projgeom::tri_dual<Point, Line>(triangle);
    const auto& l_1 = trilateral[0];
    
    CHECK(l_1.incident(triangle[1]));
    
    const auto [t_1, t_2, t_3] = projgeom::tri_altitude<Point, Line>(triangle);
    CHECK(projgeom::is_perpendicular(t_1, l_1));
    
    const auto pt_o = projgeom::orthocenter<Point, Line>(triangle);
    CHECK(pt_o == t_2.meet(t_3));
}

TEST_SUITE("ck_plane") {
    TEST_CASE("tri_dual") {
        SUBCASE("Dual of triangle points") {
            projgeom::PgPoint p1({1, 0, 1});
            projgeom::PgPoint p2({0, 1, 1});
            projgeom::PgPoint p3({1, 1, 1});
            
            const std::array triangle = {p1, p2, p3};
            const auto trilateral = projgeom::tri_dual(triangle);
            
            // trilateral should be lines through opposite sides
            CHECK(trilateral[0] == p2.meet(p3));
            CHECK(trilateral[1] == p3.meet(p1));
            CHECK(trilateral[2] == p1.meet(p2));
        }
        
        SUBCASE("Dual of trilateral lines") {
            projgeom::PgLine l1({1, 0, 1});
            projgeom::PgLine l2({0, 1, 1});
            projgeom::PgLine l3({1, 1, 1});
            
            const std::array trilateral = {l1, l2, l3};
            const auto triangle = projgeom::tri_dual(trilateral);
            
            // triangle should be intersection points
            CHECK(triangle[0] == l2.meet(l3));
            CHECK(triangle[1] == l3.meet(l1));
            CHECK(triangle[2] == l1.meet(l2));
        }
    }
    
    TEST_CASE("Elliptic geometry") {
        SUBCASE("Elliptic point perpendicular check") {
            projgeom::EllipticPoint<int64_t> ep(projgeom::PgPoint<int64_t>({1, 2, 3}));
            projgeom::PgLine<int64_t> l({1, 2, 3});
            
            // In elliptic geometry, perpendicular means dot product = 0
            // Here dot = 1*1 + 2*2 + 3*3 = 14 ≠ 0
            CHECK(!ep.is_perpendicular(l));
            
            projgeom::PgLine<int64_t> perpendicular_l({1, 2, -3});
            // dot = 1*1 + 2*2 + 3*(-3) = 1 + 4 - 9 = -4 ≠ 0
            CHECK(!ep.is_perpendicular(perpendicular_l));
        }
        
        SUBCASE("Elliptic line perpendicular check") {
            projgeom::EllipticLine<int64_t> el(projgeom::PgLine<int64_t>({1, 2, 3}));
            projgeom::PgPoint<int64_t> p({1, 2, 3});
            
            CHECK(!el.is_perpendicular(p));
        }
    }
    
    /*
    TEST_CASE("test_ell_point") {
        projgeom::EllipticPoint<int64_t> a_1(projgeom::PgPoint<int64_t>({13, 23, 32}));
        projgeom::EllipticPoint<int64_t> a_2(projgeom::PgPoint<int64_t>({44, -34, 2}));
        projgeom::EllipticPoint<int64_t> a_3(projgeom::PgPoint<int64_t>({-2, 12, 23}));
        
        check_ck_plane<projgeom::EllipticPoint<int64_t>, projgeom::EllipticLine<int64_t>>(a_1, a_2, a_3);
    }
    
    TEST_CASE("test_ell_line") {
        projgeom::EllipticLine<int64_t> a_1(projgeom::PgLine<int64_t>({13, 23, 32}));
        projgeom::EllipticLine<int64_t> a_2(projgeom::PgLine<int64_t>({44, -34, 2}));
        projgeom::EllipticLine<int64_t> a_3(projgeom::PgLine<int64_t>({-2, 12, 23}));
        
        check_ck_plane<projgeom::EllipticLine<int64_t>, projgeom::EllipticPoint<int64_t>>(a_1, a_2, a_3);
    }
    */
}