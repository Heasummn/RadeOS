#!/usr/bin/env bash
export NAME=RadeOS
export ARCH=amd64
export CROSS=$HOME/opt/cross
export PATH=$CROSS/bin:$PATH

export SRC_DIR=kernel
export BUILD_DIR=build/$ARCH

export KERNEL=$BUILD_DIR/$NAME.bin
export ISO=$NAME.iso

. ./clean.sh

make all
