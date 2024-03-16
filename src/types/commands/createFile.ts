import { invoke } from "@tauri-apps/api";
import { Id } from "../id";
import { File } from "../files/file";
import { FileType } from "../files/file";

export default async function createFile(
  parentId: Id | null,
  name: string,
  fileType: FileType,
) {
  return invoke<File>("create_file", {
    parentId: parentId ? parentId : "",
    name,
    fileType,
  });
}
