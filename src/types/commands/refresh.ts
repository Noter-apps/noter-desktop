import { invoke } from "@tauri-apps/api";
import { FilePreview } from "../filePreview/filePreview";
import { Metadata } from "../metadata";
import { Directory } from "../files/directory";

export default async function refresh() {
  return invoke<[Metadata, FilePreview[], Directory]>("refresh", {});
}
