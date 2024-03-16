import { Folder } from "lucide-react";
import { Button } from "../components/ui/button";
import { OpenDialogOptions } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import croc from "@/assets/croc.webp";
import { appWindow } from "@tauri-apps/api/window";
import { useTheme } from "@/components/theme-provider";
import { useEffect } from "react";

export default function Welcome() {
  const { setTheme } = useTheme();

  useEffect(() => {
    async function themeListener() {
      const unlisten = await appWindow.onThemeChanged(({ payload: theme }) => {
        setTheme(theme);
      });

      return unlisten;
    }

    themeListener();
  }, []);

  async function onClick() {
    const openOpts = {
      title: "Choose a notes folder",
      directory: true,
      multiple: false,
    } satisfies OpenDialogOptions;

    let selected = await open(openOpts);

    if (!selected) {
      return;
    }
    if (Array.isArray(selected)) {
      selected = selected[0];
    }

    await invoke("welcome_startup", { notesDir: selected });
  }

  return (
    <div className="flex flex-col h-screen p-4 pt-16">
      <h1 className="text-4xl font-bold text-center">Welcome to Noter!</h1>
      <div className="flex-1">
        <img src={croc} alt="Croc" className="w-full h-full object-contain" />
      </div>
      <Button
        onClick={onClick}
        className="flex items-center justify-center gap-2 font-medium text-lg h-auto py-3"
      >
        <Folder />
        Choose a folder for your notes
      </Button>
    </div>
  );
}
