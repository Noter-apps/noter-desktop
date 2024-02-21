export type Id = {
    id: string;
    notes_dir: string;
};

export function getParentId(id: Id): Id {
    return {
        id: id.id.split("/").slice(0, -1).join("/"),
        notes_dir: id.notes_dir,
    };
}
