import { useNoterState } from "@/state/state";
import { PanelLeftClose, PanelLeftOpen } from "lucide-react";

export default function Titlebar() {
  const [isSidebarOpen, toggleSidebar] = useNoterState((state) => [
    state.isSidebarOpen,
    state.toggleSidebar,
  ]);

  return (
    <div className={`ml-16 h-7 w-full flex items-center`}>
      <div
        className="p-1 rounded-sm cursor-pointer hover:bg-secondary"
        onClick={toggleSidebar}
      >
        {isSidebarOpen ? (
          <PanelLeftClose className="w-4 h-4" />
        ) : (
          <PanelLeftOpen className="w-4 h-4" />
        )}
      </div>
    </div>
  );
}
