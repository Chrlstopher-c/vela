// Mode sélecteur (portail de fichiers) : configuration reçue du binaire et réponse au portail.
import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export type PickerMode = "open" | "multiple" | "directory" | "save";
export interface PickerConfig { mode: PickerMode; path: string; }

export interface Picker {
  config: PickerConfig | null;
  finish: (paths: string[]) => void;
}

export function usePicker(): Picker {
  const [config, setConfig] = useState<PickerConfig | null>(null);
  useEffect(() => {
    invoke<PickerConfig | null>("picker_config")
      .then(setConfig)
      .catch((e: unknown) => console.error("[picker] configuration illisible", e));
  }, []);
  const finish = useCallback((paths: string[]) => {
    invoke("picker_finish", { paths }).catch((e: unknown) => console.error("[picker] réponse impossible", e));
  }, []);
  return { config, finish };
}
