~/clones/swarm-runtime $ # Move the key from your Android Downloads to your Termux home                                     mv ~/storage/downloads/ssh-key-2026-03-09.key ~/.
# Lock down the permissions (SSH requires this)               chmod 400 ~/ssh-key-2026-03-09.key                            ~/clones/swarm-runtime $ ssh -i ~/ssh-key-2026-03-09.key ubuntu@145.241.192.79                                              The authenticity of host '145.241.192.79 (145.241.192.79)' can't be established.                                            ED25519 key fingerprint is: SHA256:eaUs4f9i1ASAym3uwGDBsvBV6dDflUrEU4MMvFDqgGY                                              This key is not known by any other names.                     Are you sure you want to continue connecting (yes/no/[fingerprint])? yes                                                    Warning: Permanently added '145.241.192.79' (ED25519) to the list of known hosts.                                           Welcome to Ubuntu 24.04.4 LTS (GNU/Linux 6.17.0-1007-oracle x86_64)                                       
 * Documentation:  https://help.ubuntu.com
 * Management:     https://landscape.canonical.com             * Support:        https://ubuntu.com/pro
 System information as of Mon Mar  9 10:30:09 UTC 2026
  System load:  0.02              Processes:             119
  Usage of /:   4.1% of 47.39GB   Users logged in:       0      Memory usage: 25%               IPv4 address for ens3: 10.0.2.49                                                            Swap usage:   0%

Expanded Security Maintenance for Applications is not enabled.

0 updates can be applied immediately.

Enable ESM Apps to receive additional future security updates.
See https://ubuntu.com/esm or run: sudo pro status


The list of available updates is more than a week old.
To check for new updates run: sudo apt update


The programs included with the Ubuntu system are free software;
the exact distribution terms for each program are described in the
individual files in /usr/share/doc/*/copyright.

Ubuntu comes with ABSOLUTELY NO WARRANTY, to the extent permitted by
applicable law.

To run a command as administrator (user "root"), use "sudo <command>".
See "man sudo_root" for details.

ubuntu@swarm-vnic:~$ # 1. Install system dependencies
sudo apt update && sudo apt install -y build-essential git pkg-config libssl-dev

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# 3. Clone your Swarm Runtime
git clone https://github.com/TangoSplicer/Swarm-Runtime.git
cd Swarm-Runtime

# 4. IGNITION! Start the Gateway!
cargo run --bin swarm-node -- gateway --port 3000
Get:1 http://security.ubuntu.com/ubuntu noble-security InRelease [126 kB]
Hit:2 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble InRelease
Get:3 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates InRelease [126 kB]
Get:4 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports InRelease [126 kB]
Get:5 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/universe amd64 Packages [15.0 MB]
Get:6 http://security.ubuntu.com/ubuntu noble-security/main amd64 Packages [1504 kB]
Get:7 http://security.ubuntu.com/ubuntu noble-security/main Translation-en [241 kB]
Get:8 http://security.ubuntu.com/ubuntu noble-security/main amd64 Components [21.5 kB]
Get:9 http://security.ubuntu.com/ubuntu noble-security/main amd64 c-n-f Metadata [10.1 kB]
Get:10 http://security.ubuntu.com/ubuntu noble-security/universe amd64 Packages [975 kB]
Get:11 http://security.ubuntu.com/ubuntu noble-security/universe Translation-en [218 kB]
Get:12 http://security.ubuntu.com/ubuntu noble-security/universe amd64 Components [74.2 kB]
Get:13 http://security.ubuntu.com/ubuntu noble-security/universe amd64 c-n-f Metadata [20.6 kB]
Get:14 http://security.ubuntu.com/ubuntu noble-security/restricted amd64 Packages [2599 kB]
Get:15 http://security.ubuntu.com/ubuntu noble-security/restricted Translation-en [600 kB]
Get:16 http://security.ubuntu.com/ubuntu noble-security/restricted amd64 Components [212 B]
Get:17 http://security.ubuntu.com/ubuntu noble-security/multiverse amd64 Packages [28.8 kB]
Get:18 http://security.ubuntu.com/ubuntu noble-security/multiverse Translation-en [6732 B]
Get:19 http://security.ubuntu.com/ubuntu noble-security/multiverse amd64 Components [212 B]
Get:20 http://security.ubuntu.com/ubuntu noble-security/multiverse amd64 c-n-f Metadata [396 B]
Get:21 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/universe Translation-en [5982 kB]
Get:22 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/universe amd64 Components [3871 kB]
Get:23 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/universe amd64 c-n-f Metadata [301 kB]
Get:24 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/multiverse amd64 Packages [269 kB]
Get:25 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/multiverse Translation-en [118 kB]
Get:26 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/multiverse amd64 Components [35.0 kB]
Get:27 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/multiverse amd64 c-n-f Metadata [8328 B]
Get:28 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 Packages [1806 kB]
Get:29 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main Translation-en [332 kB]
Get:30 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 Components [177 kB]
Get:31 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 c-n-f Metadata [16.7 kB]
Get:32 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/universe amd64 Packages [1564 kB]
Get:33 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/universe Translation-en [318 kB]
Get:34 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/universe amd64 Components [386 kB]
Get:35 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/universe amd64 c-n-f Metadata [32.9 kB]
Get:36 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/restricted amd64 Packages [2747 kB]
Get:37 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/restricted Translation-en [632 kB]
Get:38 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/restricted amd64 Components [212 B]
Get:39 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/multiverse amd64 Packages [32.1 kB]
Get:40 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/multiverse Translation-en [7044 B]
Get:41 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/multiverse amd64 Components [940 B]
Get:42 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/multiverse amd64 c-n-f Metadata [496 B]
Get:43 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/main amd64 Packages [40.4 kB]
Get:44 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/main Translation-en [9208 B]
Get:45 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/main amd64 Components [7284 B]
Get:46 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/main amd64 c-n-f Metadata [368 B]
Get:47 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/universe amd64 Packages [29.5 kB]
Get:48 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/universe Translation-en [17.9 kB]
Get:49 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/universe amd64 Components [10.5 kB]
Get:50 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/universe amd64 c-n-f Metadata [1444 B]
Get:51 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/restricted amd64 Components [216 B]
Get:52 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/restricted amd64 c-n-f Metadata [116 B]
Get:53 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/multiverse amd64 Components [212 B]
Get:54 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-backports/multiverse amd64 c-n-f Metadata [116 B]
Fetched 40.5 MB in 25s (1650 kB/s)
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
8 packages can be upgraded. Run 'apt list --upgradable' to see them.
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
git is already the newest version (1:2.43.0-1ubuntu7.3).
git set to manually installed.
The following additional packages will be installed:
  binutils binutils-common binutils-x86-64-linux-gnu bzip2
  cpp cpp-13 cpp-13-x86-64-linux-gnu cpp-x86-64-linux-gnu
  dpkg-dev fakeroot fontconfig-config fonts-dejavu-core
  fonts-dejavu-mono g++ g++-13 g++-13-x86-64-linux-gnu
  g++-x86-64-linux-gnu gcc gcc-13 gcc-13-base
  gcc-13-x86-64-linux-gnu gcc-x86-64-linux-gnu
  libalgorithm-diff-perl libalgorithm-diff-xs-perl
  libalgorithm-merge-perl libaom3 libasan8 libatomic1
  libbinutils libc-dev-bin libc-devtools libc6-dev libcc1-0
  libcrypt-dev libctf-nobfd0 libctf0 libde265-0 libdeflate0
  libdpkg-perl libfakeroot libfile-fcntllock-perl
  libfontconfig1 libgcc-13-dev libgd3 libgomp1 libgprofng0
  libheif-plugin-aomdec libheif-plugin-aomenc
  libheif-plugin-libde265 libheif1 libhwasan0 libisl23
  libitm1 libjbig0 libjpeg-turbo8 libjpeg8 liblerc4 liblsan0
  libmpc3 libpkgconf3 libquadmath0 libsframe1 libsharpyuv0
  libstdc++-13-dev libtiff6 libtsan2 libubsan1 libwebp7
  libxpm4 linux-libc-dev lto-disabled-list make manpages-dev
  pkgconf pkgconf-bin rpcsvc-proto
Suggested packages:
  binutils-doc gprofng-gui bzip2-doc cpp-doc gcc-13-locales
  cpp-13-doc debian-keyring g++-multilib g++-13-multilib
  gcc-13-doc gcc-multilib autoconf automake libtool flex
  bison gdb gcc-doc gcc-13-multilib gdb-x86-64-linux-gnu
  glibc-doc bzr libgd-tools libheif-plugin-x265
  libheif-plugin-ffmpegdec libheif-plugin-jpegdec
  libheif-plugin-jpegenc libheif-plugin-j2kdec
  libheif-plugin-j2kenc libheif-plugin-rav1e
  libheif-plugin-svtenc libssl-doc libstdc++-13-doc make-doc
The following NEW packages will be installed:
  binutils binutils-common binutils-x86-64-linux-gnu
  build-essential bzip2 cpp cpp-13 cpp-13-x86-64-linux-gnu
  cpp-x86-64-linux-gnu dpkg-dev fakeroot fontconfig-config
  fonts-dejavu-core fonts-dejavu-mono g++ g++-13
  g++-13-x86-64-linux-gnu g++-x86-64-linux-gnu gcc gcc-13
  gcc-13-base gcc-13-x86-64-linux-gnu gcc-x86-64-linux-gnu
  libalgorithm-diff-perl libalgorithm-diff-xs-perl
  libalgorithm-merge-perl libaom3 libasan8 libatomic1
  libbinutils libc-dev-bin libc-devtools libc6-dev libcc1-0
  libcrypt-dev libctf-nobfd0 libctf0 libde265-0 libdeflate0
  libdpkg-perl libfakeroot libfile-fcntllock-perl
  libfontconfig1 libgcc-13-dev libgd3 libgomp1 libgprofng0
  libheif-plugin-aomdec libheif-plugin-aomenc
  libheif-plugin-libde265 libheif1 libhwasan0 libisl23
  libitm1 libjbig0 libjpeg-turbo8 libjpeg8 liblerc4 liblsan0
  libmpc3 libpkgconf3 libquadmath0 libsframe1 libsharpyuv0
  libssl-dev libstdc++-13-dev libtiff6 libtsan2 libubsan1
  libwebp7 libxpm4 linux-libc-dev lto-disabled-list make
  manpages-dev pkg-config pkgconf pkgconf-bin rpcsvc-proto
0 upgraded, 79 newly installed, 0 to remove and 8 not upgraded.
Need to get 80.7 MB of archives.
After this operation, 282 MB of additional disk space will be used.
Get:1 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 binutils-common amd64 2.42-4ubuntu2.8 [240 kB]
Get:2 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libsframe1 amd64 2.42-4ubuntu2.8 [15.6 kB]
Get:3 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libbinutils amd64 2.42-4ubuntu2.8 [576 kB]
Get:4 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libctf-nobfd0 amd64 2.42-4ubuntu2.8 [97.9 kB]
Get:5 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libctf0 amd64 2.42-4ubuntu2.8 [94.5 kB]
Get:6 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libgprofng0 amd64 2.42-4ubuntu2.8 [849 kB]
Get:7 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 binutils-x86-64-linux-gnu amd64 2.42-4ubuntu2.8 [2463 kB]
Get:8 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 binutils amd64 2.42-4ubuntu2.8 [18.1 kB]
Get:9 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libc-dev-bin amd64 2.39-0ubuntu8.7 [20.4 kB]
Get:10 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 linux-libc-dev amd64 6.8.0-101.101 [2042 kB]
Get:11 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libcrypt-dev amd64 1:4.4.36-4build1 [112 kB]
Get:12 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 rpcsvc-proto amd64 1.4.2-0ubuntu7 [67.4 kB]
Get:13 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libc6-dev amd64 2.39-0ubuntu8.7 [2124 kB]
Get:14 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 gcc-13-base amd64 13.3.0-6ubuntu2~24.04.1 [51.6 kB]
Get:15 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libisl23 amd64 0.26-3build1.1 [680 kB]
Get:16 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libmpc3 amd64 1.3.1-1build1.1 [54.6 kB]
Get:17 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 cpp-13-x86-64-linux-gnu amd64 13.3.0-6ubuntu2~24.04.1 [10.7 MB]
Get:18 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 cpp-13 amd64 13.3.0-6ubuntu2~24.04.1 [1042 B]
Get:19 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 cpp-x86-64-linux-gnu amd64 4:13.2.0-7ubuntu1 [5326 B]
Get:20 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 cpp amd64 4:13.2.0-7ubuntu1 [22.4 kB]
Get:21 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libcc1-0 amd64 14.2.0-4ubuntu2~24.04.1 [48.0 kB]
Get:22 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libgomp1 amd64 14.2.0-4ubuntu2~24.04.1 [148 kB]
Get:23 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libitm1 amd64 14.2.0-4ubuntu2~24.04.1 [29.7 kB]
Get:24 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libatomic1 amd64 14.2.0-4ubuntu2~24.04.1 [10.5 kB]
Get:25 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libasan8 amd64 14.2.0-4ubuntu2~24.04.1 [3027 kB]
Get:26 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 liblsan0 amd64 14.2.0-4ubuntu2~24.04.1 [1322 kB]
Get:27 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libtsan2 amd64 14.2.0-4ubuntu2~24.04.1 [2772 kB]
Get:28 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libubsan1 amd64 14.2.0-4ubuntu2~24.04.1 [1184 kB]
Get:29 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libhwasan0 amd64 14.2.0-4ubuntu2~24.04.1 [1641 kB]
Get:30 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libquadmath0 amd64 14.2.0-4ubuntu2~24.04.1 [153 kB]
Get:31 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libgcc-13-dev amd64 13.3.0-6ubuntu2~24.04.1 [2681 kB]
Get:32 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 gcc-13-x86-64-linux-gnu amd64 13.3.0-6ubuntu2~24.04.1 [21.1 MB]
Get:33 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 gcc-13 amd64 13.3.0-6ubuntu2~24.04.1 [494 kB]
Get:34 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 gcc-x86-64-linux-gnu amd64 4:13.2.0-7ubuntu1 [1212 B]
Get:35 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 gcc amd64 4:13.2.0-7ubuntu1 [5018 B]
Get:36 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libstdc++-13-dev amd64 13.3.0-6ubuntu2~24.04.1 [2420 kB]
Get:37 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 g++-13-x86-64-linux-gnu amd64 13.3.0-6ubuntu2~24.04.1 [12.2 MB]
Get:38 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 g++-13 amd64 13.3.0-6ubuntu2~24.04.1 [16.0 kB]
Get:39 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 g++-x86-64-linux-gnu amd64 4:13.2.0-7ubuntu1 [964 B]
Get:40 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 g++ amd64 4:13.2.0-7ubuntu1 [1100 B]
Get:41 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 make amd64 4.3-4.1build2 [180 kB]
Get:42 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libdpkg-perl all 1.22.6ubuntu6.5 [269 kB]
Get:43 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 bzip2 amd64 1.0.8-5.1build0.1 [34.5 kB]
Get:44 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 lto-disabled-list all 47 [12.4 kB]
Get:45 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 dpkg-dev all 1.22.6ubuntu6.5 [1074 kB]
Get:46 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 build-essential amd64 12.10ubuntu1 [4928 B]
Get:47 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libfakeroot amd64 1.33-1 [32.4 kB]
Get:48 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 fakeroot amd64 1.33-1 [67.2 kB]
Get:49 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 fonts-dejavu-mono all 2.37-8 [502 kB]
Get:50 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 fonts-dejavu-core all 2.37-8 [835 kB]
Get:51 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 fontconfig-config amd64 2.15.0-1.1ubuntu2 [37.3 kB]
Get:52 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libalgorithm-diff-perl all 1.201-1 [41.8 kB]
Get:53 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libalgorithm-diff-xs-perl amd64 0.04-8build3 [11.2 kB]
Get:54 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libalgorithm-merge-perl all 0.08-5 [11.4 kB]
Get:55 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libaom3 amd64 3.8.2-2ubuntu0.1 [1941 kB]
Get:56 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libfontconfig1 amd64 2.15.0-1.1ubuntu2 [139 kB]
Get:57 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libsharpyuv0 amd64 1.3.2-0.4build3 [15.8 kB]
Get:58 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libheif-plugin-aomdec amd64 1.17.6-1ubuntu4.2 [10.6 kB]
Get:59 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libde265-0 amd64 1.0.15-1build3 [166 kB]
Get:60 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libheif-plugin-libde265 amd64 1.17.6-1ubuntu4.2 [8174 B]
Get:61 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libheif1 amd64 1.17.6-1ubuntu4.2 [276 kB]
Get:62 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libjpeg-turbo8 amd64 2.1.5-2ubuntu2 [150 kB]
Get:63 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libjpeg8 amd64 8c-2ubuntu11 [2148 B]
Get:64 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libdeflate0 amd64 1.19-1build1.1 [43.9 kB]
Get:65 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libjbig0 amd64 2.1-6.1ubuntu2 [29.7 kB]
Get:66 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 liblerc4 amd64 4.0.0+ds-4ubuntu2 [179 kB]
Get:67 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libwebp7 amd64 1.3.2-0.4build3 [230 kB]
Get:68 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libtiff6 amd64 4.5.1+git230720-4ubuntu2.4 [199 kB]
Get:69 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libxpm4 amd64 1:3.5.17-1build2 [36.5 kB]
Get:70 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libgd3 amd64 2.3.3-9ubuntu5 [128 kB]
Get:71 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libc-devtools amd64 2.39-0ubuntu8.7 [29.3 kB]
Get:72 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libfile-fcntllock-perl amd64 0.22-4ubuntu5 [30.7 kB]
Get:73 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libheif-plugin-aomenc amd64 1.17.6-1ubuntu4.2 [14.7 kB]
Get:74 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 libpkgconf3 amd64 1.8.1-2build1 [30.7 kB]
Get:75 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble-updates/main amd64 libssl-dev amd64 3.0.13-0ubuntu3.7 [2407 kB]
Get:76 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 manpages-dev all 6.7-2 [2013 kB]
Get:77 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 pkgconf-bin amd64 1.8.1-2build1 [20.7 kB]
Get:78 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 pkgconf amd64 1.8.1-2build1 [16.8 kB]
Get:79 http://uk-london-1-ad-3.clouds.archive.ubuntu.com/ubuntu noble/main amd64 pkg-config amd64 1.8.1-2build1 [7264 B]
Fetched 80.7 MB in 13s (6261 kB/s)
Extracting templates from packages: 100%
Selecting previously unselected package binutils-common:amd64.
(Reading database ... 79132 files and directories currently installed.)
Preparing to unpack .../00-binutils-common_2.42-4ubuntu2.8_amd64.deb ...
Unpacking binutils-common:amd64 (2.42-4ubuntu2.8) ...
Selecting previously unselected package libsframe1:amd64.
Preparing to unpack .../01-libsframe1_2.42-4ubuntu2.8_amd64.deb ...
Unpacking libsframe1:amd64 (2.42-4ubuntu2.8) ...
Selecting previously unselected package libbinutils:amd64.
Preparing to unpack .../02-libbinutils_2.42-4ubuntu2.8_amd64.deb ...
Unpacking libbinutils:amd64 (2.42-4ubuntu2.8) ...
Selecting previously unselected package libctf-nobfd0:amd64.
Preparing to unpack .../03-libctf-nobfd0_2.42-4ubuntu2.8_amd64.deb ...
Unpacking libctf-nobfd0:amd64 (2.42-4ubuntu2.8) ...
Selecting previously unselected package libctf0:amd64.
Preparing to unpack .../04-libctf0_2.42-4ubuntu2.8_amd64.deb ...
Unpacking libctf0:amd64 (2.42-4ubuntu2.8) ...
Selecting previously unselected package libgprofng0:amd64.
Preparing to unpack .../05-libgprofng0_2.42-4ubuntu2.8_amd64.deb ...
Unpacking libgprofng0:amd64 (2.42-4ubuntu2.8) ...
Selecting previously unselected package binutils-x86-64-linux-gnu.
Preparing to unpack .../06-binutils-x86-64-linux-gnu_2.42-4ubuntu2.8_amd64.deb ...
Unpacking binutils-x86-64-linux-gnu (2.42-4ubuntu2.8) ...
Selecting previously unselected package binutils.
Preparing to unpack .../07-binutils_2.42-4ubuntu2.8_amd64.deb ...
Unpacking binutils (2.42-4ubuntu2.8) ...
Selecting previously unselected package libc-dev-bin.
Preparing to unpack .../08-libc-dev-bin_2.39-0ubuntu8.7_amd64.deb ...
Unpacking libc-dev-bin (2.39-0ubuntu8.7) ...
Selecting previously unselected package linux-libc-dev:amd64.
Preparing to unpack .../09-linux-libc-dev_6.8.0-101.101_amd64.deb ...
Unpacking linux-libc-dev:amd64 (6.8.0-101.101) ...
Selecting previously unselected package libcrypt-dev:amd64.
Preparing to unpack .../10-libcrypt-dev_1%3a4.4.36-4build1_amd64.deb ...
Unpacking libcrypt-dev:amd64 (1:4.4.36-4build1) ...
Selecting previously unselected package rpcsvc-proto.
Preparing to unpack .../11-rpcsvc-proto_1.4.2-0ubuntu7_amd64.deb ...
Unpacking rpcsvc-proto (1.4.2-0ubuntu7) ...
Selecting previously unselected package libc6-dev:amd64.
Preparing to unpack .../12-libc6-dev_2.39-0ubuntu8.7_amd64.deb ...
Unpacking libc6-dev:amd64 (2.39-0ubuntu8.7) ...
Selecting previously unselected package gcc-13-base:amd64.
Preparing to unpack .../13-gcc-13-base_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking gcc-13-base:amd64 (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package libisl23:amd64.
Preparing to unpack .../14-libisl23_0.26-3build1.1_amd64.deb ...
Unpacking libisl23:amd64 (0.26-3build1.1) ...
Selecting previously unselected package libmpc3:amd64.
Preparing to unpack .../15-libmpc3_1.3.1-1build1.1_amd64.deb ...
Unpacking libmpc3:amd64 (1.3.1-1build1.1) ...
Selecting previously unselected package cpp-13-x86-64-linux-gnu.
Preparing to unpack .../16-cpp-13-x86-64-linux-gnu_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking cpp-13-x86-64-linux-gnu (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package cpp-13.
Preparing to unpack .../17-cpp-13_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking cpp-13 (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package cpp-x86-64-linux-gnu.
Preparing to unpack .../18-cpp-x86-64-linux-gnu_4%3a13.2.0-7ubuntu1_amd64.deb ...
Unpacking cpp-x86-64-linux-gnu (4:13.2.0-7ubuntu1) ...
Selecting previously unselected package cpp.
Preparing to unpack .../19-cpp_4%3a13.2.0-7ubuntu1_amd64.deb ...
Unpacking cpp (4:13.2.0-7ubuntu1) ...
Selecting previously unselected package libcc1-0:amd64.
Preparing to unpack .../20-libcc1-0_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libcc1-0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libgomp1:amd64.
Preparing to unpack .../21-libgomp1_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libgomp1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libitm1:amd64.
Preparing to unpack .../22-libitm1_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libitm1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libatomic1:amd64.
Preparing to unpack .../23-libatomic1_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libatomic1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libasan8:amd64.
Preparing to unpack .../24-libasan8_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libasan8:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package liblsan0:amd64.
Preparing to unpack .../25-liblsan0_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking liblsan0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libtsan2:amd64.
Preparing to unpack .../26-libtsan2_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libtsan2:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libubsan1:amd64.
Preparing to unpack .../27-libubsan1_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libubsan1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libhwasan0:amd64.
Preparing to unpack .../28-libhwasan0_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libhwasan0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libquadmath0:amd64.
Preparing to unpack .../29-libquadmath0_14.2.0-4ubuntu2~24.04.1_amd64.deb ...
Unpacking libquadmath0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Selecting previously unselected package libgcc-13-dev:amd64.
Preparing to unpack .../30-libgcc-13-dev_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking libgcc-13-dev:amd64 (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package gcc-13-x86-64-linux-gnu.
Preparing to unpack .../31-gcc-13-x86-64-linux-gnu_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking gcc-13-x86-64-linux-gnu (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package gcc-13.
Preparing to unpack .../32-gcc-13_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking gcc-13 (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package gcc-x86-64-linux-gnu.
Preparing to unpack .../33-gcc-x86-64-linux-gnu_4%3a13.2.0-7ubuntu1_amd64.deb ...
Unpacking gcc-x86-64-linux-gnu (4:13.2.0-7ubuntu1) ...
Selecting previously unselected package gcc.
Preparing to unpack .../34-gcc_4%3a13.2.0-7ubuntu1_amd64.deb ...
Unpacking gcc (4:13.2.0-7ubuntu1) ...
Selecting previously unselected package libstdc++-13-dev:amd64.
Preparing to unpack .../35-libstdc++-13-dev_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking libstdc++-13-dev:amd64 (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package g++-13-x86-64-linux-gnu.
Preparing to unpack .../36-g++-13-x86-64-linux-gnu_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking g++-13-x86-64-linux-gnu (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package g++-13.
Preparing to unpack .../37-g++-13_13.3.0-6ubuntu2~24.04.1_amd64.deb ...
Unpacking g++-13 (13.3.0-6ubuntu2~24.04.1) ...
Selecting previously unselected package g++-x86-64-linux-gnu.
Preparing to unpack .../38-g++-x86-64-linux-gnu_4%3a13.2.0-7ubuntu1_amd64.deb ...
Unpacking g++-x86-64-linux-gnu (4:13.2.0-7ubuntu1) ...
Selecting previously unselected package g++.
Preparing to unpack .../39-g++_4%3a13.2.0-7ubuntu1_amd64.deb ...
Unpacking g++ (4:13.2.0-7ubuntu1) ...
Selecting previously unselected package make.
Preparing to unpack .../40-make_4.3-4.1build2_amd64.deb ...
Unpacking make (4.3-4.1build2) ...
Selecting previously unselected package libdpkg-perl.
Preparing to unpack .../41-libdpkg-perl_1.22.6ubuntu6.5_all.deb ...
Unpacking libdpkg-perl (1.22.6ubuntu6.5) ...
Selecting previously unselected package bzip2.
Preparing to unpack .../42-bzip2_1.0.8-5.1build0.1_amd64.deb ...
Unpacking bzip2 (1.0.8-5.1build0.1) ...
Selecting previously unselected package lto-disabled-list.
Preparing to unpack .../43-lto-disabled-list_47_all.deb ...
Unpacking lto-disabled-list (47) ...
Selecting previously unselected package dpkg-dev.
Preparing to unpack .../44-dpkg-dev_1.22.6ubuntu6.5_all.deb ...
Unpacking dpkg-dev (1.22.6ubuntu6.5) ...
Selecting previously unselected package build-essential.
Preparing to unpack .../45-build-essential_12.10ubuntu1_amd64.deb ...
Unpacking build-essential (12.10ubuntu1) ...
Selecting previously unselected package libfakeroot:amd64.
Preparing to unpack .../46-libfakeroot_1.33-1_amd64.deb ...
Unpacking libfakeroot:amd64 (1.33-1) ...
Selecting previously unselected package fakeroot.
Preparing to unpack .../47-fakeroot_1.33-1_amd64.deb ...
Unpacking fakeroot (1.33-1) ...
Selecting previously unselected package fonts-dejavu-mono.
Preparing to unpack .../48-fonts-dejavu-mono_2.37-8_all.deb ...
Unpacking fonts-dejavu-mono (2.37-8) ...
Selecting previously unselected package fonts-dejavu-core.
Preparing to unpack .../49-fonts-dejavu-core_2.37-8_all.deb ...
Unpacking fonts-dejavu-core (2.37-8) ...
Selecting previously unselected package fontconfig-config.
Preparing to unpack .../50-fontconfig-config_2.15.0-1.1ubuntu2_amd64.deb ...
Unpacking fontconfig-config (2.15.0-1.1ubuntu2) ...
Selecting previously unselected package libalgorithm-diff-perl.
Preparing to unpack .../51-libalgorithm-diff-perl_1.201-1_all.deb ...
Unpacking libalgorithm-diff-perl (1.201-1) ...
Selecting previously unselected package libalgorithm-diff-xs-perl:amd64.
Preparing to unpack .../52-libalgorithm-diff-xs-perl_0.04-8build3_amd64.deb ...
Unpacking libalgorithm-diff-xs-perl:amd64 (0.04-8build3) ...
Selecting previously unselected package libalgorithm-merge-perl.
Preparing to unpack .../53-libalgorithm-merge-perl_0.08-5_all.deb ...
Unpacking libalgorithm-merge-perl (0.08-5) ...
Selecting previously unselected package libaom3:amd64.
Preparing to unpack .../54-libaom3_3.8.2-2ubuntu0.1_amd64.deb ...
Unpacking libaom3:amd64 (3.8.2-2ubuntu0.1) ...
Selecting previously unselected package libfontconfig1:amd64.
Preparing to unpack .../55-libfontconfig1_2.15.0-1.1ubuntu2_amd64.deb ...
Unpacking libfontconfig1:amd64 (2.15.0-1.1ubuntu2) ...
Selecting previously unselected package libsharpyuv0:amd64.
Preparing to unpack .../56-libsharpyuv0_1.3.2-0.4build3_amd64.deb ...
Unpacking libsharpyuv0:amd64 (1.3.2-0.4build3) ...
Selecting previously unselected package libheif-plugin-aomdec:amd64.
Preparing to unpack .../57-libheif-plugin-aomdec_1.17.6-1ubuntu4.2_amd64.deb ...
Unpacking libheif-plugin-aomdec:amd64 (1.17.6-1ubuntu4.2) ...
Selecting previously unselected package libde265-0:amd64.
Preparing to unpack .../58-libde265-0_1.0.15-1build3_amd64.deb ...
Unpacking libde265-0:amd64 (1.0.15-1build3) ...
Selecting previously unselected package libheif-plugin-libde265:amd64.
Preparing to unpack .../59-libheif-plugin-libde265_1.17.6-1ubuntu4.2_amd64.deb ...
Unpacking libheif-plugin-libde265:amd64 (1.17.6-1ubuntu4.2) ...
Selecting previously unselected package libheif1:amd64.
Preparing to unpack .../60-libheif1_1.17.6-1ubuntu4.2_amd64.deb ...
Unpacking libheif1:amd64 (1.17.6-1ubuntu4.2) ...
Selecting previously unselected package libjpeg-turbo8:amd64.
Preparing to unpack .../61-libjpeg-turbo8_2.1.5-2ubuntu2_amd64.deb ...
Unpacking libjpeg-turbo8:amd64 (2.1.5-2ubuntu2) ...
Selecting previously unselected package libjpeg8:amd64.
Preparing to unpack .../62-libjpeg8_8c-2ubuntu11_amd64.deb ...
Unpacking libjpeg8:amd64 (8c-2ubuntu11) ...
Selecting previously unselected package libdeflate0:amd64.
Preparing to unpack .../63-libdeflate0_1.19-1build1.1_amd64.deb ...
Unpacking libdeflate0:amd64 (1.19-1build1.1) ...
Selecting previously unselected package libjbig0:amd64.
Preparing to unpack .../64-libjbig0_2.1-6.1ubuntu2_amd64.deb ...
Unpacking libjbig0:amd64 (2.1-6.1ubuntu2) ...
Selecting previously unselected package liblerc4:amd64.
Preparing to unpack .../65-liblerc4_4.0.0+ds-4ubuntu2_amd64.deb ...
Unpacking liblerc4:amd64 (4.0.0+ds-4ubuntu2) ...
Selecting previously unselected package libwebp7:amd64.
Preparing to unpack .../66-libwebp7_1.3.2-0.4build3_amd64.deb ...
Unpacking libwebp7:amd64 (1.3.2-0.4build3) ...
Selecting previously unselected package libtiff6:amd64.
Preparing to unpack .../67-libtiff6_4.5.1+git230720-4ubuntu2.4_amd64.deb ...
Unpacking libtiff6:amd64 (4.5.1+git230720-4ubuntu2.4) ...
Selecting previously unselected package libxpm4:amd64.
Preparing to unpack .../68-libxpm4_1%3a3.5.17-1build2_amd64.deb ...
Unpacking libxpm4:amd64 (1:3.5.17-1build2) ...
Selecting previously unselected package libgd3:amd64.
Preparing to unpack .../69-libgd3_2.3.3-9ubuntu5_amd64.deb ...
Unpacking libgd3:amd64 (2.3.3-9ubuntu5) ...
Selecting previously unselected package libc-devtools.
Preparing to unpack .../70-libc-devtools_2.39-0ubuntu8.7_amd64.deb ...
Unpacking libc-devtools (2.39-0ubuntu8.7) ...
Selecting previously unselected package libfile-fcntllock-perl.
Preparing to unpack .../71-libfile-fcntllock-perl_0.22-4ubuntu5_amd64.deb ...
Unpacking libfile-fcntllock-perl (0.22-4ubuntu5) ...
Selecting previously unselected package libheif-plugin-aomenc:amd64.
Preparing to unpack .../72-libheif-plugin-aomenc_1.17.6-1ubuntu4.2_amd64.deb ...
Unpacking libheif-plugin-aomenc:amd64 (1.17.6-1ubuntu4.2) ...
Selecting previously unselected package libpkgconf3:amd64.
Preparing to unpack .../73-libpkgconf3_1.8.1-2build1_amd64.deb ...
Unpacking libpkgconf3:amd64 (1.8.1-2build1) ...
Selecting previously unselected package libssl-dev:amd64.
Preparing to unpack .../74-libssl-dev_3.0.13-0ubuntu3.7_amd64.deb ...
Unpacking libssl-dev:amd64 (3.0.13-0ubuntu3.7) ...
Selecting previously unselected package manpages-dev.
Preparing to unpack .../75-manpages-dev_6.7-2_all.deb ...
Unpacking manpages-dev (6.7-2) ...
Selecting previously unselected package pkgconf-bin.
Preparing to unpack .../76-pkgconf-bin_1.8.1-2build1_amd64.deb ...
Unpacking pkgconf-bin (1.8.1-2build1) ...
Selecting previously unselected package pkgconf:amd64.
Preparing to unpack .../77-pkgconf_1.8.1-2build1_amd64.deb ...
Unpacking pkgconf:amd64 (1.8.1-2build1) ...
Selecting previously unselected package pkg-config:amd64.
Preparing to unpack .../78-pkg-config_1.8.1-2build1_amd64.deb ...
Unpacking pkg-config:amd64 (1.8.1-2build1) ...
Setting up libsharpyuv0:amd64 (1.3.2-0.4build3) ...
Setting up libaom3:amd64 (3.8.2-2ubuntu0.1) ...
Setting up manpages-dev (6.7-2) ...
Setting up lto-disabled-list (47) ...
Setting up liblerc4:amd64 (4.0.0+ds-4ubuntu2) ...
Setting up libxpm4:amd64 (1:3.5.17-1build2) ...
Setting up libfile-fcntllock-perl (0.22-4ubuntu5) ...
Setting up libalgorithm-diff-perl (1.201-1) ...
Setting up binutils-common:amd64 (2.42-4ubuntu2.8) ...
Setting up libdeflate0:amd64 (1.19-1build1.1) ...
Setting up linux-libc-dev:amd64 (6.8.0-101.101) ...
Setting up libctf-nobfd0:amd64 (2.42-4ubuntu2.8) ...
Setting up libgomp1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up bzip2 (1.0.8-5.1build0.1) ...
Setting up libjbig0:amd64 (2.1-6.1ubuntu2) ...
Setting up libsframe1:amd64 (2.42-4ubuntu2.8) ...
Setting up libfakeroot:amd64 (1.33-1) ...
Setting up fakeroot (1.33-1) ...
update-alternatives: using /usr/bin/fakeroot-sysv to provide /usr/bin/fakeroot (fakeroot) in auto mode
Setting up libpkgconf3:amd64 (1.8.1-2build1) ...
Setting up rpcsvc-proto (1.4.2-0ubuntu7) ...
Setting up gcc-13-base:amd64 (13.3.0-6ubuntu2~24.04.1) ...
Setting up make (4.3-4.1build2) ...
Setting up libquadmath0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up fonts-dejavu-mono (2.37-8) ...
Setting up libssl-dev:amd64 (3.0.13-0ubuntu3.7) ...
Setting up libmpc3:amd64 (1.3.1-1build1.1) ...
Setting up libatomic1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up fonts-dejavu-core (2.37-8) ...
Setting up pkgconf-bin (1.8.1-2build1) ...
Setting up libjpeg-turbo8:amd64 (2.1.5-2ubuntu2) ...
Setting up libdpkg-perl (1.22.6ubuntu6.5) ...
Setting up libwebp7:amd64 (1.3.2-0.4build3) ...
Setting up libubsan1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up libhwasan0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up libcrypt-dev:amd64 (1:4.4.36-4build1) ...
Setting up libasan8:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up libtsan2:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up libbinutils:amd64 (2.42-4ubuntu2.8) ...
Setting up libisl23:amd64 (0.26-3build1.1) ...
Setting up libde265-0:amd64 (1.0.15-1build3) ...
Setting up libc-dev-bin (2.39-0ubuntu8.7) ...
Setting up libalgorithm-diff-xs-perl:amd64 (0.04-8build3) ...
Setting up libcc1-0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up liblsan0:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up libitm1:amd64 (14.2.0-4ubuntu2~24.04.1) ...
Setting up libalgorithm-merge-perl (0.08-5) ...
Setting up libctf0:amd64 (2.42-4ubuntu2.8) ...
Setting up libjpeg8:amd64 (8c-2ubuntu11) ...
Setting up cpp-13-x86-64-linux-gnu (13.3.0-6ubuntu2~24.04.1) ...
Setting up fontconfig-config (2.15.0-1.1ubuntu2) ...
Setting up pkgconf:amd64 (1.8.1-2build1) ...
Setting up libgprofng0:amd64 (2.42-4ubuntu2.8) ...
Setting up pkg-config:amd64 (1.8.1-2build1) ...
Setting up libgcc-13-dev:amd64 (13.3.0-6ubuntu2~24.04.1) ...
Setting up libtiff6:amd64 (4.5.1+git230720-4ubuntu2.4) ...
Setting up libc6-dev:amd64 (2.39-0ubuntu8.7) ...
Setting up libstdc++-13-dev:amd64 (13.3.0-6ubuntu2~24.04.1) ...
Setting up binutils-x86-64-linux-gnu (2.42-4ubuntu2.8) ...
Setting up cpp-x86-64-linux-gnu (4:13.2.0-7ubuntu1) ...
Setting up cpp-13 (13.3.0-6ubuntu2~24.04.1) ...
Setting up gcc-13-x86-64-linux-gnu (13.3.0-6ubuntu2~24.04.1) ...
Setting up binutils (2.42-4ubuntu2.8) ...
Setting up dpkg-dev (1.22.6ubuntu6.5) ...
Setting up gcc-13 (13.3.0-6ubuntu2~24.04.1) ...
Setting up cpp (4:13.2.0-7ubuntu1) ...
Setting up g++-13-x86-64-linux-gnu (13.3.0-6ubuntu2~24.04.1) ...
Setting up gcc-x86-64-linux-gnu (4:13.2.0-7ubuntu1) ...
Setting up gcc (4:13.2.0-7ubuntu1) ...
Setting up g++-x86-64-linux-gnu (4:13.2.0-7ubuntu1) ...
Setting up g++-13 (13.3.0-6ubuntu2~24.04.1) ...
Setting up g++ (4:13.2.0-7ubuntu1) ...
update-alternatives: using /usr/bin/g++ to provide /usr/bin/c++ (c++) in auto mode
Setting up build-essential (12.10ubuntu1) ...
Setting up libheif-plugin-aomdec:amd64 (1.17.6-1ubuntu4.2) ...
Setting up libheif1:amd64 (1.17.6-1ubuntu4.2) ...
Setting up libheif-plugin-libde265:amd64 (1.17.6-1ubuntu4.2) ...
Setting up libheif-plugin-aomenc:amd64 (1.17.6-1ubuntu4.2) ...
Processing triggers for libc-bin (2.39-0ubuntu8.7) ...
Processing triggers for man-db (2.12.0-4build2) ...
Processing triggers for sgml-base (1.31) ...
Setting up libfontconfig1:amd64 (2.15.0-1.1ubuntu2) ...
Setting up libgd3:amd64 (2.3.3-9ubuntu5) ...
Setting up libc-devtools (2.39-0ubuntu8.7) ...
Processing triggers for libc-bin (2.39-0ubuntu8.7) ...
Scanning processes...
Scanning linux images...

Running kernel seems to be up-to-date.

No services need to be restarted.

No containers need to be restarted.

No user sessions are running outdated binaries.

No VM guests are running outdated hypervisor (qemu) binaries
 on this host.
info: downloading installer
info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2026-03-05, rust version 1.94.0 (4a4ef493e 2026-03-02)
info: downloading component 'cargo'
 10.5 MiB /  10.5 MiB (100 %)   8.4 MiB/s in  1s
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 20.6 MiB /  20.6 MiB (100 %)   6.3 MiB/s in  3s
info: downloading component 'rust-std'
 28.4 MiB /  28.4 MiB (100 %)   6.3 MiB/s in  5s
info: downloading component 'rustc'
 74.9 MiB /  74.9 MiB (100 %)   5.8 MiB/s in 13s
info: downloading component 'rustfmt'
info: installing component 'cargo'
 10.5 MiB /  10.5 MiB (100 %)   6.8 MiB/s in  1s
info: installing component 'clippy'
info: installing component 'rust-docs'
 20.6 MiB /  20.6 MiB (100 %)   2.1 MiB/s in 18s
info: installing component 'rust-std'
 28.4 MiB /  28.4 MiB (100 %)   5.0 MiB/s in  6s
info: installing component 'rustc'
 74.9 MiB /  74.9 MiB (100 %)   5.9 MiB/s in 14s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.94.0 (4a4ef493e 2026-03-02)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source $"($nu.home-path)/.cargo/env.nu"  # For nushell
Cloning into 'Swarm-Runtime'...
remote: Enumerating objects: 1453, done.
remote: Counting objects: 100% (922/922), done.
remote: Compressing objects: 100% (700/700), done.
remote: Total 1453 (delta 247), reused 857 (delta 187), pack-reused 531 (from 1)
Receiving objects: 100% (1453/1453), 35.49 MiB | 6.19 MiB/s, done.
Resolving deltas: 100% (472/472), done.
Updating files: 100% (566/566), done.
    Updating crates.io index
  Downloaded http-body v0.4.6
  Downloaded fxhash v0.2.1
  Downloaded heck v0.4.1
  Downloaded hyper-tls v0.5.0
  Downloaded heck v0.5.0
  Downloaded ghash v0.5.1
  Downloaded httpdate v1.0.3
  Downloaded futures-rustls v0.26.0
  Downloaded idna_adapter v1.2.1
  Downloaded lru-cache v0.1.2
  Downloaded hex_fmt v0.3.0
  Downloaded netlink-packet-utils v0.5.2
  Downloaded libp2p-upnp v0.2.2
  Downloaded itoa v1.0.17
  Downloaded libp2p-metrics v0.14.1
  Downloaded match-lookup v0.1.2
  Downloaded netlink-packet-core v0.7.0
  Downloaded multistream-select v0.13.0
  Downloaded native-tls v0.2.18
  Downloaded oid-registry v0.7.1
  Downloaded num-conv v0.2.0
  Downloaded netlink-sys v0.8.8
  Downloaded litemap v0.8.1
  Downloaded libp2p-identity v0.2.13
  Downloaded futures-executor v0.3.31
  Downloaded base64ct v1.8.3
  Downloaded crossbeam-epoch v0.9.18
  Downloaded futures-core v0.3.31
  Downloaded data-encoding-macro v0.1.19
  Downloaded foreign-types v0.3.2
  Downloaded deranged v0.5.6
  Downloaded futures-bounded v0.2.4
  Downloaded cfg-if v1.0.4
  Downloaded futures-io v0.3.31
  Downloaded form_urlencoded v1.2.2
  Downloaded find-msvc-tools v0.1.9
  Downloaded dirs v4.0.0
  Downloaded dirs-sys v0.3.7
  Downloaded downcast-rs v1.2.1
  Downloaded equivalent v1.0.2
  Downloaded crossbeam-utils v0.8.21
  Downloaded fs2 v0.4.3
  Downloaded ctr v0.9.2
  Downloaded data-encoding v2.10.0
  Downloaded bitflags v1.3.2
  Downloaded colorchoice v1.0.4
  Downloaded dtoa v1.0.11
  Downloaded foreign-types-shared v0.1.1
  Downloaded clap_derive v4.5.55
  Downloaded chacha20 v0.9.1
  Downloaded crypto-common v0.1.7
  Downloaded errno v0.3.14
  Downloaded enum-as-inner v0.6.1
  Downloaded log v0.4.29
  Downloaded futures-macro v0.3.31
  Downloaded cfg_aliases v0.2.1
  Downloaded der-parser v9.0.0
  Downloaded num-integer v0.1.46
  Downloaded getrandom v0.3.4
  Downloaded data-encoding-macro-internal v0.1.17
  Downloaded aes v0.8.4
  Downloaded cbor4ii v0.3.3
  Downloaded futures-channel v0.3.31
  Downloaded fnv v1.0.7
  Downloaded displaydoc v0.2.5
  Downloaded foldhash v0.1.5
  Downloaded ed25519 v2.2.3
  Downloaded clap v4.5.58
  Downloaded dashmap v5.5.3
  Downloaded clap_lex v1.0.0
  Downloaded bs58 v0.5.1
  Downloaded cipher v0.4.4
  Downloaded digest v0.10.7
  Downloaded curve25519-dalek-derive v0.1.1
  Downloaded crunchy v0.2.4
  Downloaded core2 v0.4.0
  Downloaded const-oid v0.9.6
  Downloaded blake2 v0.10.6
  Downloaded const-str v0.4.3
  Downloaded openssl-macros v0.1.1
  Downloaded cpufeatures v0.2.17
  Downloaded byteorder v1.5.0
  Downloaded crc32fast v1.5.0
  Downloaded bitflags v2.10.0
  Downloaded base256emoji v1.0.2
  Downloaded libp2p-core v0.41.3
  Downloaded env_logger v0.10.2
  Downloaded futures v0.3.31
  Downloaded libp2p v0.53.2
  Downloaded anstyle-query v1.1.5
  Downloaded io-lifetimes v0.7.5
  Downloaded igd-next v0.14.3
  Downloaded asn1-rs v0.6.2
  Downloaded icu_provider v2.1.1
  Downloaded parking_lot v0.12.5
  Downloaded parking_lot v0.11.2
  Downloaded icu_properties v2.1.2
  Downloaded num_cpus v1.17.0
  Downloaded percent-encoding v2.3.2
  Downloaded pin-utils v0.1.0
  Downloaded nohash-hasher v0.2.0
  Downloaded icu_normalizer v2.1.1
  Downloaded pkcs8 v0.10.2
  Downloaded polyval v0.6.2
  Downloaded icu_collections v2.1.1
  Downloaded pin-project-lite v0.2.16
  Downloaded axum v0.6.20
  Downloaded icu_normalizer_data v2.1.1
  Downloaded icu_locale_core v2.1.1
  Downloaded aho-corasick v1.1.4
  Downloaded rand_core v0.9.5
  Downloaded opaque-debug v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded openssl-probe v0.2.1
  Downloaded once_cell v1.21.3
  Downloaded minimal-lexical v0.2.1
  Downloaded num-traits v0.2.19
  Downloaded hickory-resolver v0.24.4
  Downloaded libp2p-identify v0.44.2
  Downloaded memchr v2.8.0
  Downloaded libp2p-swarm v0.44.2
  Downloaded rustc_version v0.4.1
  Downloaded clap_builder v4.5.58
  Downloaded stable_deref_trait v1.2.1
  Downloaded rusticata-macros v4.1.0
  Downloaded aes-gcm v0.10.3
  Downloaded netlink-proto v0.11.5
  Downloaded libp2p-request-response v0.26.3
  Downloaded ipnet v2.11.0
  Downloaded semver v1.0.27
  Downloaded base64 v0.22.1
  Downloaded httparse v1.10.1
  Downloaded hmac v0.12.1
  Downloaded either v1.15.0
  Downloaded block-buffer v0.10.4
  Downloaded autocfg v1.5.0
  Downloaded multihash v0.19.3
  Downloaded static_assertions v1.1.0
  Downloaded sync_wrapper v0.1.2
  Downloaded strsim v0.11.1
  Downloaded rcgen v0.11.3
  Downloaded pem v3.0.6
  Downloaded paste v1.0.15
  Downloaded serde_core v1.0.228
  Downloaded subtle v2.6.1
  Downloaded indexmap v2.13.0
  Downloaded multibase v0.9.2
  Downloaded maybe-owned v0.3.4
  Downloaded matchit v0.7.3
  Downloaded lock_api v0.4.14
  Downloaded libp2p-tcp v0.41.0
  Downloaded libp2p-swarm-derive v0.34.2
  Downloaded libp2p-kad v0.45.3
  Downloaded libp2p-gossipsub v0.46.1
  Downloaded synstructure v0.13.2
  Downloaded parking_lot_core v0.9.12
  Downloaded netlink-packet-route v0.17.1
  Downloaded termcolor v1.4.1
  Downloaded chacha20poly1305 v0.10.1
  Downloaded mio v1.1.1
  Downloaded hashbrown v0.14.5
  Downloaded parking_lot_core v0.8.6
  Downloaded hashbrown v0.15.5
  Downloaded http v0.2.12
  Downloaded time-core v0.1.8
  Downloaded futures-util v0.3.31
  Downloaded pin-project v1.1.10
  Downloaded pin-project-internal v1.1.10
  Downloaded tinyvec_macros v0.1.1
  Downloaded icu_properties_data v2.1.2
  Downloaded tower-service v0.3.3
  Downloaded libm v0.2.16
  Downloaded tinystr v0.8.2
  Downloaded h2 v0.3.27
  Downloaded time-macros v0.2.27
  Downloaded pkg-config v0.3.32
  Downloaded thiserror-impl v2.0.18
  Downloaded tower-layer v0.3.3
  Downloaded thiserror-impl v1.0.69
  Downloaded thiserror v2.0.18
  Downloaded hyper v0.14.32
  Downloaded tokio-native-tls v0.3.1
  Downloaded thiserror v1.0.69
  Downloaded try-lock v0.2.5
  Downloaded nix v0.26.4
  Downloaded tokio-macros v2.6.0
  Downloaded hickory-proto v0.24.4
  Downloaded universal-hash v0.5.1
  Downloaded unsigned-varint v0.7.2
  Downloaded der v0.7.10
  Downloaded hkdf v0.12.4
  Downloaded tinyvec v1.10.0
  Downloaded idna v1.1.0
  Downloaded hashbrown v0.16.1
  Downloaded ed25519-dalek v2.2.0
  Downloaded base64 v0.21.7
  Downloaded if-watch v3.2.1
  Downloaded untrusted v0.7.1
  Downloaded uint v0.9.5
  Downloaded poly1305 v0.8.0
  Downloaded utf8parse v0.2.2
  Downloaded bytes v1.11.1
  Downloaded tracing-attributes v0.1.31
  Downloaded unsigned-varint v0.8.0
  Downloaded want v0.3.1
  Downloaded void v1.0.2
  Downloaded base-x v0.2.11
  Downloaded version_check v0.9.5
  Downloaded ambient-authority v0.0.1
  Downloaded potential_utf v0.1.4
  Downloaded asynchronous-codec v0.7.0
  Downloaded powerfmt v0.2.0
  Downloaded ppv-lite86 v0.2.21
  Downloaded untrusted v0.9.0
  Downloaded quick-protobuf-codec v0.3.1
  Downloaded quinn-udp v0.5.14
  Downloaded prometheus-client-derive-encode v0.4.2
  Downloaded quote v1.0.44
  Downloaded prometheus-client v0.22.3
  Downloaded rand_chacha v0.3.1
  Downloaded num-bigint v0.4.6
  Downloaded rustc-hash v2.1.1
  Downloaded rand_chacha v0.9.0
  Downloaded openssl-sys v0.9.111
  Downloaded scopeguard v1.2.0
  Downloaded ryu v1.0.23
  Downloaded signal-hook-registry v1.4.8
  Downloaded serde_urlencoded v0.7.1
  Downloaded rustls-pki-types v1.14.0
  Downloaded shlex v1.3.0
  Downloaded web-time v1.1.0
  Downloaded spin v0.9.8
  Downloaded spki v0.7.3
  Downloaded wasmi_arena v0.4.1
  Downloaded zeroize v1.8.2
  Downloaded cap-std v0.26.1
  Downloaded zmij v1.0.20
  Downloaded quinn v0.11.9
  Downloaded rand v0.8.5
  Downloaded xmltree v0.10.3
  Downloaded reqwest v0.11.27
  Downloaded wasmi_wasi v0.31.2
  Downloaded rand v0.9.2
  Downloaded quinn-proto v0.11.13
  Downloaded quick-protobuf v0.8.1
  Downloaded yoke-derive v0.8.1
  Downloaded wiggle-macro v2.0.2
  Downloaded openssl v0.10.75
  Downloaded unicode-ident v1.0.23
  Downloaded yoke v0.8.1
  Downloaded nom v7.1.3
  Downloaded regex-automata v0.4.14
  Downloaded is-terminal v0.3.0
  Downloaded zerofrom v0.1.6
  Downloaded writeable v0.6.2
  Downloaded zeroize_derive v1.4.3
  Downloaded tracing-core v0.1.36
  Downloaded io-extras v0.15.0
  Downloaded utf8_iter v1.0.4
  Downloaded spin v0.5.2
  Downloaded socket2 v0.6.2
  Downloaded wasi-cap-std-sync v2.0.2
  Downloaded socket2 v0.5.10
  Downloaded signature v2.2.0
  Downloaded system-interface v0.23.0
  Downloaded slab v0.4.12
  Downloaded wasmi_core v0.13.0
  Downloaded smallvec v1.15.1
  Downloaded shellexpand v2.1.2
  Downloaded sha2 v0.10.9
  Downloaded serde_path_to_error v0.1.20
  Downloaded serde_derive v1.0.228
  Downloaded rustls-pemfile v1.0.4
  Downloaded fs-set-times v0.17.1
  Downloaded cap-rand v0.26.1
  Downloaded cap-fs-ext v0.26.1
  Downloaded yamux v0.13.8
  Downloaded ring v0.17.14
  Downloaded zerofrom-derive v0.1.6
  Downloaded witx v0.9.1
  Downloaded uuid v1.20.0
  Downloaded rustls-webpki v0.103.9
  Downloaded yamux v0.12.1
  Downloaded wiggle-generate v2.0.2
  Downloaded wiggle v2.0.2
  Downloaded rw-stream-sink v0.4.0
  Downloaded rustversion v1.0.22
  Downloaded rtnetlink v0.13.1
  Downloaded zerovec-derive v0.11.2
  Downloaded resolv-conf v0.7.6
  Downloaded yasna v0.5.2
  Downloaded rustls-webpki v0.101.7
  Downloaded cc v1.2.55
  Downloaded xml-rs v0.8.28
  Downloaded regex-syntax v0.8.9
  Downloaded proc-macro2 v1.0.106
  Downloaded cap-time-ext v0.26.1
  Downloaded libc v0.2.181
  Downloaded serde v1.0.228
  Downloaded rustls v0.23.36
  Downloaded linux-raw-sys v0.0.46
  Downloaded asn1-rs-derive v0.5.1
  Downloaded multiaddr v0.18.2
  Downloaded mime v0.3.17
  Downloaded libp2p-quic v0.10.3
  Downloaded libp2p-noise v0.44.0
  Downloaded libp2p-mdns v0.45.1
  Downloaded typenum v1.19.0
  Downloaded getrandom v0.2.17
  Downloaded regex v1.12.3
  Downloaded futures-timer v3.0.3
  Downloaded axum-macros v0.3.8
  Downloaded attohttpc v0.24.1
  Downloaded asn1-rs-impl v0.2.0
  Downloaded aead v0.5.2
  Downloaded lru v0.12.5
  Downloaded lazy_static v1.5.0
  Downloaded humantime v2.3.0
  Downloaded lru-slab v0.1.2
  Downloaded libp2p-yamux v0.45.2
  Downloaded libp2p-tls v0.4.1
  Downloaded libp2p-allow-block-list v0.3.0
  Downloaded is_terminal_polyfill v1.70.2
  Downloaded instant v0.1.13
  Downloaded inout v0.1.4
  Downloaded indexmap-nostd v0.4.0
  Downloaded serde_json v1.0.149
  Downloaded tower v0.4.13
  Downloaded libp2p-dns v0.41.1
  Downloaded leb128 v0.2.5
  Downloaded arrayvec v0.7.6
  Downloaded async-trait v0.1.89
  Downloaded arrayref v0.3.9
  Downloaded anyhow v1.0.101
  Downloaded libp2p-connection-limits v0.3.1
  Downloaded axum-core v0.3.4
  Downloaded linked-hash-map v0.5.6
  Downloaded is-terminal v0.4.17
  Downloaded anstyle-parse v0.2.7
  Downloaded anstream v0.6.21
  Downloaded allocator-api2 v0.2.21
  Downloaded anstyle v1.0.13
  Downloaded futures-ticker v0.0.3
  Downloaded tokio-util v0.7.18
  Downloaded futures-task v0.3.31
  Downloaded cap-primitives v0.26.1
  Downloaded url v2.5.8
  Downloaded hex v0.4.3
  Downloaded generic-array v0.14.7
  Downloaded futures-sink v0.3.31
  Downloaded sled v0.34.7
  Downloaded snow v0.9.6
  Downloaded curve25519-dalek v4.1.3
  Downloaded wast v35.0.2
  Downloaded zerocopy v0.8.39
  Downloaded x25519-dalek v2.0.1
  Downloaded x509-parser v0.16.0
  Downloaded syn v1.0.109
  Downloaded sysinfo v0.38.2
  Downloaded zerotrie v0.2.3
  Downloaded zerovec v0.11.5
  Downloaded wasmparser-nostd v0.100.2
  Downloaded wasmi v0.31.2
  Downloaded wasi-common v2.0.2
  Downloaded time v0.3.47
  Downloaded syn v2.0.114
  Downloaded vcpkg v0.2.15
  Downloaded rustix v0.35.16
  Downloaded ring v0.16.20
  Downloaded tracing v0.1.44
  Downloaded tokio v1.49.0
  Downloaded encoding_rs v0.8.35
  Downloaded 372 crates (28.7MiB) in 5.39s (largest was `ring` at 4.8MiB)
   Compiling proc-macro2 v1.0.106
   Compiling unicode-ident v1.0.23
   Compiling quote v1.0.44
   Compiling libc v0.2.181
   Compiling syn v2.0.114
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.16
   Compiling memchr v2.8.0
   Compiling smallvec v1.15.1
   Compiling log v0.4.29
   Compiling futures-core v0.3.31
   Compiling futures-sink v0.3.31
   Compiling once_cell v1.21.3
   Compiling futures-channel v0.3.31
   Compiling slab v0.4.12
   Compiling futures-task v0.3.31
   Compiling pin-utils v0.1.0
   Compiling futures-io v0.3.31
   Compiling thiserror v1.0.69
   Compiling futures-macro v0.3.31
   Compiling thiserror-impl v1.0.69
   Compiling futures-util v0.3.31
   Compiling synstructure v0.13.2
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling parking_lot_core v0.9.12
   Compiling zerocopy v0.8.39
   Compiling zerofrom-derive v0.1.6
   Compiling bytes v1.11.1
   Compiling zerofrom v0.1.6
   Compiling yoke-derive v0.8.1
   Compiling getrandom v0.2.17
   Compiling typenum v1.19.0
   Compiling stable_deref_trait v1.2.1
   Compiling version_check v0.9.5
   Compiling generic-array v0.14.7
   Compiling yoke v0.8.1
   Compiling ppv-lite86 v0.2.21
   Compiling rand_core v0.6.4
   Compiling parking_lot v0.12.5
   Compiling zerovec-derive v0.11.2
   Compiling tracing-attributes v0.1.31
   Compiling tracing-core v0.1.36
   Compiling byteorder v1.5.0
   Compiling zerovec v0.11.5
   Compiling tracing v0.1.44
   Compiling displaydoc v0.2.5
   Compiling semver v1.0.27
   Compiling itoa v1.0.17
   Compiling serde_core v1.0.228
   Compiling subtle v2.6.1
   Compiling rustc_version v0.4.1
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling rand_chacha v0.3.1
   Compiling num_cpus v1.17.0
   Compiling rand v0.8.5
   Compiling futures-executor v0.3.31
   Compiling digest v0.10.7
   Compiling tinystr v0.8.2
   Compiling litemap v0.8.1
   Compiling writeable v0.6.2
   Compiling icu_locale_core v2.1.1
   Compiling futures v0.3.31
   Compiling zerotrie v0.2.3
   Compiling potential_utf v0.1.4
   Compiling errno v0.3.14
   Compiling bitflags v1.3.2
   Compiling icu_properties_data v2.1.2
   Compiling icu_normalizer_data v2.1.1
   Compiling signal-hook-registry v1.4.8
   Compiling icu_collections v2.1.1
   Compiling icu_provider v2.1.1
   Compiling pin-project-internal v1.1.10
   Compiling tokio-macros v2.6.0
   Compiling socket2 v0.6.2
   Compiling mio v1.1.1
   Compiling anyhow v1.0.101
   Compiling thiserror v2.0.18
   Compiling fnv v1.0.7
   Compiling cpufeatures v0.2.17
   Compiling tokio v1.49.0
   Compiling pin-project v1.1.10
   Compiling curve25519-dalek v4.1.3
   Compiling thiserror-impl v2.0.18
   Compiling zeroize_derive v1.4.3
   Compiling percent-encoding v2.3.2
   Compiling serde v1.0.228
   Compiling form_urlencoded v1.2.2
   Compiling zeroize v1.8.2
   Compiling icu_normalizer v2.1.1
   Compiling icu_properties v2.1.2
   Compiling curve25519-dalek-derive v0.1.1
   Compiling serde_derive v1.0.228
   Compiling static_assertions v1.1.0
   Compiling idna_adapter v1.2.1
   Compiling sha2 v0.10.9
   Compiling data-encoding v2.10.0
   Compiling unsigned-varint v0.8.0
   Compiling utf8_iter v1.0.4
   Compiling signature v2.2.0
   Compiling data-encoding-macro-internal v0.1.17
   Compiling ed25519 v2.2.3
   Compiling idna v1.1.0
   Compiling hmac v0.12.1
   Compiling quick-protobuf v0.8.1
   Compiling match-lookup v0.1.2
   Compiling core2 v0.4.0
   Compiling const-str v0.4.3
   Compiling multihash v0.19.3
   Compiling hkdf v0.12.4
   Compiling data-encoding-macro v0.1.19
   Compiling base256emoji v1.0.2
   Compiling url v2.5.8
   Compiling ed25519-dalek v2.2.0
   Compiling base-x v0.2.11
   Compiling paste v1.0.15
   Compiling bs58 v0.5.1
   Compiling equivalent v1.0.2
   Compiling futures-timer v3.0.3
   Compiling libp2p-identity v0.2.13
   Compiling multibase v0.9.2
   Compiling ipnet v2.11.0
   Compiling heck v0.5.0
   Compiling unsigned-varint v0.7.2
   Compiling web-time v1.1.0
   Compiling arrayref v0.3.9
   Compiling io-lifetimes v0.7.5
   Compiling multiaddr v0.18.2
   Compiling multistream-select v0.13.0
   Compiling rw-stream-sink v0.4.0
   Compiling either v1.15.0
   Compiling void v1.0.2
   Compiling libp2p-core v0.41.3
   Compiling instant v0.1.13
   Compiling rustix v0.35.16
   Compiling async-trait v0.1.89
   Compiling allocator-api2 v0.2.21
   Compiling foldhash v0.1.5
   Compiling linux-raw-sys v0.0.46
   Compiling hashbrown v0.15.5
   Compiling io-extras v0.15.0
   Compiling lru v0.12.5
   Compiling libp2p-swarm-derive v0.34.2
   Compiling getrandom v0.3.4
   Compiling cap-primitives v0.26.1
   Compiling leb128 v0.2.5
   Compiling ambient-authority v0.0.1
   Compiling wast v35.0.2
   Compiling libp2p-swarm v0.44.2
   Compiling dirs-sys v0.3.7
   Compiling fs-set-times v0.17.1
   Compiling netlink-packet-utils v0.5.2
   Compiling shlex v1.3.0
   Compiling maybe-owned v0.3.4
   Compiling syn v1.0.109
   Compiling find-msvc-tools v0.1.9
   Compiling heck v0.4.1
   Compiling cc v1.2.55
   Compiling netlink-packet-core v0.7.0
   Compiling dirs v4.0.0
   Compiling cap-std v0.26.1
   Compiling rustversion v1.0.22
   Compiling witx v0.9.1
   Compiling ring v0.17.14
   Compiling shellexpand v2.1.2
   Compiling netlink-sys v0.8.8
   Compiling asynchronous-codec v0.7.0
   Compiling socket2 v0.5.10
   Compiling autocfg v1.5.0
   Compiling num-traits v0.2.19
   Compiling netlink-proto v0.11.5
   Compiling wiggle-generate v2.0.2
   Compiling rand_core v0.9.5
   Compiling netlink-packet-route v0.17.1
   Compiling nix v0.26.4
   Compiling aho-corasick v1.1.4
   Compiling crunchy v0.2.4
   Compiling regex-syntax v0.8.9
   Compiling zmij v1.0.20
   Compiling libm v0.2.16
   Compiling rtnetlink v0.13.1
   Compiling regex-automata v0.4.14
   Compiling rand_chacha v0.9.0
   Compiling wiggle-macro v2.0.2
   Compiling quick-protobuf-codec v0.3.1
   Compiling futures-bounded v0.2.4
   Compiling http v0.2.12
   Compiling snow v0.9.6
   Compiling cap-fs-ext v0.26.1
   Compiling wasi-common v2.0.2
   Compiling system-interface v0.23.0
   Compiling serde_json v1.0.149
   Compiling tinyvec_macros v0.1.1
   Compiling nohash-hasher v0.2.0
   Compiling untrusted v0.9.0
   Compiling prometheus-client v0.22.3
   Compiling tinyvec v1.10.0
   Compiling wiggle v2.0.2
   Compiling regex v1.12.3
   Compiling rand v0.9.2
   Compiling if-watch v3.2.1
   Compiling cap-rand v0.26.1
   Compiling enum-as-inner v0.6.1
   Compiling prometheus-client-derive-encode v0.4.2
   Compiling parking_lot_core v0.8.6
   Compiling hashbrown v0.16.1
   Compiling indexmap-nostd v0.4.0
   Compiling hex v0.4.3
   Compiling tower-service v0.3.3
   Compiling utf8parse v0.2.2
   Compiling httparse v1.10.1
   Compiling downcast-rs v1.2.1
   Compiling crossbeam-utils v0.8.21
   Compiling dtoa v1.0.11
   Compiling wasmi_core v0.13.0
   Compiling anstyle-parse v0.2.7
   Compiling indexmap v2.13.0
   Compiling uint v0.9.5
   Compiling wasmparser-nostd v0.100.2
   Compiling hickory-proto v0.24.4
   Compiling yamux v0.13.8
   Compiling yamux v0.12.1
   Compiling http-body v0.4.6
   Compiling axum-core v0.3.4
   Compiling cap-time-ext v0.26.1
   Compiling is-terminal v0.3.0
   Compiling futures-ticker v0.0.3
   Compiling cbor4ii v0.3.3
   Compiling x25519-dalek v2.0.1
   Compiling tokio-util v0.7.18
   Compiling spin v0.9.8
   Compiling anstyle-query v1.1.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling hex_fmt v0.3.0
   Compiling wasmi_arena v0.4.1
   Compiling arrayvec v0.7.6
   Compiling colorchoice v1.0.4
   Compiling crc32fast v1.5.0
   Compiling tower-layer v0.3.3
   Compiling anstyle v1.0.13
   Compiling try-lock v0.2.5
   Compiling base64 v0.21.7
   Compiling want v0.3.1
   Compiling libp2p-gossipsub v0.46.1
   Compiling anstream v0.6.21
   Compiling libp2p-kad v0.45.3
   Compiling wasmi v0.31.2
   Compiling h2 v0.3.27
Read from remote host 145.241.192.79: Connection reset by peer
Connection to 145.241.192.79 closed.
client_loop: send disconnect: Broken pipe
~/clones/swarm-runtime $