#!/bin/bash

wd=$(pwd)
bd="${wd}/build"

[[ -d ${bd} ]] && rm -rf $bd

mkdir -p "${bd}/bin/src"
mkdir -p "${bd}/web/src"
cp ${wd}/pkgbuild-bin ${bd}/bin/PKGBUILD
cp ${wd}/pkgbuild-web ${bd}/web/PKGBUILD

function build_bin() {
    cd ${bd}/bin
    sed -i "s/md5sums=.*/$(makepkg -g)/g" PKGBUILD
    makepkg -s
    cp *.tar.zst ${bd}/web/src
}

function build_web() {
    cd ${bd}/web
    sed -i "s/md5sums=.*/$(makepkg -g)/g" PKGBUILD
    makepkg -s
    ${bd}/web/src/*.tar.zst ${bd}
    cp *.tar.zst ${bd}/
}

case "$1" in
    bin)
        build_bin
    ;;

    web)
        build_web
    ;;
    *)
        build_bin
        build_web
    ;;
esac

