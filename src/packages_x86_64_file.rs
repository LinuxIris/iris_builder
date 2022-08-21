pub fn build(packages: &Vec<String>) -> String {
    format!("{}\n{}",PACKAGES_X86_64_FILE, packages.join("\n"))
}

pub const PACKAGES_X86_64_FILE: &'static str = "alsa-utils
amd-ucode
arch-install-scripts
archinstall
b43-fwcutter
base
bind
brltty
broadcom-wl
btrfs-progs
clonezilla
cloud-init
cryptsetup
darkhttpd
ddrescue
dhclient
dhcpcd
diffutils
dmidecode
dmraid
dnsmasq
dosfstools
e2fsprogs
edk2-shell
efibootmgr
espeakup
ethtool
exfatprogs
f2fs-tools
fatresize
fsarchiver
gnu-netcat
gpart
gpm
gptfdisk
grml-zsh-config
grub
hdparm
hyperv
intel-ucode
ipw2100-fw
ipw2200-fw
irssi
iw
iwd
jfsutils
kitty-terminfo
less
lftp
libfido2
libusb-compat
linux
linux-atm
linux-firmware
linux-firmware-marvell
livecd-sounds
lsscsi
lvm2
lynx
man-db
man-pages
mc
mdadm
memtest86+
mkinitcpio
mkinitcpio-archiso
mkinitcpio-nfs-utils
modemmanager
mtools
nano
nbd
ndisc6
nfs-utils
nilfs-utils
nmap
ntfs-3g
nvme-cli
open-iscsi
open-vm-tools
openconnect
openssh
openvpn
partclone
parted
partimage
pcsclite
ppp
pptpclient
pv
qemu-guest-agent
refind
reflector
reiserfsprogs
rp-pppoe
rsync
rxvt-unicode-terminfo
screen
sdparm
sg3_utils
smartmontools
sof-firmware
squashfs-tools
sudo
syslinux
systemd-resolvconf
tcpdump
terminus-font
testdisk
tmux
tpm2-tss
udftools
usb_modeswitch
usbmuxd
usbutils
vim
virtualbox-guest-utils-nox
vpnc
wireless-regdb
wireless_tools
wpa_supplicant
wvdial
xfsprogs
xl2tpd
zsh


# everything above comes from archiso releng folder
#######################################################
###                    ALCI                         ###
#######################################################

#######################################################
###  PACKAGES THAT ARE REMOVED AFTER INSTALLATION   ###
###             VIA CALAMARES CONFIG                ###
#######################################################

alci-dwm
#alci-dwm-nemesis
alci-calamares
#alci-calamares-dev
#alci-calamares-config
#alci-calamares-config-btrfs
#alci-calamares-config-dev
#alci-calamares-config-hardened
#alci-calamares-config-lts
#alci-calamares-config-pure
#alci-calamares-config-xanmod-edge
#alci-calamares-config-zen
xterm


sddm
bash-completion
dex
libxinerama
make
xorg-xkill
xterm
xorg-xrdb

#######################################################
###      PACKAGES THAT STAY AFTER INSTALLATION      ###
#######################################################

os-prober
#virtualbox-guest-utils
mkinitcpio-openswap
git

#######################################################
###                   USER PACKAGES                 ###
#######################################################";
