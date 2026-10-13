// ============================================================================
//  PerfReport (C++)  -  a tiny Windows utility for IS2083 Lab 3
// ============================================================================
//  It reports a few computer performance and efficiency numbers: processors,
//  memory, uptime, and disk usage, then gives an overall efficiency score.
//
//  This version uses the Windows API, so it ONLY runs on Windows. That is the
//  reason we will migrate it to Rust: the Rust version will run on Windows,
//  Linux, and inside a container, from one code base.
//
//  Build it by double-clicking build_cpp.bat, or:
//      cl /EHsc /O2 perfreport.cpp /Fe:perfreport.exe
// ============================================================================

#include <windows.h>
#include <iostream>
#include <iomanip>

static const char* rating(int score) {
    if (score >= 85) return "GOOD";
    if (score >= 60) return "FAIR";
    return "POOR";
}

int main() {
    const double GB = 1024.0 * 1024.0 * 1024.0;
    std::cout << "==== PerfReport (C++) ====\n";
    std::cout << std::fixed << std::setprecision(2);

    // --- Processors ---------------------------------------------------------
    SYSTEM_INFO si;
    GetSystemInfo(&si);
    std::cout << "Logical processors : " << si.dwNumberOfProcessors << "\n";

    // --- Memory -------------------------------------------------------------
    MEMORYSTATUSEX mem;
    mem.dwLength = sizeof(mem);
    GlobalMemoryStatusEx(&mem);
    unsigned long memLoad = mem.dwMemoryLoad;                 // percent in use
    std::cout << "Memory total (GB)  : " << mem.ullTotalPhys / GB << "\n";
    std::cout << "Memory free  (GB)  : " << mem.ullAvailPhys / GB << "\n";
    std::cout << "Memory in use (%)  : " << memLoad << "\n";

    // --- Uptime -------------------------------------------------------------
    unsigned long long uptimeMin = GetTickCount64() / 1000ULL / 60ULL;
    std::cout << "Uptime (minutes)   : " << uptimeMin << "\n";

    // --- Disk C: ------------------------------------------------------------
    double diskUsedPct = 0.0;
    ULARGE_INTEGER freeForCaller, totalBytes, totalFree;
    if (GetDiskFreeSpaceExA("C:\\", &freeForCaller, &totalBytes, &totalFree)) {
        double dTotalGB = totalBytes.QuadPart / GB;
        double dFreeGB  = totalFree.QuadPart / GB;
        if (dTotalGB > 0.0) diskUsedPct = 100.0 * (dTotalGB - dFreeGB) / dTotalGB;
        std::cout << "Disk C: total (GB) : " << dTotalGB << "\n";
        std::cout << "Disk C: free  (GB) : " << dFreeGB << "\n";
        std::cout << "Disk C: used  (%)  : " << diskUsedPct << "\n";
    }

    // --- Efficiency score ---------------------------------------------------
    int score = 100;
    if      (memLoad > 85) score -= 25;
    else if (memLoad > 70) score -= 10;
    if      (diskUsedPct > 90.0) score -= 25;
    else if (diskUsedPct > 75.0) score -= 10;
    if (score < 0) score = 0;
    std::cout << "Efficiency score   : " << score << " / 100 (" << rating(score) << ")\n";

    std::cout << "Done.\n";
    return 0;
}
