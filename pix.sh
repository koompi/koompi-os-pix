#!/bin/bash

# Application version --------------------------------------------------------------
LOCAL_VERSION="0.0.2"
GET_VERSION=$(curl -s https://repo.koompi.org/script/pix.sh | grep LOCAL_VERSION=)
SERVER_VERSION="${GET_VERSION[@]:15:5}"

# Application list -----------------------------------------------------------------
RAW_DATABASE=($(grep -o '".*"' <(curl -s https://repo.koompi.org/pix/) | sed 's/"//g'))
APP_LIST=("${RAW_DATABASE[@]:1}")

# Color variables ------------------------------------------------------------------
RED=$(tput setaf 1)
GREEN=$(tput setaf 2)
YELLOW=$(tput setaf 3)
BLUE=$( tput setaf 4 )
NORMAL=$(tput sgr0)

# Working directories --------------------------------------------------------------
REPO_ADDR="https://repo.koompi.org"
WINE_NAME="wine-stable-4.0.2-1-x86_64.pkg.tar.xz"
THUMBNAILER="exe-thumbnailer-0.10.1-1-any.pkg.tar.xz"
DOWNLOAD_DIR=$HOME/Downloads
INSTALLATION_DIR=$HOME/.pix

# Dependencies ---------------------------------------------------------------------

WINE_DEPS=( "desktop-file-utils"  
            "fontconfig"  
            "freetype2"  
            "gettext"  
            "glu"  
            "lcms2"  
            "libpcap"  
            "libsm"  
            "libxcursor"  
            "libxdamage"  
            "libxi"  
            "libxml2"  
            "libxrandr"  
            "lib32-fontconfig"  
            "lib32-freetype2"  
            "lib32-gcc-libs"  
            "lib32-gettext"  
            "lib32-glu"  
            "lib32-lcms2"  
            "lib32-libpcap"  
            "lib32-libsm"  
            "lib32-libxcursor"  
            "lib32-libxdamage"  
            "lib32-libxi"  
            "lib32-libxml2"  
            "lib32-libxrandr"
            "alsa-lib" 
            "alsa-plugins" 
            "cups" 
            "dosbox" 
            "giflib" 
            "gnutls" 
            "gsm" 
            "gst-plugins-base-libs" 
            "libgphoto2" 
            "libjpeg-turbo" 
            "libldap" 
            "libpng" 
            "libpulse" 
            "libxcomposite" 
            "libxinerama" 
            "libxslt" 
            "mpg123" 
            "ncurses" 
            "ocl-icd" 
            "openal" 
            "samba" 
            "sane" 
            "sdl2" 
            "v4l-utils" 
            "vkd3d" 
            "vulkan-icd-loader" 
            "lib32-alsa-lib" 
            "lib32-alsa-plugins" 
            "lib32-giflib" 
            "lib32-gnutls" 
            "lib32-gst-plugins-base-libs" 
            "lib32-libjpeg-turbo" 
            "lib32-libldap" 
            "lib32-libpng" 
            "lib32-libpulse" 
            "lib32-libxcomposite" 
            "lib32-libxinerama" 
            "lib32-libxslt" 
            "lib32-mpg123" 
            "lib32-ncurses" 
            "lib32-ocl-icd" 
            "lib32-openal" 
            "lib32-sdl2" 
            "lib32-v4l-utils" 
            "lib32-vkd3d" 
            "lib32-vulkan-icd-loader" 
            "wine-mono" 
            "wine_gecko" 
            "winetricks" 
            "rsync" 
            "pv" );

MISSING_DEPS="";

# Functions -----------------------------------------------------------------------

semver_compare() {
    local version_a version_b pr_a pr_b
    # strip word "v" and extract first subset version (x.y.z from x.y.z-foo.n)
    version_a=$(echo "${1//v/}" | awk -F'-' '{print $1}')
    version_b=$(echo "${2//v/}" | awk -F'-' '{print $1}')
    
    if [ "$version_a" \= "$version_b" ]
    then
        # check for pre-release
        # extract pre-release (-foo.n from x.y.z-foo.n)
        pr_a=$(echo "$1" | awk -F'-' '{print $2}')
        pr_b=$(echo "$2" | awk -F'-' '{print $2}')
        
        ####
        # Return 0 when A is equal to B
        [ "$pr_a" \= "$pr_b" ] && echo 0 && return 0
        
        ####
        # Return 1
        
        # Case when A is not pre-release
        if [ -z "$pr_a" ]
        then
            echo 1 && return 0
        fi
        
        ####
        # Case when pre-release A exists and is greater than B's pre-release
        
        # extract numbers -rc.x --> x
        number_a=$(echo ${pr_a//[!0-9]/})
        number_b=$(echo ${pr_b//[!0-9]/})
        [ -z "${number_a}" ] && number_a=0
        [ -z "${number_b}" ] && number_b=0
        
        [ "$pr_a" \> "$pr_b" ] && [ -n "$pr_b" ] && [ "$number_a" -gt "$number_b" ] && echo 1 && return 0
        
        ####
        # Retrun -1 when A is lower than B
        echo -1 && return 0
    fi
    arr_version_a=(${version_a//./ })
    arr_version_b=(${version_b//./ })
    cursor=0
    # Iterate arrays from left to right and find the first difference
    while [ "$([ "${arr_version_a[$cursor]}" -eq "${arr_version_b[$cursor]}" ] && [ $cursor -lt ${#arr_version_a[@]} ] && echo true)" == true ]
    do
        cursor=$((cursor+1))
    done
    [ "${arr_version_a[$cursor]}" -gt "${arr_version_b[$cursor]}" ] && echo 1 || echo -1
}

check_deps() {
    MISSING_NUM=0
    FOUND_NUM=0

    if [ ! -d "$DOWNLOAD_DIR" ]; then 
        mkdir -p $DOWNLOAD_DIR; 
    fi
    sudo pacman -Sy
    echo -e "Checking runtime version:"
    pacman -Qi --color always wine-stable &> /dev/null;
    if [ $? -ne 0 ] ; then
        echo -e "${RED}[no] Runtime version is invalid.${NORMAL}\n"
        echo -e "${YELLOW}=> Downloading runtime software....${NORMAL}"
        curl -# $REPO_ADDR/packages/$WINE_NAME -o $DOWNLOAD_DIR/$WINE_NAME
        yes | sudo pacman -U $DOWNLOAD_DIR/packages/$WINE_NAME;
    else
        echo -e "${GREEN}[ok] Runtime version is valid.${NORMAL}\n"
    fi;

    pacman -Qi --color always exe-thumbnailer &> /dev/null;
    if [ $? -ne 0 ] ; then
        curl -# $REPO_ADDR/packages/$THUMBNAILER -o $DOWNLOAD_DIR/$THUMBNAILER
        yes | sudo pacman -U $DOWNLOAD_DIR/$THUMBNAILER;
    fi;

    for((i=0;i<${#WINE_DEPS[@]};i++))
    do
        NUM=$(( i + 1 ))
        echo -ne "Checking Dependencies:${YELLOW} $(( $NUM * 100 / ${#WINE_DEPS[@]}))%${NORMAL}\033[0K\r";
        pacman -Qi --color always "${WINE_DEPS[$i]}" &> /dev/null;
        if [ $? -eq 0 ] ; then
            ((FOUND_NUM++))
        else
            ((MISSING_NUM++))
            MISSING_DEPS+=" ${WINE_DEPS[$i]}"
        fi;
    done;

    if [[ $MISSING_NUM -gt 0 ]]; then
        echo -e ""
        echo -e "${RED}[no] Some Problems Found.${NORMAL}\n"
        echo -e "Dependencies Found: \t${GREEN}${FOUND_NUM}${NORMAL}"
        echo -e "Dependencies Missing: \t${RED}${MISSING_NUM}${NORMAL}"
        echo -e "Installing Missing Dependencies:\n"
        sudo pacman -S --noconfirm $MISSING_DEPS
    else
        echo -e ""
        echo -e "${GREEN}[ok] No Problems Found.${NORMAL}\n"
        echo -e "Dependencies Found: \t${GREEN}${FOUND_NUM}${NORMAL}"
        echo -e "Dependencies Missing: \t${RED}${MISSING_NUM}${NORMAL}\n"
    fi;
}

extract() {
    archive="$1"
    
    originalsize=$(file $archive | rev | cut -d' ' -f1 | rev)
    step=100
    blocks=$(echo "$originalsize / 512 / 20 / $step" | bc)
    
    tar -xz --checkpoint=$step --totals \
    --checkpoint-action="exec='p=\$(echo "\$TAR_CHECKPOINT/$blocks" | bc -l);printf \"Extracting package: %.4f%%\r\" \$p'" \
    -f $archive
}

version() {
    printf "${GREEN}PiX $LOCAL_VERSION${NORMAL}
Made with ${RED}LOVE${NORMAL} by KOOMPI
Website: ${BLUE}https://koompi.com${NORMAL}
Telegram: ${BLUE}https://t.me/koompi${NORMAL}
Facebook: ${BLUE}https://fb.com/koompi${NORMAL}
"
}


list() {
    
    for((i=0;i<${#APP_LIST[@]};i++))
    do
        if [[ i -eq 0 ]]; then
            printf "${GREEN}\n"
            printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "NO" "APPLICATION" "INSTALL" "REMOVE";
            printf "${NORMAL}\n"
        fi
        printf "%d \x1d %s \x1d %s \x1d %s \x1d\n" $((i + 1)) ${APP_LIST[$i]::(-16)} "pix -i ${APP_LIST[$i]::(-16)}" "pix -r ${APP_LIST[$i]::(-16)}";

    done | column -t -s$'\x1d'
    printf "\n"
    
}

install() {
    
    exist=0
    check_deps
    app_to_install=""
    for((i=0;i<${#APP_LIST[@]};i++))
    do
        if [[ $1 == ${APP_LIST[$i]::(-16)} ]]; then
            exist=1
            app_to_install=${APP_LIST[$i]}
            break;
        fi
    done;
    
    if [[ $exist == 0 ]]; then
        SIMILAR=$(list | grep -c $1);
        printf "\n"
        if [[ $SIMILAR -gt 0 ]]; then
            printf "${RED}\"$1\" is incorrect or unavailable for now\n";
            printf "Here are similar apps:\n${NORMAL}";
            list | grep $1
        fi
        
        if [[ $SIMILAR -eq 0 ]]; then
            echo -e "${RED}\"$1\" ${NORMAL}is incorrect or unavailable for now.";
            echo -e "Request this app at KOOMPI Telegram community at:${GREEN} \nhttps://t.me/koompi${NORMAL}";
            echo -e "Checkout all available apps below\n";
            list;
        fi
    fi

    check_pkg_exist=$(curl -I $REPO_ADDR/pix/${app_to_install} 2>/dev/null | head -n 1 | cut -d$' ' -f2)
    echo "${REPO_ADDR}/pix/${app_to_install}"

    if [[ $exist == 1 ]]; then

        if [[ $check_pkg_exist == 200 ]]; then
        
            echo -e "${GREEN}Dowloading ${1^^} ${NORMAL}";
            curl -# $REPO_ADDR/pix/${app_to_install} -o $DOWNLOAD_DIR/${app_to_install}
            cd $DOWNLOAD_DIR
            extract ${app_to_install}
            cd $DOWNLOAD_DIR/${1}
            echo -e "${GREEN}Preparing to install ${1^^} ${NORMAL}";
            chmod +x $DOWNLOAD_DIR/${1}/installer.sh
            $(which bash) $DOWNLOAD_DIR/${1}/installer.sh
            echo -e "${GREEN}Installation Succeeded.${NORMAL}"

        elif [[ $check_pkg_exist == 404 ]]; then

            echo -e "${RED}[404] Package not found";
        
        else

            echo -e "${RED}[${check_pkg_exist}] Error! Something went wrong on the server side.${NORMAL}"
            echo -e "${YELLOW}Please try again in a few minutes later.${NORMAL}"
            echo -e "${GREEN}If the problem still persists please let us know at: https://t.me/koompi"

        fi;
    fi
}

update() {
    
    RESULT=$(semver_compare ${SERVER_VERSION} ${LOCAL_VERSION})
    echo -e "Checking PiX version updates ..."
    if [[ $RESULT -gt 0 ]]; then
        echo -e "${GREEN}PiX Version ${SERVER_VERSION} is available for download.${NORMAL}"
        read -p "Do you want to update PiX now? [Y/n]" confirmation;
        if [[ ${confirmation,,} == "y" || ${confirmation,,} == "yes" || ${confirmation,,} == "" ]]; then
            curl -# $REPO_ADDR/script/pix.sh -o $HOME/pix
            chmod +x $HOME/pix
            sudo mv $HOME/pix /usr/bin/
        fi
    else
        echo -e "${GREEN}You have the latest PiX installed.${NORMAL}"
    fi
    check_deps

    INSTALLED_APPS=($(ls $INSTALLATION_DIR));
    LOCAL_APPS_VERSION=()
    SERVER_APP_VERSION=()
    NEW_VERSION_APP=()
    OLD_VERSION_APP=()
    # Get all installed apps to create app list with version

    for((i=0;i<${#INSTALLED_APPS[@]};i++)) {
        # echo "${INSTALLED_APPS[$i]}"
        VERSION=$($(which bash) ${INSTALLATION_DIR}/${INSTALLED_APPS[$i]}/version.sh)
        LOCAL_APPS_VERSION+=("${INSTALLED_APPS[$i]}_${VERSION}")
    }
    
    # Get all installed application list on the server

    for ((i=0;i<${#LOCAL_APPS_VERSION[@]};i++)) {
        for((j=0;j<${#APP_LIST[@]};j++)){
            SERVER_FULL="${APP_LIST[$j]::-7}"
            LOCAL_FULL="${LOCAL_APPS_VERSION[$i]}"

            if [[ ${SERVER_FULL:(-8)} -gt ${LOCAL_FULL:(-8)} ]]; then
                OLD_VERSION_APP+=("${LOCAL_APPS_VERSION[$i]}")
                NEW_VERSION_APP+=("${SERVER_FULL}")
            fi
        }
    }
    
    if [[ ${#NEW_VERSION_APP[@]} -gt 0 ]]; then
        for((i=0;i<${#NEW_VERSION_APP[@]};i++)) {
            if [[ i -eq 0 ]]; then
                printf "${NORMAL}\n"
                printf "%s\x1d %s\x1d %s\x1d" "NO" "${RED}LOCAL VERSION${NORMAL}" "${GREEN}NEW VERSION${NORMAL}";
                printf "${NORMAL}\n"
            fi
            
            printf "%d \x1d %s \x1d %s \x1d" "$((i + 1))" "${RED}${OLD_VERSION_APP}${NORMAL}" "${GREEN}${NEW_VERSION_APP[$i]}${NORMAL}";
        } | column -t -s$'\x1d'
        printf "${NORMAL}\n"

        read -p "Do you want to update now? [Y/n]" answer;
        if [[ ${answer,,} == "y" || ${answer,,} == "yes" ]]; then
            
            for((i=0;i<${#NEW_VERSION_APP[@]};i++)){
                echo -e "Updating apps ${NEW_VERSION_APP[$i]}"
                $(which bash) "$(pwd)/pix.sh" install ${NEW_VERSION_APP[$i]::(-9)}
            }

        else
            echo -e "Updating aborted."
        fi
    fi
}


remove() {
    if [ -d "${INSTALLATION_DIR}/${1}" ]; then 
        if [[ -f "${INSTALLATION_DIR}/${1}/uninstaller.sh" ]]; then
            read -p "${RED}Are you sure that you want to uninstall ${1^^}? [y/N]: ${NORMAL}" comfirmation
             
            if [[ ${comfirmation,,} == "y" || ${comfirmation,,} == "yes" ]]; then
                echo -e "Uninstalling ${1^^}"
                $(which bash) ${INSTALLATION_DIR}/${1}/uninstaller.sh
                echo -e "Uninstallation completed."
            else
                echo -e "Uninstallation aborted."
            fi
        else
            echo -e "${YELLOW}Warning! Uninstaller not found!"
            echo -e "The uninstallation process maybe not 100% cleaned.${NORMAL}"
            read -p "${RED}Are you sure that you want to uninstall ${1^^}? [y/N]: ${NORMAL}" comfirmation
             
            if [[ ${comfirmation,,} == "y" || ${comfirmation,,} == "yes" ]]; then
                echo -e "Uninstalling ${1^^}"
                rm -rf ${INSTALLATION_DIR}/${1}
                echo -e "Uninstallation completed."
            else
                echo -e "Uninstallation aborted."
            fi
        fi
    else
        echo -e "${RED}Application not found!${NORMAL}"
        echo -e "${GREEN}Tip${NORMAL}: Applicatio name might be incorrect or not installed."
    fi
}

help(){
    {
    echo -e "${GREEN}"
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Features" "Commands" "Examples" "Explanations";
    echo -e "${NORMAL}"
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Checking available apps:" "pix l" "" "l = list";
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Installing appplication:" "pix i app-name" "pix i ms-office-2013" "i = install";
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Removing application:" "pix r app-name" "pix r ms-office-2013" "r = remove";
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Updating application:" "pix u" "" "u = update";
    } | column -t -s$'\x1d'
    echo -e ""
} 

case "$1" in
    l | list | -l | --list)
        list
    ;;
    i | install | -i | --install)
        install $2
    ;;
    r | remove | -r | --remove)
        remove $2
    ;;
    u | update | -u | --update)
        update $2
    ;;
    v | version | -v | --version)
        version
    ;;
    *)
        help
        exit 1
esac