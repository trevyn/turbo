#!/bin/bash
# This will configure a new Ubuntu 20.04 server for development.
/bin/bash <<EOF
set -ex
apt update
apt dist-upgrade -y
curl --proto '=https' --tlsv1.2 -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
apt install -y nodejs build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
curl --proto '=https' --tlsv1.3 -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
fallocate -l 2g /mnt/2GiB.swap
chmod 600 /mnt/2GiB.swap
mkswap /mnt/2GiB.swap
swapon /mnt/2GiB.swap
echo '/mnt/2GiB.swap swap swap defaults 0 0' | sudo tee -a /etc/fstab
reboot
EOF

# eval $(ssh-agent) && ssh-add
# rsync -r --exclude 'target' --exclude '.git' --exclude 'node_modules' . root@test6.turbonet.to:turbo
# npm run start-info -- -- -d test6.turbonet.to -p 443
