# Terrarium Packages

## Adding it to your system

Make sure you have a “trusted GPG” directory for storing signing keys.

```bash
sudo mkdir -p /etc/apt/trusted.gpg.d/
```

Download the Terrariumn GPG public key.

```bash
curl -s "https://fujexo.github.io/rs-terrarium/terrarium.asc" \
    | sudo tee /etc/apt/trusted.gpg.d/terrarium.asc >/dev/null
```

Add the Terrarium repository to your local APT configuration, with autodetection of Ubuntu vs. Debian.

```txt
# /etc/apt/sources.list.d/terrarium.list
deb [arch=amd64,arm64 signed-by=/etc/apt/trusted.gpg.d/terrarium.asc] https://fujexo.github.io/rs-terrarium/debian trixie stable
```

Update your local package cache.

```bash
sudo apt update
```

## Listing Packages

Use `apt search` to list the packages available:

```bash
apt search terrarium
```
