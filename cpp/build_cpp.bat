@echo off
setlocal
REM ===========================================================================
REM  build_cpp.bat  -  one-click build for the C++ PerfReport (IS2083 Lab 3)
REM  Put this next to perfreport.cpp (for example on your Desktop), then just
REM  double-click it. It finds the C++ compiler for you.
REM ===========================================================================

cd /d "%~dp0"

where cl >nul 2>nul
if %errorlevel%==0 goto build

set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" (
    echo.
    echo  Could not find a C++ compiler.
    echo  Install "Visual Studio Build Tools" and check "Desktop development
    echo  with C++", then double-click this file again.
    echo  Download: https://visualstudio.microsoft.com/downloads/
    echo.
    pause
    exit /b 1
)

set "VSPATH="
for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "VSPATH=%%i"

if not defined VSPATH (
    echo.
    echo  Visual Studio is installed but the C++ tools are missing.
    echo  Re-run the installer and add "Desktop development with C++".
    echo.
    pause
    exit /b 1
)

call "%VSPATH%\VC\Auxiliary\Build\vcvars64.bat" >nul

:build
echo.
echo  Building perfreport.exe ...
echo.
cl /nologo /EHsc /O2 perfreport.cpp /Fe:perfreport.exe

echo.
if exist perfreport.exe (
    echo  ============================================================
    echo   SUCCESS: perfreport.exe is built. Double-click it to run.
    echo  ============================================================
) else (
    echo  ============================================================
    echo   BUILD FAILED. Read the messages above.
    echo  ============================================================
)
echo.
pause
