import { useNoterState } from "@/state/state";
import { File } from "@/types/files/file";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { File as FileIcon, XCircle } from "lucide-react";
import { useEffect, useState } from "react";
import getFile from "@/types/commands/getFile";
import NoteView from "./Note";
import { FilePreview } from "@/types/filePreview/filePreview";
import GraphView from "./GraphView";

function TabRow() {
  const [selected, removeSelected, open, setOpen] = useNoterState((state) => [
    state.selected,
    state.removeSelected,
    state.open,
    state.setOpen,
  ]);

  function handleOpen(file: FilePreview) {
    setOpen(file.id);
  }

  function handleClose(file: FilePreview) {
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
            variant={open?.id === file.id ? "secondary" : "ghost"}
            className="flex gap-1 grow justify-between p-0 overflow-hidden"
            key={file.id}
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
  const open = useNoterState((state) => state.open);

  if (!open) {
    return null;
  }

  const [file, setFile] = useState<File | null>(null);

  useEffect(() => {
    if (open) {
      getFile(open.id).then((file) => {
        setFile(file);
      });
    }
  }, [open]);

  if (!file) {
    return null;
  }

  return (
    <div className="w-full h-full max-h-full">
      {file.content.Note && <NoteView file={file} />}
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
          <FileView key={open.id} />
        ) : (
          <div className="w-full h-full flex items-center justify-center"></div>
        )}
      </div>
    </div>
  );
}
