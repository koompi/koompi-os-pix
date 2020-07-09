# PIX

A simple package manager for helping the installation of software that is difficult to deal with to be as simple as one command and everything just works

**DEPLOYMENT**

Using source from GitHub:

```bash
git clone https://github.com/koompi/koompi-os-pix.git
cp koompi-os-pix
chmod +x pix.sh
sudo cp pix.sh /usr/bin/pix
```

Using KOOMPI software repository:

```bash
curl -S https://repo.koompi.org/script/pix.sh -o pix && chmod +x pix && sudo mv pix /usr/bin/
```

**USAGE**

Getting help:

```bash
pix h
```

Listing all packages:

```bash
pix l
```

Installing a package:

```bash
pix i package-name
```

Removing a package

```bash
pix r package-name
```

Updating all packages

```bash
pix u
```
