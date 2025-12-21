#include "projgeom/pg_object.hpp"

namespace projgeom {

// Explicit template instantiations
template struct PgPoint<int64_t>;
template struct PgLine<int64_t>;

template int64_t dot_product<int64_t>(const std::array<int64_t, 3>&, const std::array<int64_t, 3>&);
template int64_t dot1<int64_t>(const std::array<int64_t, 2>&, const std::array<int64_t, 2>&);
template int64_t cross2<int64_t>(const std::array<int64_t, 2>&, const std::array<int64_t, 2>&);
template std::array<int64_t, 3> cross_product<int64_t>(const std::array<int64_t, 3>&, const std::array<int64_t, 3>&);
template std::array<int64_t, 3> plucker_operation<int64_t>(int64_t, const std::array<int64_t, 3>&, int64_t, const std::array<int64_t, 3>&);

} // namespace projgeom