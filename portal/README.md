# Vela comme sélecteur de fichiers (Discord, navigateurs, Ctrl+O…)

Passe par `xdg-desktop-portal-termfilechooser` (AUR, fork hunkyburrito), qui lance `vela-wrapper.sh` ;
Vela s'ouvre en mode sélecteur (`vela-bin --pick <open|multiple|directory|save> <chemin> <sortie>`) et écrit les
chemins choisis dans le fichier de sortie.

- `vela-wrapper.sh`, `config` → `~/.config/xdg-desktop-portal-termfilechooser/`
- `hyprland-portals.conf` → `~/.config/xdg-desktop-portal/`
- puis `systemctl --user restart xdg-desktop-portal`
- Firefox : `about:config` → `widget.use-xdg-desktop-portal.file-picker` = 1
- Hyprland : `windowrule = match:title ^(Vela · sélection)$, float on, size 1100 720, center on`
