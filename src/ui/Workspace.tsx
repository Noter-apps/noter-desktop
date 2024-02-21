import { useNoterState } from "@/state/state";
import { File } from "@/types/file";
import Editor from "./Editor";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { File as FileIcon, XCircle } from "lucide-react";
import { EditorState } from "lexical";
import { $convertToMarkdownString, TRANSFORMERS } from "@lexical/markdown";
import putFile from "@/types/commands/putFile";

function TabRow() {
    const [selected, removeSelected, open, setOpen] = useNoterState((state) => [
        state.selected,
        state.removeSelected,
        state.open,
        state.setOpen,
    ]);

    function handleOpen(file: File) {
        setOpen(file);
    }

    function handleClose(file: File) {
        removeSelected(file);
    }

    if (selected.length === 0) {
        return null;
    }

    return (
        <>
            <div className="flex items-center gap-1 p-1">
                {selected.map((file) => (
                    <Button
                        variant={
                            open?.id.id === file.id.id ? "secondary" : "ghost"
                        }
                        className="flex gap-1 grow justify-between p-0 overflow-hidden"
                        key={file.id.id}
                    >
                        <div
                            className="h-full flex items-center gap-1 grow p-1"
                            onClick={() => handleOpen(file)}
                        >
                            <FileIcon className="w-4 h-4 text-muted-foreground" />
                            <p>{file.name}</p>
                        </div>

                        <div
                            className="w-6 h-6 p-1 mr-1 group hover:bg-destructive rounded-sm"
                            onClick={() => handleClose(file)}
                        >
                            <XCircle className="w-full h-full group-hover:stroke-destructive-foreground" />
                        </div>
                    </Button>
                ))}
            </div>
            <Separator />
        </>
    );
}

function FileView() {
    const [open, selected] = useNoterState((state) => [
        state.open,
        state.selected,
    ]);

    if (!open) {
        return null;
    }

    function onSave(editorState: EditorState) {
        if (!open) {
            return;
        }

        const body = editorState.read(() => {
            return $convertToMarkdownString(TRANSFORMERS);
        });

        putFile(open.id, {
            body
        });
    }

    return (
        <div className="w-full h-full max-h-full">
            {selected.map((selectedFile) =>
                selectedFile.id.id === open.id.id ? (
                    <div
                        className="w-full h-full overflow-y-scroll"
                        key={selectedFile.id.id}
                    >
                        <h1 className="max-w-full text-2xl font-bold p-2">
                            {selectedFile.name}
                        </h1>
                        <Separator className="mx-2" />
                        <Editor
                            key={selectedFile.id.id}
                            note={selectedFile.content.Note!}
                            onSave={onSave}
                        />
                    </div>
                ) : null,
            )}
        </div>
    );
}

export default function Workspace() {
    const open = useNoterState((state) => state.open);

    return (
        <div className="w-full h-full flex flex-col">
            <TabRow />
            <div className="grow overflow-hidden">
                {open ? (
                    <FileView key={open.id.id} />
                ) : (
                    <div className="w-full h-full flex items-center justify-center text-muted-foreground">
                        Select a file to view
                    </div>
                )}
            </div>
        </div>
    );
}
