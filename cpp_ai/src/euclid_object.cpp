#include "projgeom/euclid_object.hpp"

namespace projgeom {

// Explicit template instantiations
template struct EuclidPoint<int64_t>;
template struct EuclidLine<int64_t>;

// Note: tri_altitude and orthocenter are commented out in header due to altitude method issues
// template std::array<EuclidLine<int64_t>, 3> tri_altitude<int64_t>(const std::array<EuclidPoint<int64_t>, 3>&);
// template EuclidPoint<int64_t> orthocenter<int64_t>(const std::array<EuclidPoint<int64_t>, 3>&);

} // namespace projgeom