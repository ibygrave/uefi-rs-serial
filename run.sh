#!/bin/bash
set -ex

exe=$1

HERE=$(realpath $(dirname $0))

mkdir -p $HERE/downloads
mkdir -p $HERE/esp/efi/boot

if [ ! -f $HERE/downloads/ovmf.rpm ]
then
  curl -o $HERE/downloads/ovmf.rpm 'https://www.kraxel.org/repos/jenkins/edk2/edk2.git-ovmf-x64-0-20220719.209.gf0064ac3af.EOL.no.nore.updates.noarch.rpm'
  rpm2cpio $HERE/downloads/ovmf.rpm | cpio -i -D $HERE/downloads --make-directories
fi

if [ ! -d $HERE/flash ]
then
  mkdir -p $HERE/flash
  cp \
    $HERE/downloads/usr/share/edk2.git/ovmf-x64/OVMF_CODE-pure-efi.fd \
    $HERE/flash/OVMF_CODE.fd
  cp \
    $HERE/downloads/usr/share/edk2.git/ovmf-x64/OVMF_VARS-pure-efi.fd \
    $HERE/flash/OVMF_VARS.fd
fi

cp $exe $HERE/esp/efi/boot/bootx64.efi

qemu-system-x86_64 \
  -drive if=pflash,format=raw,readonly=on,file=$HERE/flash/OVMF_CODE.fd \
  -drive if=pflash,format=raw,file=$HERE/flash/OVMF_VARS.fd \
  -drive format=raw,file=fat:rw:esp \
  -serial vc -serial vc
