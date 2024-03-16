import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useNoterState } from "@/state/state";
import createFile from "@/types/commands/createFile";
import { Id, getParentId } from "@/types/id";
import { FilePlus } from "lucide-react";
import { useState } from "react";

export default function CreateNote() {
  const [name, setName] = useState("");
  const [open, setOpen, refresh] = useNoterState((state) => [
    state.open,
    state.setOpen,
    state.refresh
  ]);

  async function handleCreateNote() {
    if (!name) return;

    let parentId: Id | null = null;
    if (open) {
      parentId = getParentId(open.id);
    }

    const file = await createFile(parentId, name, "Note");
    await refresh();
    await setOpen(file.id);
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="default" className="flex gap-1">
          <FilePlus className="w-4 h-4" /> Create Note
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogTitle>Create New Note</DialogTitle>
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
            <Button variant="ghost">Cancel</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="default" onClick={handleCreateNote}>
              Create
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
