#include <doctest/doctest.h>
#include "projgeom.hpp"

TEST_SUITE("pg_plane") {
    TEST_CASE("PgPoint incident") {
        SUBCASE("Point not on line") {
            projgeom::PgPoint p({1, 1, 1}); // Point (1,1) in Euclidean plane
            projgeom::PgLine l({1, 1, 0}); // Line x + y = 0 in Euclidean plane
            
            // Point (1,1) is not on line x+y=0
            CHECK(!p.incident(l));
        }
        
        SUBCASE("Point on line") {
            projgeom::PgPoint p_on_l({1, -1, 1}); // Point (1,-1) on line x+y=0
            projgeom::PgLine l({1, 1, 0}); // Line x+y=0
            
            CHECK(p_on_l.incident(l));
            
            projgeom::PgPoint p({1, 1, 1}); // Point (1,1)
            projgeom::PgLine l_through_p({1, -1, 0}); // Line x-y=0, passes through (1,1)
            
            CHECK(p.incident(l_through_p));
        }
    }
    
    TEST_CASE("PgLine incident") {
        SUBCASE("Line not incident with point") {
            projgeom::PgLine l_not_incident({1, 0, 0}); // Line x=0
            projgeom::PgPoint p_not_incident({1, 1, 1}); // Point (1,1)
            
            CHECK(!l_not_incident.incident(p_not_incident));
        }
        
        SUBCASE("Line incident with point") {
            projgeom::PgLine l_on_p({1, 1, -2}); // Line x+y-2=0, passes through (1,1)
            projgeom::PgPoint p({1, 1, 1}); // Point (1,1)
            
            CHECK(l_on_p.incident(p));
            
            projgeom::PgLine l({1, -1, 0}); // Line x-y=0
            projgeom::PgPoint p_on_l({1, 1, 1}); // Point (1,1)
            
            CHECK(l.incident(p_on_l));
        }
    }
    
    TEST_CASE("PgPoint meet") {
        SUBCASE("Meet of two points at infinity") {
            projgeom::PgPoint p1({1, 0, 0}); // Point at infinity on x-axis
            projgeom::PgPoint p2({0, 1, 0}); // Point at infinity on y-axis
            projgeom::PgLine line_at_infinity({0, 0, 1}); // Line at infinity
            
            // Meet of two points is the line connecting them
            CHECK(p1.meet(p2) == line_at_infinity);
        }
        
        SUBCASE("Meet of two Euclidean points") {
            projgeom::PgPoint p3({1, 2, 1}); // Euclidean point (1,2)
            projgeom::PgPoint p4({3, 4, 1}); // Euclidean point (3,4)
            projgeom::PgLine line_p3_p4 = p3.meet(p4);
            
            // The line connecting (1,2) and (3,4) should be x - y + 1 = 0, or [1, -1, 1]
            // cross_product([1,2,1], [3,4,1]) = [2*1 - 1*4, 1*3 - 1*1, 1*4 - 2*3] = [-2, 2, -2]
            // This is homogeneous to [1, -1, 1]
            CHECK(line_p3_p4 == projgeom::PgLine({1, -1, 1}));
        }
    }
    
    TEST_CASE("PgLine meet") {
        SUBCASE("Meet of two lines at origin") {
            projgeom::PgLine l1({1, 0, 0}); // Line x=0 (y-axis)
            projgeom::PgLine l2({0, 1, 0}); // Line y=0 (x-axis)
            projgeom::PgPoint origin({0, 0, 1}); // Origin (0,0)
            
            // Meet of two lines is their intersection point
            CHECK(l1.meet(l2) == origin);
        }
        
        SUBCASE("Meet of two Euclidean lines") {
            projgeom::PgLine l3({1, -1, 0}); // Line x - y = 0
            projgeom::PgLine l4({1, 1, -2}); // Line x + y - 2 = 0
            projgeom::PgPoint intersection_point = l3.meet(l4);
            
            // Intersection of x-y=0 and x+y-2=0 is (1,1)
            // cross_product([1,-1,0], [1,1,-2]) = [(-1)*(-2) - 0*1, 0*1 - 1*(-2), 1*1 - (-1)*1] = [2, 2, 2]
            // This is homogeneous to [1, 1, 1]
            CHECK(intersection_point == projgeom::PgPoint({1, 1, 1}));
        }
    }
    
    TEST_CASE("PgPoint parametrize") {
        projgeom::PgPoint p1({1, 0, 0});
        projgeom::PgPoint p2({0, 1, 0});
        
        SUBCASE("Parametrize with lambda=1, mu=1") {
            projgeom::PgPoint p_mid = p1.parametrize(1, p2, 1);
            CHECK(p_mid == projgeom::PgPoint({1, 1, 0}));
        }
        
        SUBCASE("Parametrize with lambda=2, mu=1") {
            projgeom::PgPoint p_weighted = p1.parametrize(2, p2, 1);
            CHECK(p_weighted == projgeom::PgPoint({2, 1, 0}));
        }
        
        SUBCASE("Parametrize with lambda=0, mu=1") {
            projgeom::PgPoint p_only_p2 = p1.parametrize(0, p2, 1);
            CHECK(p_only_p2 == p2);
        }
    }
    
    TEST_CASE("PgLine parametrize") {
        projgeom::PgLine l1({1, 0, 0});
        projgeom::PgLine l2({0, 1, 0});
        
        SUBCASE("Parametrize with lambda=1, mu=1") {
            projgeom::PgLine l_mid = l1.parametrize(1, l2, 1);
            CHECK(l_mid == projgeom::PgLine({1, 1, 0}));
        }
        
        SUBCASE("Parametrize with lambda=2, mu=1") {
            projgeom::PgLine l_weighted = l1.parametrize(2, l2, 1);
            CHECK(l_weighted == projgeom::PgLine({2, 1, 0}));
        }
        
        SUBCASE("Parametrize with lambda=0, mu=1") {
            projgeom::PgLine l_only_l2 = l1.parametrize(0, l2, 1);
            CHECK(l_only_l2 == l2);
        }
    }
    
    TEST_CASE("coincident") {
        SUBCASE("Three collinear points") {
            projgeom::PgPoint p1({1, 2, 1});
            projgeom::PgPoint p2({2, 4, 2}); // Same as p1 (homogeneous)
            projgeom::PgPoint p3({3, 6, 3}); // Same line
            
            CHECK(projgeom::coincident(p1, p2, p3));
        }
        
        SUBCASE("Three non-collinear points") {
            projgeom::PgPoint p1({1, 0, 1});
            projgeom::PgPoint p2({0, 1, 1});
            projgeom::PgPoint p3({1, 1, 1});
            
            CHECK(!projgeom::coincident(p1, p2, p3));
        }
    }
    
    TEST_CASE("harm_conj") {
        projgeom::PgPoint p1({1, 0, 1});
        projgeom::PgPoint p2({0, 1, 1});
        projgeom::PgPoint p3({1, 1, 2});
        
        SUBCASE("Harmonic conjugate of points") {
            projgeom::PgPoint h = projgeom::harm_conj(p1, p2, p3);
            projgeom::PgPoint p3_again = projgeom::harm_conj(p1, p2, h);
            
            // Should get back original point
            CHECK(p3 == p3_again);
        }
        
        SUBCASE("Harmonic conjugate of lines") {
            projgeom::PgLine l1({1, 0, 1});
            projgeom::PgLine l2({0, 1, 1});
            projgeom::PgLine l3({1, 1, 2});
            
            projgeom::PgLine h = projgeom::harm_conj(l1, l2, l3);
            projgeom::PgLine l3_again = projgeom::harm_conj(l1, l2, h);
            
            CHECK(l3 == l3_again);
        }
    }
}