#pragma once

#include "pch.hpp"

namespace projgeom {

template<typename T>
concept Arithmetic = std::is_arithmetic_v<T>;

template<typename T>
concept Integral = std::integral<T>;

template<typename T>
concept SignedIntegral = Integral<T> && std::signed_integral<T>;

template<typename Point, typename Line>
concept ProjectivePlanePrimitive = requires(const Point& p, const Point& q, const Line& l) {
    { p.meet(q) } -> std::convertible_to<Line>;
    { p.incident(l) } -> std::same_as<bool>;
    { p == q } -> std::same_as<bool>;
    { p != q } -> std::same_as<bool>;
};

template<typename Point, typename Line, typename Scalar>
concept ProjectivePlane = ProjectivePlanePrimitive<Point, Line> && requires(
    const Point& p, const Point& q, Scalar lambda, Scalar mu) {
    { p.parametrize(lambda, q, mu) } -> std::convertible_to<Point>;
    { coincident(p, q, p.parametrize(lambda, q, mu)) } -> std::same_as<bool>;
};

template<typename Point, typename Line, typename Scalar>
concept CayleyKleinPlane = ProjectivePlane<Point, Line, Scalar> && requires(
    const Point& p, const Line& l, const std::array<Point, 3>& triangle) {
    { p.is_perpendicular(l) } -> std::same_as<bool>;
    { orthocenter(triangle) } -> std::convertible_to<Point>;
    { tri_dual(triangle) } -> std::convertible_to<std::array<Line, 3>>;
    { tri_altitude(triangle) } -> std::convertible_to<std::array<Line, 3>>;
};

template<typename T, typename Scalar>
concept HasDotProduct = requires(const T& a, const T& b) {
    { dot_product(a, b) } -> std::same_as<Scalar>;
};

template<typename T, typename Scalar>
concept HasCrossProduct = requires(const T& a, const T& b) {
    { cross_product(a, b) } -> std::same_as<T>;
};

template<typename T, typename Scalar>
concept Vector3D = requires(const T& v) {
    requires std::same_as<std::remove_cvref_t<decltype(v[0])>, Scalar>;
    requires std::same_as<std::remove_cvref_t<decltype(v[1])>, Scalar>;
    requires std::same_as<std::remove_cvref_t<decltype(v[2])>, Scalar>;
    { v.size() } -> std::same_as<std::size_t>;
} && HasDotProduct<T, Scalar> && HasCrossProduct<T, Scalar>;

} // namespace projgeom