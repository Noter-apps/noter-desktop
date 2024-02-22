import { Button } from "@/components/ui/button";

import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { useNoterState } from "@/state/state";
import { Directory } from "@/types/directory";
import { File } from "@/types/file";
import {
  ChevronRight,
  FileText,
  TextCursor,
  Trash,
  File as FileIcon,
} from "lucide-react";
import { useState } from "react";
import CreateNote from "./CreateNote";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";
import putFile from "@/types/commands/putFile";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";

function FileView({ file }: { file: File }) {
  const [name, setName] = useState(file.name);
  const [getDirectory, open, setOpen, addSelected] = useNoterState((state) => [
    state.getDirectory,
    state.open,
    state.setOpen,
    state.addSelected,
  ]);

  async function handleOpen() {
    await setOpen(file.id);
  }

  async function handleOpenInNewTab() {
    await addSelected(file.id);
  }

  async function handleRename() {
    if (!open || !name || name === open.name) {
      return;
    }

    if (name.length > 255) {
      return;
    }

    if (name.includes("/")) {
      return;
    }
    await putFile(file.id, { name });
    await getDirectory();
  }

  return (
    <ContextMenu>
      <Dialog>
        <ContextMenuTrigger asChild>
          <Button
            variant="ghost"
            className={
              "flex items-center justify-normal gap-1 p-1 w-full h-full " +
              (open?.id.id === file.id.id ? " bg-secondary" : "")
            }
            onClick={handleOpen}
          >
            <FileIcon className="w-4 h-4 text-muted-foreground" />
            {file.name}
          </Button>
        </ContextMenuTrigger>
        <ContextMenuContent className="flex flex-col">
          <ContextMenuItem className="flex gap-2 text-sm" onClick={handleOpen}>
            <FileText className="w-3 h-3" />
            Open
          </ContextMenuItem>
          <ContextMenuItem
            className="flex gap-2 text-sm"
            onClick={handleOpenInNewTab}
          >
            <FileText className="w-3 h-3" />
            Open in New Tab
          </ContextMenuItem>
          <DialogTrigger asChild>
            <ContextMenuItem className="flex gap-2 text-sm">
              <TextCursor className="w-3 h-3" />
              Rename
            </ContextMenuItem>
          </DialogTrigger>
          <ContextMenuSeparator />
          <ContextMenuItem className="flex gap-2 text-sm">
            <Trash className="w-3 h-3" />
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
        <DialogContent>
          <DialogHeader>Rename</DialogHeader>
          <div>
            <Label htmlFor="name">Name</Label>
            <Input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="default" onClick={handleRename}>
                Rename
              </Button>
            </DialogClose>
            <DialogClose asChild>
              <Button variant="ghost">Cancel</Button>
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </ContextMenu>
  );
}

function DirectoryView({ dir }: { dir: Directory }) {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div className="w-full h-full">
      <Button
        variant="ghost"
        className="rounded flex justify-between gap-1 w-full h-full hover:bg-secondary p-1"
        onClick={() => setIsOpen(!isOpen)}
      >
        <div className="flex items-center truncate">
          <ChevronRight
            className={`w-4 h-4 transform duration-100 text-muted-foreground ${isOpen && "rotate-90"}`}
          />
          {dir.name}
        </div>
        <div className="opacity-0 group-hover:opacity-100 transition ease-in-out duration-300 text-muted-foreground px-2">
          {dir.entries.length}
        </div>
      </Button>
      {isOpen && (
        <div className="flex">
          <div className="mx-3 outline outline-1 outline-secondary rounded-full"></div>
          <div className="w-full h-full">
            {dir.entries.map((entry) => {
              if (entry.Directory) {
                const dir = entry.Directory;
                return <DirectoryView key={dir.id.id} dir={dir} />;
              } else if (entry.File) {
                const file = entry.File;
                return <FileView key={file.id.id} file={file} />;
              }
            })}
          </div>
        </div>
      )}
    </div>
  );
}

export default function Sidebar() {
  const dir = useNoterState((state) => state.directory);

  return (
    <div className="w-full h-full flex flex-col">
      <div className="flex justify-end p-1">
        <CreateNote />
      </div>
      <Separator />
      <ScrollArea className="grow cursor-default group p-2">
        <DirectoryView dir={dir} />
      </ScrollArea>
    </div>
  );
}
