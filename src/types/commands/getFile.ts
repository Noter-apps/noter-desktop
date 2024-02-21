import { invoke } from "@tauri-apps/api";
import { File } from "../file";
import { Id } from "../id";

export default async function getFile(id: Id): Promise<File> {
    const file = invoke<File>("get_file", {
        id,
    });

    return file;
}
