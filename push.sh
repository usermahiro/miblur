#!/bin/sh
# Jalankan dari folder hasil ekstrak zip artifact (butuh root, partisi system_ext rw)
# Bisa langsung di device (su) atau lewat adb: adb push folder ini ke /data/local/tmp lalu su -c sh push.sh
D="$(cd "$(dirname "$0")" && pwd)"
BIN=/system_ext/bin/smoothcornerd
RC=/system_ext/etc/init/init.hyperlagos.smoothcorner.rc

mount -o rw,remount /system_ext 2>/dev/null || mount -o rw,remount / 2>/dev/null

cp "$D/system_ext/bin/smoothcornerd" $BIN || exit 1
cp "$D/system_ext/etc/init/init.hyperlagos.smoothcorner.rc" $RC || exit 1

chown root:root $BIN $RC
chmod 0755 $BIN
chmod 0644 $RC
chcon u:object_r:system_file:s0 $BIN $RC

ls -lZ $BIN $RC
echo "Selesai. Reboot, lalu cek: getprop persist.sys.support_view_smoothcorner"
