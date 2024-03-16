import { Id } from "@/types/id";
import { create } from "zustand";
import { FilePreview } from "@/types/filePreview/filePreview";
import { Metadata } from "@/types/metadata";
import refresh from "@/types/commands/refresh";
import { File } from "@/types/files/file";
import getFile from "@/types/commands/getFile";
import { Directory } from "@/types/files/directory";

type Fields = {
  files: FilePreview[];
  dir: Directory;
  selected: FilePreview[];
  open: FilePreview | null;
  isSidebarOpen: boolean;
  metadata: Metadata;
};

type Actions = {
  setSelected: (files: Fields["selected"]) => Fields["selected"];
  addSelected: (id: Id) => Fields["selected"];
  removeSelected: (id: Id) => Fields["selected"];
  setOpen: (id: Id | null) => Promise<File | null>;
  toggleSidebar: () => void;
  refresh: () => Promise<[Metadata, FilePreview[]]>;
};

const [metadata, files, dir] = await refresh();

export const useNoterState = create<Fields & Actions>((set, get) => ({
  metadata,
  files,
  dir,
  selected: [],
  open: null,
  isSidebarOpen: true,
  setSelected: (files) => {
    set({ selected: files });
    return files;
  },
  addSelected: (input) => {
    const { files, selected } = get();
    if (selected.some((file) => file.id === input)) {
      return selected;
    }

    const file = files.find((file) => file.id === input);

    if (!file) {
      return selected;
    }

    const newSelected = [...selected, file];

    set({ selected: newSelected });
    return newSelected;
  },
  removeSelected: (input) => {
    const { selected, open } = get();
    const files = selected.filter((file) => file.id !== input);

    if (files.length === 0 || input === open?.id) {
      set({ open: null });
    }

    set({ selected: files });
    return files;
  },
  setOpen: async (input) => {
    const { addSelected } = get();

    if (input === null) {
      set({ open: null });
      return null;
    }

    const selected = addSelected(input);

    const filePreview = selected.find((file) => file.id === input);
    const file = await getFile(input);

    if (!file || !filePreview) {
      return null;
    }

    set({ open: filePreview });
    return file;
  },
  toggleSidebar: () => {
    set({ isSidebarOpen: !get().isSidebarOpen });
  },
  refresh: async () => {
    const [metadata, files] = await refresh();
    set({ metadata, files });
    return [metadata, files];
  },
}));
