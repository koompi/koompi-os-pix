#!/bin/bash
wd=$(pwd)
targets=('pkgbuild-bin' 'pkgbuild-web')

mkdir -p "${wd}/docker-build"
docker build -t $USER/pkgbuild:latest .

for((i=0;i<${#targets[@]};i++)) {
    tg="${wd}/docker-build/${targets[$i]}"
    mkdir -p "${tg}"
    cp "${wd}/${targets[$i]}" "${tg}/PKGBUILD"
    cd "${tg}"
    docker run -v ${tg}:/work --rm $USER/pkgbuild:latest
    mv *tar.zst "${wd}/docker-build/"
}
