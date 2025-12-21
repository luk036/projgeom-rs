#!/usr/bin/env python3
"""Simple test runner to verify the conversion."""

import os
import sys
import traceback

# Add src to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "src"))

# Import the test functions
test_file_path = os.path.join(os.path.dirname(__file__), "tests", "test_lib.py")
exec(open(test_file_path).read())


def run_basic_tests():
    """Run a subset of basic tests."""

    try:
        test_pg_point_creation()

        test_pg_line_creation()

        test_pg_point_equality()

        test_pg_line_equality()

        test_pg_point_incident()

        test_pg_point_meet()

        test_pg_line_meet()

        test_pg_point_parametrize()

        return True

    except Exception:
        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = run_basic_tests()
    sys.exit(0 if success else 1)
