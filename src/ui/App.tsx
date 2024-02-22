import { useTheme } from "@/components/theme-provider";
import { useNoterState } from "@/state/state";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";
import Sidebar from "./Sidebar";
import Titlebar from "./Titlebar";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import Workspace from "./Workspace";

function App() {
  const { setTheme } = useTheme();
  const isSidebarOpen = useNoterState((state) => state.isSidebarOpen);

  useEffect(() => {
    async function themeListener() {
      const unlisten = await appWindow.onThemeChanged(({ payload: theme }) => {
        setTheme(theme);
      });

      return unlisten;
    }

    themeListener();
  }, []);

  return (
    <div className="flex flex-col w-full h-screen select-none">
      <Titlebar />
      <ResizablePanelGroup direction="horizontal" className="grow">
        {isSidebarOpen && (
          <ResizablePanel minSize={10} defaultSize={20} order={0} id="panel-1">
            <Sidebar />
          </ResizablePanel>
        )}
        <ResizableHandle className="outline hover:outline-2 outline-0 outline-secondary" />
        <ResizablePanel minSize={20} order={1} id="panel-2">
          <Workspace />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
