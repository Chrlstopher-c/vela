// Barre du mode sélecteur : rappelle ce qui est demandé, nom du fichier à enregistrer, Annuler / valider.
import { useEffect, useState } from "react";
import type { Picker } from "../hooks/usePicker";
import type { DirEntry } from "../types";
import { baseName } from "../services/path-util";

interface Props {
  picker: Picker;
  cwd: string;
  selected: DirEntry[];
}

const TITLES = {
  open: "Choisir un fichier",
  multiple: "Choisir des fichiers",
  directory: "Choisir un dossier",
  save: "Enregistrer sous",
} as const;

export function PickerBar({ picker, cwd, selected }: Props): React.ReactElement | null {
  const mode = picker.config?.mode;
  const [name, setName] = useState("");
  useEffect(() => { if (mode === "save") setName(baseName(picker.config?.path ?? "")); }, [mode, picker.config]);
  if (!mode) return null;

  const files = selected.filter((e) => !e.is_dir);
  const chosen: string[] = mode === "save" ? (name.trim() ? [`${cwd}/${name.trim()}`] : [])
    : mode === "directory" ? [selected.find((e) => e.is_dir)?.path ?? cwd]
    : (mode === "multiple" ? files : files.slice(0, 1)).map((e) => e.path);
  const summary = mode === "save" ? cwd : mode === "directory" ? baseName(chosen[0]) || cwd
    : files.length ? files.map((e) => e.name).join(", ") : "Aucun fichier sélectionné";

  return (
    <div className="shrink-0 flex items-center gap-3 px-4 py-2.5 border-t border-[var(--color-border)] bg-[var(--color-surface)] text-sm select-none">
      <span className="font-semibold text-[var(--color-text)] shrink-0">{TITLES[mode]}</span>
      {mode === "save" ? (
        <input
          autoFocus
          value={name}
          onChange={(e) => setName(e.target.value)}
          onKeyDown={(e) => { if (e.key === "Enter" && chosen.length) picker.finish(chosen); }}
          className="flex-1 min-w-0 px-3 h-8 rounded-md bg-[var(--color-bg)] border border-[var(--color-border)] text-[var(--color-text)] outline-none focus:border-[var(--color-accent)]"
        />
      ) : (
        <span className="flex-1 min-w-0 truncate text-[var(--color-text-dim)]">{summary}</span>
      )}
      <button onClick={() => picker.finish([])} className="px-3 h-8 rounded-md border border-[var(--color-border)] text-[var(--color-text)] hover:bg-[var(--color-bg)]">Annuler</button>
      <button
        onClick={() => picker.finish(chosen)}
        disabled={!chosen.length}
        className="px-4 h-8 rounded-md bg-[var(--color-accent)] text-[var(--color-bg)] font-medium hover:brightness-110 disabled:opacity-50"
      >
        {mode === "save" ? "Enregistrer" : mode === "directory" ? "Choisir ce dossier" : "Ouvrir"}
      </button>
    </div>
  );
}
