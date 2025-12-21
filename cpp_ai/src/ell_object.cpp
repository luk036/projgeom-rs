#include "projgeom/ell_object.hpp"

namespace projgeom {

// Explicit template instantiations
template struct EllipticPoint<int64_t>;
template struct EllipticLine<int64_t>;

} // namespace projgeom