#pragma once

#include "concepts.hpp"
#include <array>
#include <cstdint>

namespace projgeom {

template<SignedIntegral Scalar = int64_t>
constexpr Scalar dot_product(const std::array<Scalar, 3>& v_a, const std::array<Scalar, 3>& v_b) noexcept {
    return v_a[0] * v_b[0] + v_a[1] * v_b[1] + v_a[2] * v_b[2];
}

template<SignedIntegral Scalar = int64_t>
constexpr Scalar dot1(const std::array<Scalar, 2>& v_a, const std::array<Scalar, 2>& v_b) noexcept {
    return v_a[0] * v_b[0] + v_a[1] * v_b[1];
}

template<SignedIntegral Scalar = int64_t>
constexpr Scalar cross2(const std::array<Scalar, 2>& v_a, const std::array<Scalar, 2>& v_b) noexcept {
    return v_a[0] * v_b[1] - v_a[1] * v_b[0];
}

template<SignedIntegral Scalar = int64_t>
constexpr std::array<Scalar, 3> cross_product(
    const std::array<Scalar, 3>& v_a, const std::array<Scalar, 3>& v_b) noexcept {
    return {
        v_a[1] * v_b[2] - v_a[2] * v_b[1],
        v_a[2] * v_b[0] - v_a[0] * v_b[2],
        v_a[0] * v_b[1] - v_a[1] * v_b[0]
    };
}

template<SignedIntegral Scalar = int64_t>
constexpr std::array<Scalar, 3> plucker_operation(
    Scalar lambda, const std::array<Scalar, 3>& v_a,
    Scalar mu, const std::array<Scalar, 3>& v_b) noexcept {
    return {
        lambda * v_a[0] + mu * v_b[0],
        lambda * v_a[1] + mu * v_b[1],
        lambda * v_a[2] + mu * v_b[2]
    };
}

// Forward declarations
template<SignedIntegral Scalar> struct PgPoint;
template<SignedIntegral Scalar> struct PgLine;

template<SignedIntegral Scalar = int64_t>
struct PgPoint {
    using ScalarType = Scalar;
    std::array<Scalar, 3> coord;
    
    constexpr PgPoint() noexcept : coord{0, 0, 0} {}
    constexpr explicit PgPoint(std::array<Scalar, 3> c) noexcept : coord(std::move(c)) {}
    
    constexpr bool operator==(const PgPoint& other) const noexcept {
        const auto& coord_a = coord;
        const auto& coord_b = other.coord;
        return cross_product(coord_a, coord_b) == std::array<Scalar, 3>{0, 0, 0};
    }
    
    constexpr bool operator!=(const PgPoint& other) const noexcept {
        return !(*this == other);
    }
    
    constexpr auto operator<=>(const PgPoint&) const = default;
    
    // Projective plane operations
    constexpr bool incident(const PgLine<Scalar>& line) const noexcept;
    constexpr PgLine<Scalar> meet(const PgPoint& other) const noexcept;
    constexpr PgPoint parametrize(Scalar lambda, const PgPoint& other, Scalar mu) const noexcept;
};

template<SignedIntegral Scalar = int64_t>
struct PgLine {
    using ScalarType = Scalar;
    std::array<Scalar, 3> coord;
    
    constexpr PgLine() noexcept : coord{0, 0, 0} {}
    constexpr explicit PgLine(std::array<Scalar, 3> c) noexcept : coord(std::move(c)) {}
    
    constexpr bool operator==(const PgLine& other) const noexcept {
        const auto& coord_a = coord;
        const auto& coord_b = other.coord;
        return cross_product(coord_a, coord_b) == std::array<Scalar, 3>{0, 0, 0};
    }
    
    constexpr bool operator!=(const PgLine& other) const noexcept {
        return !(*this == other);
    }
    
    constexpr auto operator<=>(const PgLine&) const = default;
    
    // Projective plane operations
    constexpr bool incident(const PgPoint<Scalar>& point) const noexcept;
    constexpr PgPoint<Scalar> meet(const PgLine& other) const noexcept;
    constexpr PgLine parametrize(Scalar lambda, const PgLine& other, Scalar mu) const noexcept;
};

} // namespace projgeom