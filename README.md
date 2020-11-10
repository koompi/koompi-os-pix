# PIX

A simple package manager for helping the installation of software that is difficult to deal with to be as simple as one command and everything just works

**Installation**

Using official website

```bash
curl -Ssf https://pix.koompi.org/installer.sh | sh
```

Installing from source.

```bash
git clone https://github.com/koompi/os-pix.git
cd os-pix
chmod +x build.sh
./build.sh
cd build
sudo pacman -U ./pix-version-rel-arch.pkg.tar.zst
```

**USAGE**

Getting help:

```bash
pix -h
```

Listing all packages:

```bash
pix -l
```

Installing multiple packages:

```bash
pix -i package-name package-name package-name 
```

Removing a package

```bash
pix -r package-name package-name package-name
```

Updating all packages

```bash
pix -u
```
