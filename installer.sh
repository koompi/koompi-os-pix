#!/bin/bash

work_dir=$(pwd)

echo "Downloading pix..."
curl -# https://pix.koompi.org/pix-v1.0.0.alpha.r6.g557e2ef-1-any.pkg.tar.zst -O
echo "Unstalling pix..."

[[ -f "/usr/bin/pix" ]] && sudo rm -rf /usr/bin/pix

sudo groupadd pix
sudo usermod -a -G pix $USER
sudo pacman -Sy --noconfirm glibc
sudo pacman -U --noconfirm pix-v1.0.0.alpha.r6.g557e2ef-1-any.pkg.tar.zst
sudo chgrp -R pix /var/lib/pix
sudo chmod -R 2775 /var/lib/pix

