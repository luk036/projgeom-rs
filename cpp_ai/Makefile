CXX = g++
CXXFLAGS = -std=c++23 -Wall -Wextra -Iinclude
AR = ar
ARFLAGS = rcs

# Source files
SOURCES = $(wildcard src/*.cpp)
OBJECTS = $(SOURCES:.cpp=.o)
LIBRARY = libprojgeom-cpp.a

# Default target
all: $(LIBRARY)

# Library
$(LIBRARY): $(OBJECTS)
	$(AR) $(ARFLAGS) $@ $^

# Object files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean
clean:
	rm -f $(OBJECTS) $(LIBRARY)

# Test (requires doctest)
test: $(LIBRARY)
	@echo "Tests require doctest to be installed"
	@echo "Use xmake to build tests or install doctest manually"

.PHONY: all clean test