#include <doctest/doctest.h>
#include "projgeom.hpp"



TEST_SUITE("geometries") {
    TEST_CASE("Hyperbolic geometry") {
        /*
        SUBCASE("test_hyp_point") {
            projgeom::HyperbolicPoint<int64_t> a_1(projgeom::PgPoint<int64_t>({13, 23, 32}));
            projgeom::HyperbolicPoint<int64_t> a_2(projgeom::PgPoint<int64_t>({44, -34, 2}));
            projgeom::HyperbolicPoint<int64_t> a_3(projgeom::PgPoint<int64_t>({-2, 12, 23}));
            
            check_ck_plane<projgeom::HyperbolicPoint<int64_t>, projgeom::HyperbolicLine<int64_t>>(a_1, a_2, a_3);
        }
        
        SUBCASE("test_hyp_line") {
            projgeom::HyperbolicLine<int64_t> a_1(projgeom::PgLine<int64_t>({13, 23, 32}));
            projgeom::HyperbolicLine<int64_t> a_2(projgeom::PgLine<int64_t>({44, -34, 2}));
            projgeom::HyperbolicLine<int64_t> a_3(projgeom::PgLine<int64_t>({-2, 12, 23}));
            
            check_ck_plane<projgeom::HyperbolicLine<int64_t>, projgeom::HyperbolicPoint<int64_t>>(a_1, a_2, a_3);
        }
        */
    }
    
    TEST_CASE("MyCK geometry") {
        /*
        SUBCASE("test_myck_point") {
            projgeom::MyCKPoint<int64_t> a_1(projgeom::PgPoint<int64_t>({13, 23, 32}));
            projgeom::MyCKPoint<int64_t> a_2(projgeom::PgPoint<int64_t>({44, -34, 2}));
            projgeom::MyCKPoint<int64_t> a_3(projgeom::PgPoint<int64_t>({-2, 12, 23}));
            
            check_ck_plane<projgeom::MyCKPoint<int64_t>, projgeom::MyCKLine<int64_t>>(a_1, a_2, a_3);
        }
        
        SUBCASE("test_myck_line") {
            projgeom::MyCKLine<int64_t> a_1(projgeom::PgLine<int64_t>({13, 23, 32}));
            projgeom::MyCKLine<int64_t> a_2(projgeom::PgLine<int64_t>({44, -34, 2}));
            projgeom::MyCKLine<int64_t> a_3(projgeom::PgLine<int64_t>({-2, 12, 23}));
            
            check_ck_plane<projgeom::MyCKLine<int64_t>, projgeom::MyCKPoint<int64_t>>(a_1, a_2, a_3);
        }
        */
    }
    
    TEST_CASE("Perspective geometry") {
        /*
        SUBCASE("test_persp_point") {
            projgeom::PerspPoint<int64_t> a_1(projgeom::PgPoint<int64_t>({13, 23, 32}));
            projgeom::PerspPoint<int64_t> a_2(projgeom::PgPoint<int64_t>({44, -34, 2}));
            projgeom::PerspPoint<int64_t> a_3(projgeom::PgPoint<int64_t>({-2, 12, 23}));
            
            check_ck_plane<projgeom::PerspPoint<int64_t>, projgeom::PerspLine<int64_t>>(a_1, a_2, a_3);
        }
        */
        
        SUBCASE("Perspective line parallel check") {
            projgeom::PerspLine<int64_t> l1(projgeom::PgLine<int64_t>({1, 0, 1}));
            projgeom::PerspLine<int64_t> l2(projgeom::PgLine<int64_t>({2, 0, 2})); // Same direction
            
            // These should be parallel (meet on L_INF)
            CHECK(l1.is_parallel(l2));
            
            projgeom::PerspLine<int64_t> l3(projgeom::PgLine<int64_t>({0, 1, 1}));
            CHECK(!l1.is_parallel(l3));
        }
    }
    
    TEST_CASE("Euclidean geometry") {
        /*
        SUBCASE("test_euclid_point") {
            projgeom::EuclidPoint<int64_t> a_1(projgeom::PgPoint<int64_t>({13, 23, 32}));
            projgeom::EuclidPoint<int64_t> a_2(projgeom::PgPoint<int64_t>({44, -34, 2}));
            projgeom::EuclidPoint<int64_t> a_3(projgeom::PgPoint<int64_t>({-2, 12, 23}));
            
            check_ck_plane<projgeom::EuclidPoint<int64_t>, projgeom::EuclidLine<int64_t>>(a_1, a_2, a_3);
        }
        */
        
        /*
        SUBCASE("Euclidean line operations") {
            projgeom::EuclidLine<int64_t> l1(projgeom::PgLine<int64_t>({1, 0, -1})); // x = 1
            projgeom::EuclidLine<int64_t> l2(projgeom::PgLine<int64_t>({2, 0, -5})); // x = 2.5 (parallel to l1)
            projgeom::EuclidLine<int64_t> l3(projgeom::PgLine<int64_t>({0, 1, -1})); // y = 1 (not parallel to l1)
            
            CHECK(l1.is_parallel(l2));
            CHECK(!l1.is_parallel(l3));
            
            CHECK(l1.is_perpendicular(l3)); // x=1 is perpendicular to y=1
            CHECK(!l1.is_perpendicular(l2)); // Parallel lines not perpendicular
            
            projgeom::EuclidPoint<int64_t> p(projgeom::PgPoint<int64_t>({1, 2, 1})); // Point (1,2)
            projgeom::EuclidLine<int64_t> alt = l1.altitude(p);
            // The altitude from (1,2) to x=1 is the line y=2, which is (0,1,-2)
            CHECK(alt == projgeom::EuclidLine<int64_t>(projgeom::PgLine<int64_t>({0, 1, -2})));
        }
        */
        
        SUBCASE("Euclidean point midpoint") {
            projgeom::EuclidPoint<int64_t> p1(projgeom::PgPoint<int64_t>({0, 0, 1}));
            projgeom::EuclidPoint<int64_t> p2(projgeom::PgPoint<int64_t>({2, 4, 1}));
            projgeom::EuclidPoint<int64_t> mid = p1.midpoint(p2);
            
            // Midpoint of (0,0) and (2,4) is (1,2)
            CHECK(mid == projgeom::EuclidPoint<int64_t>(projgeom::PgPoint<int64_t>({1, 2, 1})));
        }
        
        /*
        SUBCASE("Euclidean triangle operations") {
            projgeom::EuclidPoint<int64_t> p1(projgeom::PgPoint<int64_t>({0, 0, 1}));
            projgeom::EuclidPoint<int64_t> p2(projgeom::PgPoint<int64_t>({2, 0, 1}));
            projgeom::EuclidPoint<int64_t> p3(projgeom::PgPoint<int64_t>({1, 3, 1}));
            const std::array triangle = {p1, p2, p3};
            
            const auto altitudes = projgeom::tri_altitude(triangle);
            // For p1(0,0), opposite side is line p2p3
            // Line p2p3: (3, -1, -6) or (1, -1/3, -2)
            // Perpendicular through (0,0) is (-1, 3, 0)
            CHECK(altitudes[0] == projgeom::EuclidLine<int64_t>(projgeom::PgLine<int64_t>({-1, 3, 0})));
            
            const auto ortho = projgeom::orthocenter(triangle);
            // Orthocenter of triangle (0,0), (2,0), (1,3) is (1, 1/3)
            CHECK(ortho == projgeom::EuclidPoint<int64_t>(projgeom::PgPoint<int64_t>({3, 1, 3})));
        }
        */
    }
    
    TEST_CASE("Projective plane axioms check_axiom") {
        projgeom::PgPoint pt_p(std::array<int64_t, 3>{1, 2, 3});
        projgeom::PgPoint pt_q(std::array<int64_t, 3>{4, 5, 6});
        projgeom::PgLine ln_l(std::array<int64_t, 3>{1, 1, -1});
        
        // This should not throw assertions
        projgeom::check_axiom(pt_p, pt_q, ln_l);
    }
}