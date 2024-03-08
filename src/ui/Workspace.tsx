import { useNoterState } from "@/state/state";
import { File } from "@/types/file";
import Editor from "./Editor";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { File as FileIcon, XCircle } from "lucide-react";
import { EditorState } from "lexical";
import { $convertToMarkdownString, TRANSFORMERS } from "@lexical/markdown";
import putFile from "@/types/commands/putFile";
import { useState } from "react";
import { Input } from "@/components/ui/input";

function TabRow() {
  const [selected, removeSelected, open, setOpen] = useNoterState((state) => [
    state.selected,
    state.removeSelected,
    state.open,
    state.setOpen,
  ]);

  function handleOpen(file: File) {
    setOpen(file.id);
  }

  function handleClose(file: File) {
    removeSelected(file.id);
  }

  if (selected.length === 0) {
    return null;
  }

  return (
    <>
      <div className="flex items-center gap-1 p-1">
        {selected.map((file) => (
          <Button
            variant={open?.id.id === file.id.id ? "secondary" : "ghost"}
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

function FileView({ rename }: { rename: (name: string) => void }) {
  const [open, selected] = useNoterState((state) => [
    state.open,
    state.selected,
  ]);

  if (!open) {
    return null;
  }

  const [name, setName] = useState(open.name);

  function onSave(editorState: EditorState) {
    if (!open) {
      return;
    }

    const body = editorState.read(() => {
      return $convertToMarkdownString(TRANSFORMERS);
    });

    putFile(open.id, {
      body,
    });
  }

  function onRename() {
    if (!open) {
      return;
    }

    if (!name || name === open.name) {
      setName(open.name);
      return;
    }

    if (name.length > 255) {
      setName(open.name);
      return;
    }

    if (name.includes("/")) {
      setName(open.name);
      return;
    }

    rename(name);
  }

  return (
    <div className="w-full h-full max-h-full">
      {selected.map((selectedFile) =>
        selectedFile.id.id === open.id.id ? (
          <div
            className="w-full h-full overflow-y-scroll overflow-x-hidden"
            key={selectedFile.id.id}
          >
            <Input
              className="max-w-full tracking-tight lg:text-4xl text-3xl font-bold p-2 border-none rounded-none lg:max-w-2xl mx-auto"
              contentEditable
              value={name}
              onChange={(e) => setName(e.target.value)}
              onBlur={onRename}
            />
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
  const [getDirectory, open, setOpen, removeSelected] = useNoterState(
    (state) => [
      state.getDirectory,
      state.open,
      state.setOpen,
      state.removeSelected,
    ],
  );

  async function rename(name: string) {
    await removeSelected(open.id);
    const renamedFile = await putFile(open.id, { name });
    await getDirectory();
    await setOpen(renamedFile.id);
  }

  return (
    <div className="w-full h-full flex flex-col">
      <TabRow />
      <div className="grow overflow-hidden">
        {open ? (
          <FileView key={open.id.id} rename={rename} />
        ) : (
          <div className="w-full h-full flex items-center justify-center text-muted-foreground">
            Select a file to view
          </div>
        )}
      </div>
    </div>
  );
}
