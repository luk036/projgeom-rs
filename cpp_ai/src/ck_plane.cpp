#include "projgeom/ck_plane.hpp"

namespace projgeom {

// Explicit template instantiations
template std::array<PgLine<int64_t>, 3> tri_dual<int64_t>(const std::array<PgPoint<int64_t>, 3>&);
template std::array<PgPoint<int64_t>, 3> tri_dual<int64_t>(const std::array<PgLine<int64_t>, 3>&);

} // namespace projgeom