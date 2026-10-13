# PerfReport Project Notes

## Project Overview

This project migrates PerfReport, a Windows-specific C++ hardware reporting tool, to Rust using the `sysinfo` crate. It reports logical processor count, memory usage, system uptime, disk usage, and an efficiency score while preserving the original scoring rules. A multi-stage Dockerfile packages the application for Linux containers. GitHub Actions automates formatting, linting, builds, tests, and container verification.

## Architecture and Improvements

- **Memory safety:** Rust checks ownership and borrowing at compile time. The application reads system information through `sysinfo` without application-level unsafe code.
- **Cross-platform portability:** The C++ version calls Windows APIs directly. The Rust version uses one shared codebase for Windows and Linux.
- **Container compatibility:** The Docker build compiles the program in a Rust builder stage and copies the executable into a smaller Debian runtime image.
- **Verification:** Unit tests cover scoring thresholds, combined penalties, and rating boundaries. CI checks native Windows and Linux builds before building and running the container.

## Tools and Development Notes

Rust and Cargo were used to implement, format, build, and test the application. Docker was used to package and run the Linux container. Git and GitHub manage the source history, and GitHub Actions runs the automated checks.

AI assistance was used for standard development support, syntax lookup, and troubleshooting during the migration. All project code was reviewed, tested locally, and validated through CI. Local verification included Windows execution, formatting, strict Clippy checks, unit tests, and a Docker build and run. GitHub Actions verified the Windows and Linux builds and tests, then built and ran the container.
