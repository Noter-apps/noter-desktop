import { Directory } from "@/types/directory";
import { File } from "@/types/file";
import { Id } from "@/types/id";
import { invoke } from "@tauri-apps/api";
import { create } from "zustand";
import readDirectoryCommand from "@/types/commands/getDirectory";
import getFileCommand from "@/types/commands/getFile";

type Fields = {
  directory: Directory;
  selected: File[];
  open: File | null;
  isSidebarOpen: boolean;
};

type Actions = {
  getDirectory: () => Promise<Fields["directory"]>;
  setSelected: (files: Fields["selected"]) => Promise<Fields["selected"]>;
  addSelected: (id: Id) => Promise<Fields["selected"]>;
  removeSelected: (id: Id) => Promise<Fields["selected"]>;
  setOpen: (id: Id | null) => Promise<Fields["open"]>;
  toggleSidebar: () => void;
};

const dir = await invoke<Directory>("get_directory", { id: "" });

export const useNoterState = create<Fields & Actions>((set, get) => ({
  directory: dir,
  selected: [],
  open: null,
  isSidebarOpen: true,
  getDirectory: async (id?: Id) => {
    const directory = await readDirectoryCommand(id);
    set({ directory });
    return directory;
  },
  setSelected: async (files) => {
    set({ selected: files });
    return files;
  },
  addSelected: async (input) => {
    const selected = get().selected;

    if (selected.some((file) => file.id.id === input.id)) {
      return selected;
    }

    const file = await getFileCommand(input);
    const files: Fields["selected"] = [...selected, file];

    set({ selected: files });
    return files;
  },
  removeSelected: async (input) => {
    const { selected, open } = get();
    const files = selected.filter((file) => file.id.id !== input.id);

    if (files.length === 0 || input.id === open?.id.id) {
      set({ open: null });
    }

    set({ selected: files });
    return files;
  },
  setOpen: async (input) => {
    const addSelected = get().addSelected;

    if (input === null) {
      set({ open: null });
      return null;
    }

    const selected = await addSelected(input);

    const file = selected.find((file) => file.id.id === input.id);

    if (!file) {
      return null;
    }

    set({ open: file });
    return file;
  },
  toggleSidebar: () => {
    set({ isSidebarOpen: !get().isSidebarOpen });
  },
}));
