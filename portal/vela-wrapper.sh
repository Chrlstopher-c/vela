#!/usr/bin/env sh
# Appelé par xdg-desktop-portal-termfilechooser : ouvre Vela en mode sélecteur ; Vela écrit les chemins choisis
# dans $out (vide = annulé).
multiple="$1" directory="$2" save="$3" path="$4" out="$5"
mode=open
if [ "$save" = 1 ]; then mode=save
elif [ "$directory" = 1 ]; then mode=directory
elif [ "$multiple" = 1 ]; then mode=multiple
fi
[ -n "$path" ] || path="$HOME"
exec env WEBKIT_DISABLE_DMABUF_RENDERER=1 /usr/local/bin/vela-bin --pick "$mode" "$path" "$out"
