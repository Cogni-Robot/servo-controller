@echo off
REM Script de build pour Windows

echo ========================================
echo Build ST3215 Servo Controller
echo ========================================

echo.
echo [1/3] Compilation de la bibliotheque Rust...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ERREUR: La compilation Rust a echoue
    exit /b 1
)

echo.
echo [2/3] Generation du header C/C++...
if exist "include\st3215.h" (
    echo Header deja genere
) else (
    echo Le header sera genere automatiquement par build.rs
)

echo.
echo [3/3] Compilation de l'exemple C++...
if not exist "build" mkdir build
cd build
cmake -G "Visual Studio 17 2022" -DCMAKE_BUILD_TYPE=Release ..\examples\cpp
if %ERRORLEVEL% NEQ 0 (
    echo ERREUR: La configuration CMake a echoue
    cd ..
    exit /b 1
)

cmake --build . --config Release
if %ERRORLEVEL% NEQ 0 (
    echo ERREUR: La compilation C++ a echoue
    cd ..
    exit /b 1
)

cd ..

echo.
echo ========================================
echo Build termine avec succes!
echo ========================================
echo.
echo Bibliotheque Rust: target\release\st3215.dll
echo Header C/C++: include\st3215.h
echo Exemple C++: build\Release\example.exe
echo.
echo Pour executer l'exemple C++:
echo   cd build\Release
echo   example.exe
echo.
