// ============================================================================
//  PerfReport (Rust)  |  IS2083 Lab 3
// ============================================================================
//  Reports processors, memory, uptime, and disk usage using sysinfo.
//  The lab's original scoring logic and rating thresholds are preserved.
// ============================================================================

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// --- Given: findings + scoring (do not change) ------------------------------
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");

    // This line makes the sysinfo tools available. You will read from `sys`.
    let mut sys = System::new_all();
    sys.refresh_all();

    // Section 1: logical processors.
    let cpus = sys.cpus().len();
    println!("Logical processors : {}", cpus);

    // Section 2: memory. sysinfo reports memory in bytes.
    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB)  : {:.2}", total as f64 / GB);
    println!("Memory free  (GB)  : {:.2}", (total - used) as f64 / GB);
    println!("Memory in use (%)  : {}", mem_load);

    // Section 3: uptime is reported in seconds; convert to minutes.
    println!("Uptime (minutes)   : {}", System::uptime() / 60);

    // Section 4: use the first disk, as directed by the lab handout.
    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct = 0.0;
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        if dtotal > 0 {
            disk_used_pct = 100.0 * (dtotal - dfree) as f64 / dtotal as f64;
        }
        println!("Disk total (GB)    : {:.2}", dtotal as f64 / GB);
        println!("Disk free  (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used  (%)     : {:.2}", disk_used_pct);
    } else {
        eprintln!("Warning: no disk was detected; disk usage defaults to 0%.");
    }

    // Section 5: score (given, once the two variables above are real)
    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score   : {} / 100 ({})", score, rating(score));
    println!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_penalties_use_strict_thresholds() {
        for (mem_load, expected) in [(0, 100), (70, 100), (71, 90), (85, 90), (86, 75), (100, 75)] {
            assert_eq!(
                compute_score(&Findings {
                    mem_load,
                    disk_used_pct: 0.0
                }),
                expected
            );
        }
    }

    #[test]
    fn disk_penalties_use_strict_thresholds() {
        for (disk_used_pct, expected) in [
            (0.0, 100),
            (75.0, 100),
            (75.01, 90),
            (90.0, 90),
            (90.01, 75),
            (100.0, 75),
        ] {
            assert_eq!(
                compute_score(&Findings {
                    mem_load: 0,
                    disk_used_pct
                }),
                expected
            );
        }
    }

    #[test]
    fn penalties_combine_independently() {
        for (mem_load, disk_used_pct, expected) in [
            (71, 76.0, 80),
            (86, 76.0, 65),
            (71, 91.0, 65),
            (86, 91.0, 50),
        ] {
            assert_eq!(
                compute_score(&Findings {
                    mem_load,
                    disk_used_pct
                }),
                expected
            );
        }
    }

    #[test]
    fn ratings_use_expected_boundaries() {
        for (score, expected) in [
            (100, "GOOD"),
            (85, "GOOD"),
            (84, "FAIR"),
            (60, "FAIR"),
            (59, "POOR"),
            (0, "POOR"),
        ] {
            assert_eq!(rating(score), expected);
        }
    }
}
