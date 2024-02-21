import { invoke } from "@tauri-apps/api";
import { Id } from "../id";

export default async function deleteFile(id: Id) {
    return invoke("delete_file", {
        id,
    });
}
