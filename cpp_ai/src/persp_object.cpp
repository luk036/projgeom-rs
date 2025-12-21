#include "projgeom/persp_object.hpp"

namespace projgeom {

// Explicit template instantiations
template struct PerspPoint<int64_t>;
template struct PerspLine<int64_t>;

} // namespace projgeom