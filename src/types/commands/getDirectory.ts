import { invoke } from "@tauri-apps/api";
import { Id } from "../id";
import { Directory } from "../files/directory";

export default async function getDirectory(id?: Id) {
  return invoke<Directory>("get_directory", {
    id: id ? id : "",
  });
}
