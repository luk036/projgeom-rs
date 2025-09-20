# Project Overview

This project, `projgeom-rs`, is a Rust library for working with projective geometry. It provides a set of generic traits and structures for representing and manipulating geometric objects in projective, elliptic, hyperbolic, and Euclidean planes. The library is designed to be flexible and extensible, allowing users to define their own geometric objects and operations.

The core of the library is built around a set of traits that define the behavior of points and lines in different geometric spaces. These traits include `ProjectivePlanePrimitive`, `ProjectivePlane`, `CayleyKleinPlanePrimitive`, and `CayleyKleinPlane`. The library also provides concrete implementations of these traits for various geometric objects, such as `PgPoint`, `PgLine`, `EllipticPoint`, `EllipticLine`, etc.

# Building and Running

The following commands are useful for building, testing, and running the project:

*   **Build and run in release mode:**
    ```shell
    cargo build --release && cargo run --release
    ```

*   **Run Clippy (linter):**
    ```shell
    cargo clippy --all-targets --all-features --workspace
    ```

*   **Run all tests:**
    ```shell
    cargo test --all-features --workspace
    ```

*   **Check code formatting:**
    ```shell
    cargo fmt --all -- --check
    ```

*   **Format the code:**
    ```shell
    cargo fmt --all
    ```

# Development Conventions

*   **Contributions:** Before making any significant changes, it is recommended to open an issue to discuss the proposed changes.
*   **Changelog:** All pull requests should update the `CHANGELOG.md` file with a summary of the changes.
*   **Code Style:** The project follows the standard Rust code style, which can be enforced using `cargo fmt`.
*   **Testing:** The project has an extensive test suite that uses `quickcheck` for property-based testing. All new features should be accompanied by corresponding tests.
