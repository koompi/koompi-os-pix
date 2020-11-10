FROM archlinux

WORKDIR /var/www/pix.koompi.org

COPY . .

RUN pacman -Syu --noconfirm rustup glibc base-devel
RUN rustup default stable
RUN cargo build --release -p backend

ENV PORT=4444

EXPOSE 4444

CMD [ "cargo", "run", "--release", "-p", "backend"]