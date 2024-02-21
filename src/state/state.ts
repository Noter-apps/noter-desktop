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
    addSelected: (id: Id | File) => Promise<Fields["selected"]>;
    removeSelected: (id: Id | File) => Promise<Fields["selected"]>;
    setOpen: (id: Id | File | null) => Promise<Fields["open"]>;
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
        let file: File;

        if (typeof input.id === "string") {
            const id = input as Id;

            if (selected.some((file) => file.id.id === id.id)) {
                return selected;
            }

            file = await getFileCommand(id);
        } else {
            file = input as File;

            if (
                selected.some(
                    (selectedFile) => file.id.id === selectedFile.id.id,
                )
            ) {
                return selected;
            }
        }

        const files: Fields["selected"] = [...selected, file];
        set({ selected: files });
        return files;
    },
    removeSelected: async (input) => {
        let id: Id;
        if (typeof input.id === "string") {
            id = input as Id;
        } else {
            const file = input as File;
            id = file.id;
        }

        const selected = get().selected;
        const files = selected.filter((file) => file.id.id !== id.id);
        if (files.length === 0) {
            set({ open: null });
        }
        set({ selected: files });
        return files;
    },
    setOpen: async (input) => {
        if (input === null) {
            set({ open: null });
            return null;
        }

        let file: Fields["open"] = null;

        if (typeof input.id === "string") {
            const id = input as Id;
            file = await getFileCommand(id);
        } else {
            file = input as File;
        }

        get().addSelected(file);
        set({ open: file });
        return file;
    },
    toggleSidebar: () => {
        set({ isSidebarOpen: !get().isSidebarOpen });
    },
}));
