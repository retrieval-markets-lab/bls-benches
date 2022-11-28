#!/bin/bash
BUILD_DIR=../build

run() {
    if [ ! -d "$BUILD_DIR" ]; then
        echo "No build directory found. Creating build directory..."
        mkdir -p "$BUILD_DIR"
    fi

    echo "****Running phase 1 powers of tau****"
    start=`date +%s`
    snarkjs powersoftau new bn128 26 $BUILD_DIR/pot26_0000.ptau -v
    snarkjs powersoftau contribute $BUILD_DIR/pot26_0000.ptau $BUILD_DIR/pot26_0001.ptau --name="First contribution" -v
    snarkjs powersoftau prepare phase2 $BUILD_DIR/pot26_0001.ptau $BUILD_DIR/pot26_final.ptau -v
    end=`date +%s`
    echo "DONE ($((end-start))s)"

}

mkdir -p logs
run 2>&1 | tee logs/ceremony_phase1_$(date '+%Y-%m-%d-%H-%M').log
