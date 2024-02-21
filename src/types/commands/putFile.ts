import { invoke } from "@tauri-apps/api";
import { File } from "../file";
import { Id } from "../id";

export default async function putFile(
    id: Id,
    args: {
        name?: string;
        body?: string;
    },
) {

    return invoke<File>("put_file", {
        id: id.id,
        name: args.name,
        content: args.body,
    });
}
