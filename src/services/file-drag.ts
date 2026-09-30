// Données d'un glisser de fichier : type interne (déplacement dans Vela) + text/uri-list pour les autres applis
// (île Echo, Discord, navigateur, terminal).
export function setFileDragData(dt: DataTransfer, path: string): void {
  const uri = "file://" + path.split("/").map(encodeURIComponent).join("/");
  dt.setData("application/vela", path);
  dt.setData("text/uri-list", uri);
  dt.setData("text/plain", path);
  dt.effectAllowed = "copyMove";
}
