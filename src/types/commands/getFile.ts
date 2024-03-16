import { invoke } from "@tauri-apps/api";
import { File } from "../files/file";
import { Id } from "../id";

export default async function getFile(id: Id): Promise<File> {
  return invoke<File>("get_file", { id });
}
