#!/bin/bash
# Script de build pour Linux/macOS

set -e

echo "========================================"
echo "Build ST3215 Servo Controller"
echo "========================================"

echo ""
echo "[1/3] Compilation de la bibliothèque Rust..."
cargo build --release

echo ""
echo "[2/3] Génération du header C/C++..."
if [ -f "include/st3215.h" ]; then
    echo "Header déjà généré"
else
    echo "Le header sera généré automatiquement par build.rs"
fi

echo ""
echo "[3/3] Compilation de l'exemple C++..."
mkdir -p build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ../examples/cpp
cmake --build . --config Release
cd ..

echo ""
echo "========================================"
echo "Build terminé avec succès!"
echo "========================================"
echo ""
echo "Bibliothèque Rust: target/release/libst3215.so (Linux) ou libst3215.dylib (macOS)"
echo "Header C/C++: include/st3215.h"
echo "Exemple C++: build/example"
echo ""
echo "Pour exécuter l'exemple C++:"
echo "  ./build/example"
echo ""
