import { invoke } from "@tauri-apps/api";
import { FileType } from "../file";
import { File } from "../file";
import { Id } from "../id";

export default async function createFile(
    parentId: Id | null,
    name: string,
    fileType: FileType,
) {
    return invoke<File>("create_file", {
        parentId: parentId ? parentId.id : "",
        name,
        fileType,
    });
}
