import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import putFile from "@/types/commands/putFile";
import { File } from "@/types/files/file";
import { Note } from "@/types/files/note";
import { useEffect, useRef, useState } from "react";

function TempEditor({
  note,
  onSave,
}: {
  note: Note;
  onSave: (body: string) => void;
}) {
  const [body, setBody] = useState(note.body);

  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  function onChange() {
    if (timeoutRef.current) {
      clearInterval(timeoutRef.current);
    }

    timeoutRef.current = setTimeout(() => {
      onSave(body);
    }, 1000);
  }

  useEffect(() => {
    onChange();

    return () => {
      if (timeoutRef.current) {
        clearInterval(timeoutRef.current);
      }
    };
  }, [body]);

  return (
    <textarea
      className="w-full h-full p-2 outline-none"
      value={body}
      onChange={(e) => setBody(e.target.value)}
    />
  );
}

export default function NoteView({ file }: { file: File }) {
  if (!file.content.Note) {
    return null;
  }

  const content = file.content.Note;
  const [name, setName] = useState(open.name);

  function onSave(body: string) {
    putFile(file.id, {
      body,
    });
  }

  function onRename() {
    if (!name || name === file.name) {
      setName(file.name);
      return;
    }

    if (name.length > 255) {
      setName(file.name);
      return;
    }

    if (name.includes("/")) {
      setName(file.name);
      return;
    }
  }
  return (
    <div
      className="w-full h-full overflow-y-scroll overflow-x-hidden"
      key={file.id.id}
    >
      <Input
        className="max-w-full tracking-tight lg:text-4xl text-3xl font-bold p-2 border-none rounded-none lg:max-w-2xl mx-auto"
        contentEditable
        value={name}
        onChange={(e) => setName(e.target.value)}
        onBlur={onRename}
      />
      <Separator className="mx-2" />
      <TempEditor note={content} onSave={onSave} />
    </div>
  );
}
