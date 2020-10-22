#!/bin/bash

# Application version --------------------------------------------------------------
LOCAL_VERSION="0.2.0"
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
DOWNLOAD_DIR=$HOME/Downloads
INSTALLATION_DIR=$HOME/.pix
FIXES_DIR=/usr/share/org.koompi.pix/fixes

# Dependencies ---------------------------------------------------------------------

PIX_DEPS=( "rsync" "pv" );

MISSING_DEPS="";

# Functions ------------------------------------------------------------------------

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

    for((i=0;i<${#PIX_DEPS[@]};i++))
    do
        NUM=$(( i + 1 ))
        echo -ne "Checking Dependencies:${YELLOW} $(( $NUM * 100 / ${#PIX_DEPS[@]}))%${NORMAL}\033[0K\r";
        pacman -Qi --color always "${PIX_DEPS[$i]}" &> /dev/null;
        if [ $? -eq 0 ] ; then
            ((FOUND_NUM++))
        else
            ((MISSING_NUM++))
            MISSING_DEPS+=" ${PIX_DEPS[$i]}"
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
    pv $1 | tar xzf - -C $DOWNLOAD_DIR
}

version() {
    echo -e "${GREEN}PiX $LOCAL_VERSION${NORMAL}\nMade with ${RED}LOVE${NORMAL} by KOOMPI\nWebsite: ${BLUE}https://koompi.com${NORMAL}\nTelegram: ${BLUE}https://t.me/koompi${NORMAL}\nFacebook: ${BLUE}https://fb.com/koompi${NORMAL}"
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
    check_pkg_size=$(curl -sI $REPO_ADDR/pix/${app_to_install} | grep -i content-length | cut -d' ' -f2 | tr -dc '0-9');

    if [[ $exist == 1 ]]; then
        
        if [[ $check_pkg_exist == 200 ]]; then

            # check disk space before installation
            disk_size=$(df --block-size=M --output=avail /home/ | grep M | tr -d '[:space:]' | tr -dc '0-9')
            disk_size_in_kb="$(($disk_size * 1024 * 1024))"
            runtime_size="$(($check_pkg_size * 3))"

            if [[ $disk_size_in_kb -gt $runtime_size ]]; then
                echo -e "${GREEN}Dear ${USER^},${NORMAL}\n"
                echo -e "Please make sure you have a stable internet connection."
                echo -e "If the download error or incompleted, please run the installation again.\nThe download will continue from where it was.\n"
                echo -e "${GREEN}Dowloading ${1^^} ${NORMAL}";
                
                curl -# -C - $REPO_ADDR/pix/${app_to_install} -o $DOWNLOAD_DIR/${app_to_install}

                downloaded_data_size=$(wc -c $DOWNLOAD_DIR/${app_to_install} | cut -d' ' -f1)
                echo -e "Verifying downloaded package."

                if [[ ${downloaded_data_size} -ge ${check_pkg_size} ]]; then
                   
                    cd $DOWNLOAD_DIR
                    extract ${app_to_install}
                    cd $DOWNLOAD_DIR/${1}
                    echo -e "${GREEN}Preparing to install ${1^^} ${NORMAL}";
                    chmod +x $DOWNLOAD_DIR/${1}/installer.sh
                    $(which bash) $DOWNLOAD_DIR/${1}/installer.sh
                    echo -e "${GREEN}Installation Succeeded.${NORMAL}"

                else
                    echo -e "${RED}Downloading unsuccessful.${NORMAL}"
                    echo -e "${YELLOW}Please try again in a few minutes later.${NORMAL}"
                    echo -e "${GREEN}If the problem still persists please let us know at: https://t.me/koompi ${NORMAL}"
                    exit 1;
                fi
            else
                echo -e "${RED}Not enough disk space for installation.${NORMAL}"
                {
                    printf "%s \x1d %s \x1d\n" "Available space:" "${disk_size} MBs"
                    printf "%s \x1d %s \x1d\n" "Required space:" "$((runtime_size / 1024 / 1024)) MBs"
                } | column -t -s$'\x1d'
                exit 1;
            fi      

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
            curl -# -C - $REPO_ADDR/script/pix.sh -o $HOME/pix
            chmod +x $HOME/pix
            sudo mv $HOME/pix /usr/bin/
        fi
    else
        echo -e "${GREEN}You have the latest PiX installed.${NORMAL}"
    fi

    check_deps

    [[ -d $INSTALLATION_DIR ]] || mkdir -p $INSTALLATION_DIR

    INSTALLED_APPS=($(ls $INSTALLATION_DIR));
    LOCAL_APPS_VERSION=()
    SERVER_APP_VERSION=()
    NEW_VERSION_APP=()
    OLD_VERSION_APP=()

    # Get all installed apps to create app list with version

    if [[ ${#INSTALLED_APPS[@]} -eq 0 ]]; then 
        echo -e "There is no apps installed yet."
        exit 1;
    else
        # get all installed app and only if they have a versio file
        for((i=0;i<${#INSTALLED_APPS[@]};i++)) {
            if [[ -f "${INSTALLATION_DIR}/${INSTALLED_APPS[$i]}/version.sh" ]]; then
                VERSION=$($(which bash) ${INSTALLATION_DIR}/${INSTALLED_APPS[$i]}/version.sh)
                LOCAL_APPS_VERSION+=("${INSTALLED_APPS[$i]}_${VERSION}")
            fi
        }

        # Get all installed application list on the server

        for ((i=0;i<${#LOCAL_APPS_VERSION[@]};i++)) {

            LOCAL_FULL_NAME_APP="${LOCAL_APPS_VERSION[$i]}"

            for((j=0;j<${#APP_LIST[@]};j++)){

                # compare name first
                if [[ "${APP_LIST[$j]::(-16)}" ==  "${LOCAL_FULL_NAME_APP::(-9)}" ]]; then 
                    server_app=${APP_LIST[$j]::(-7)}
                    server_app_version=${server_app:(-8)}
                    local_app_version=${LOCAL_FULL_NAME_APP:(-8)}
                    # now compare version
                    if [[ $server_app_version -gt $local_app_version ]]; then 
                        OLD_VERSION_APP+=("$LOCAL_FULL_NAME_APP")
                        NEW_VERSION_APP+=("${APP_LIST[$j]::(-7)}")
                    fi

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
                
                printf "%d \x1d %s \x1d %s \x1d\n" "$((i + 1))" "${RED}${OLD_VERSION_APP[$i]}${NORMAL}" "${GREEN}${NEW_VERSION_APP[$i]}${NORMAL}";
            } | column -t -s$'\x1d'
            printf "${NORMAL}\n"

            read -p "Do you want to update now? [Y/n]" answer;
            if [[ ${answer,,} == "y" || ${answer,,} == "yes" ]]; then
                
                for((i=0;i<${#NEW_VERSION_APP[@]};i++)){
                    echo -e "Updating apps ${NEW_VERSION_APP[$i]}"
                    $(which bash) "/usr/bin/pix" install ${NEW_VERSION_APP[$i]::(-9)}
                }

            else
                echo -e "Updating aborted."
            fi
        else
            echo -e "${GREEN}No app updates available.${NORMAL}"
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
        echo -e "${GREEN}Tip${NORMAL}: Application name might be incorrect or not installed."
    fi
}

fix() {
    ARGS=($@)
    LIST_FIXES=($(ls $FIXES_DIR))
    if [[ ${ARGS[1]} == "" ]]; then 
        echo -e "Showing available fixes..."
        [[ ${#LIST_FIXES[@]} -gt 0 ]] && 
            for((i=0;i<${#LIST_FIXES[@]};i++))
            do
                if [[ i -eq 0 ]]; then
                    printf "${GREEN}\n"
                    printf "%s \x1d %s \x1d %s \x1d\n" "NO" "TOOLS" "USAGE";
                    printf "${NORMAL}\n"
                fi
                printf "%d \x1d %s \x1d %s \x1d\n" $((i + 1)) ${LIST_FIXES[$i]} "pix fix ${LIST_FIXES[$i]}";

            done | column -t -s$'\x1d'
        printf "\n"
    else
        VALID_FIX=()
        INVALID_FIX=()
        for((i=1;i<${#ARGS[@]};i++)) {
            [[ -f ${FIXES_DIR}/${ARGS[$i]} ]] && 
                VALID_FIX+=("${ARGS[$i]}") || 
                INVALID_FIX+=("${ARGS[$i]}");
        }

        if [[ ${#INVALID_FIX[@]} -gt 0 ]]; then 
            for((i=0;i<${#INVALID_FIX[@]};i++)) {
                [[ $i -eq 0 ]] && echo -e "\n${YELLOW}These fixes are incorrect or not available.${NORMAL}\n"
                echo -e "${RED}=> ${INVALID_FIX[$i]}${NORMAL}"
            }
            echo -e "\n${YELLOW}Please double check your spelling. Skipping...${NORMAL}\n"
        fi

        if [[ ${#VALID_FIX[@]} -gt 0 ]]; then 
            for((i=0;i<${#VALID_FIX[@]};i++)) {
                [[ $i -eq 0 ]] && echo -e "Executing maintenance...\n"
                echo -e "${GREEN}=> ${VALID_FIX[$i]}${NORMAL}"
                $(which bash) ${FIXES_DIR}/${VALID_FIX[$i]}
            }
            echo -e "\n${GREEN}Maintenance completed. Exiting...${NORMAL}\n"
        fi
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
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Checking fixes list" "pix f" "" "f = fix";
    printf "%s \x1d %s \x1d %s \x1d %s \x1d\n" "Checking fixes list" "pix f problem" "pix f panel" "f = fix";
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
    f | fix | -f | --fix)
        fix $@
    ;;
    v | version | -v | --version)
        version
    ;;
    *)
        help
        exit 1;
esac