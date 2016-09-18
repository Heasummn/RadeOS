#!/usr/bin/env bash

set -e

. ./build.sh

debug=false

log ()
{
    if [ "$debug" == "true" ]; then
        echo "$1"
    fi
}


if [ "$1" != "" ]; then
    if [ "$1" == "-d" ]; then
        debug=true
    fi
fi
log "Making isodir directory."
mkdir -p build/isodir/boot/grub

log "Copying kernel into boot directory."
cp $KERNEL build/isodir/boot

log "Creating grub.cfg."
cat > build/isodir/boot/grub/grub.cfg << EOF
set timeout=0
set default=0

menuentry "$NAME" {
	multiboot2 /boot/$NAME.bin
  	boot
}
EOF
log "Making iso file."
grub-mkrescue -o $NAME.iso build/isodir
