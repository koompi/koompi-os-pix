#!/bin/bash
wd=$(pwd)
bd=${wd}/build

cd ${wd}
sh ${wd}/build.sh
cd ${bd}
sudo pacman -U ./pix-web*
[[ ! -f ${wd}/.env ]] && echo -e "\nENV file not found.\nPlease place a .env file in\n/var/www/pix-web/\n"

sudo systemctl enable pix-web.service
echo -e "To start service, run:"
echo -e "sudo systemctl start pix-web.service"