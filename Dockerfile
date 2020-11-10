# This Dockerfile builds an image that can be used to build ArchLinux packages

FROM archlinux

RUN pacman-key --init 
RUN pacman -Sy --noconfirm archlinux-keyring 
RUN pacman -Syu --noconfirm base-devel git boost rsync
RUN pacman-db-upgrade 
RUN update-ca-trust 
RUN pacman -Scc --noconfirm
COPY sudoers /etc/sudoers
RUN chown -c root:root /etc/sudoers && \
    chmod -c 0440 /etc/sudoers && \
    mkdir -p /work
RUN groupadd pix
VOLUME /work
WORKDIR /work

ENV USER_UID 1000
CMD useradd -u ${USER_UID} -d /home/makepkg -m makepkg && \
           chown -R makepkg /work /home/makepkg && \
           pacman -Syu --noconfirm && \
           pacman-db-upgrade && \
           pacman -Scc --noconfirm && \
           su -l makepkg -c "cd /work && makepkg --noconfirm -s --skipinteg"
