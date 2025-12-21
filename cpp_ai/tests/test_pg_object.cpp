#include <doctest/doctest.h>
#include "projgeom.hpp"

TEST_SUITE("pg_object") {
    TEST_CASE("dot_product") {
        SUBCASE("Test with non-zero vectors") {
            CHECK(projgeom::dot_product({1, 2, 3}, {3, 4, 5}) == 26);
        }
        
        SUBCASE("Test with zero vector") {
            CHECK(projgeom::dot_product({0, 0, 0}, {3, 4, 5}) == 0);
            CHECK(projgeom::dot_product({1, 2, 3}, {0, 0, 0}) == 0);
        }
        
        SUBCASE("Test with negative numbers") {
            CHECK(projgeom::dot_product({-1, -2, -3}, {3, 4, 5}) == -26);
        }
        
        SUBCASE("Test with orthogonal vectors") {
            CHECK(projgeom::dot_product({1, 0, 0}, {0, 1, 0}) == 0);
        }
    }
    
    TEST_CASE("cross_product") {
        SUBCASE("Test with non-zero vectors") {
            CHECK(projgeom::cross_product({1, 2, 3}, {3, 4, 5}) == std::array<int64_t, 3>{-2, 4, -2});
        }
        
        SUBCASE("Test with parallel vectors") {
            CHECK(projgeom::cross_product({1, 2, 3}, {2, 4, 6}) == std::array<int64_t, 3>{0, 0, 0});
        }
        
        SUBCASE("Test with zero vector") {
            CHECK(projgeom::cross_product({0, 0, 0}, {3, 4, 5}) == std::array<int64_t, 3>{0, 0, 0});
            CHECK(projgeom::cross_product({1, 2, 3}, {0, 0, 0}) == std::array<int64_t, 3>{0, 0, 0});
        }
        
        SUBCASE("Test with standard basis vectors") {
            CHECK(projgeom::cross_product({1, 0, 0}, {0, 1, 0}) == std::array<int64_t, 3>{0, 0, 1});
            CHECK(projgeom::cross_product({0, 1, 0}, {0, 0, 1}) == std::array<int64_t, 3>{1, 0, 0});
        }
    }
    
    TEST_CASE("plucker_operation") {
        SUBCASE("Test with basic values") {
            auto result = projgeom::plucker_operation<int64_t>(1, std::array<int64_t, 3>{1, 2, 3}, 1, std::array<int64_t, 3>{3, 4, 5});
            CHECK(result[0] == 4);
            CHECK(result[1] == 6);
            CHECK(result[2] == 8);
        }
        
        SUBCASE("Test with negative lambda") {
            auto result = projgeom::plucker_operation<int64_t>(-1, std::array<int64_t, 3>{1, 2, 3}, 1, std::array<int64_t, 3>{3, 4, 5});
            CHECK(result[0] == 2);
            CHECK(result[1] == 2);
            CHECK(result[2] == 2);
        }
        
        SUBCASE("Test with negative mu") {
            auto result = projgeom::plucker_operation<int64_t>(1, std::array<int64_t, 3>{1, 2, 3}, -1, std::array<int64_t, 3>{3, 4, 5});
            CHECK(result[0] == -2);
            CHECK(result[1] == -2);
            CHECK(result[2] == -2);
        }
        
        SUBCASE("Test with zero lambda") {
            auto result = projgeom::plucker_operation<int64_t>(0, std::array<int64_t, 3>{1, 2, 3}, 1, std::array<int64_t, 3>{3, 4, 5});
            CHECK(result[0] == 3);
            CHECK(result[1] == 4);
            CHECK(result[2] == 5);
        }
        
        SUBCASE("Test with zero mu") {
            auto result = projgeom::plucker_operation<int64_t>(1, std::array<int64_t, 3>{1, 2, 3}, 0, std::array<int64_t, 3>{3, 4, 5});
            CHECK(result[0] == 1);
            CHECK(result[1] == 2);
            CHECK(result[2] == 3);
        }
        
        SUBCASE("Test with both zero") {
            auto result = projgeom::plucker_operation<int64_t>(0, std::array<int64_t, 3>{1, 2, 3}, 0, std::array<int64_t, 3>{3, 4, 5});
            CHECK(result[0] == 0);
            CHECK(result[1] == 0);
            CHECK(result[2] == 0);
        }
    }
    
    TEST_CASE("PgPoint construction and equality") {
        SUBCASE("Test PgPoint::new") {
            projgeom::PgPoint p({1, 2, 3});
            CHECK(p.coord == std::array<int64_t, 3>{1, 2, 3});
        }
        
        SUBCASE("Test PgPoint equality") {
            projgeom::PgPoint p1({1, 2, 3});
            projgeom::PgPoint p2({2, 4, 6}); // Homogeneous equivalent
            projgeom::PgPoint p3({1, 2, 4}); // Different point
            
            CHECK(p1 == p2);
            CHECK(p1 != p3);
        }
    }
    
    TEST_CASE("PgLine construction and equality") {
        SUBCASE("Test PgLine::new") {
            projgeom::PgLine l({1, 2, 3});
            CHECK(l.coord == std::array<int64_t, 3>{1, 2, 3});
        }
        
        SUBCASE("Test PgLine equality") {
            projgeom::PgLine l1({1, 2, 3});
            projgeom::PgLine l2({2, 4, 6}); // Homogeneous equivalent
            projgeom::PgLine l3({1, 2, 4}); // Different line
            
            CHECK(l1 == l2);
            CHECK(l1 != l3);
        }
    }
}