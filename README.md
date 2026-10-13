# PerfReport — IS2083 Lab 3

A Rust migration of the supplied Windows C++ PerfReport. The program reports logical processors, RAM, uptime, disk usage, and the original efficiency score using `sysinfo = "0.33"`.

## Run on Windows

Open PowerShell in this folder:

```powershell
cargo run --locked
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
cargo test --locked
```

The release executable is created at `target\release\perfreport.exe`.

## How the migration works

- `System::new_all()` and `refresh_all()` collect the operating system readings.
- `sys.cpus().len()` counts logical processors.
- Memory readings are in bytes. Dividing by `1024^3` produces the lab's GB display (technically GiB). Used memory divided by total memory produces the integer percentage; the zero-total guard prevents division by zero.
- `System::uptime()` returns seconds; dividing by 60 gives whole minutes.
- `Disks::new_with_refreshed_list()` finds disks. The first disk supplies total and available space. Used space is total minus available space.
- Memory above 70% costs 10 points; above 85% costs 25 instead. Disk usage above 75% costs 10 points; above 90% costs 25 instead. The two penalties add together.
- Scores of 85 or more are GOOD; 60–84 are FAIR; lower scores are POOR. The supplied scoring functions are unchanged.
- Four unit tests cover exact threshold boundaries, combined penalties, and ratings.

The C++ source is preserved in `cpp/`. Run its supplied `build_cpp.bat` to build it with Visual Studio C++ tools.

## Comparison caveats

Measurements taken at different times can vary. The Rust implementation follows the lab's first-disk rule; the C++ program explicitly reads `C:\`. The first Rust disk is not guaranteed to be `C:\` on every Windows computer or `/` on Linux. On this PC, the observed disk total matched the C++ baseline.

Container readings depend on what Linux exposes to the container. They can reflect host-wide RAM, CPU, and uptime, rather than the container's resource limits or time since launch. Missing disk readings produce a warning and use the starter's zero disk penalty.

## Docker

Start Docker Desktop with Linux containers enabled, then run:

```powershell
docker build -t perfreport .
docker run --rm perfreport
```

The multi-stage Dockerfile builds with Rust 1.97.1 on Debian Bookworm, then copies only the executable into a Debian Bookworm slim runtime. It copies `Cargo.lock` and builds with `--locked` for repeatable dependency selection. The builder version differs from the handout's old 1.82 image to match the tested local compiler and avoid newer dependency compiler requirements. Both stages use Bookworm to keep the runtime C library compatible.

## GitHub Actions

`.github/workflows/ci.yml` defines:

- `test`: formatting, strict Clippy, release build, unit tests, and a native report run on Windows and Linux.
- `container`: builds and runs the image after both native checks pass.

The workflow runs on pushes to `main`, pull requests, and manual dispatch. Windows checks and the local Docker build/run passed on October 13, 2026. The Windows, Linux, and container jobs passed: https://github.com/brunobgarcia2007-cpu/perfreport/actions/runs/38448782204.

To publish, create an empty public GitHub repository named `perfreport`. Set your own Git identity if needed, then use the real repository URL in these commands:

```powershell
git init
git add .
git commit -m "Rust PerfReport with Docker and CI"
git branch -M main
git remote add origin https://github.com/brunobgarcia2007-cpu/perfreport.git
git push -u origin main
```

Sign in when Git requests authentication. Open the repository's Actions tab and confirm every job has passed.

## Submission still required

The provided Word template requires seven genuine screenshots: C++ output; Rust/Cargo versions; project creation and Hello World; migrated Rust output; passing tests; Docker build/run; and passing GitHub Actions. Insert the screenshots and public repository URL, then export to the required PDF filename using your first initial and last name. Terminal text logs are supporting evidence, not screenshots.

## API reference

[sysinfo 0.33.1 documentation](https://docs.rs/sysinfo/0.33.1/sysinfo/struct.System.html)
