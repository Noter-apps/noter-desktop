import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import Welcome from "./ui/Welcome";
import { ThemeProvider } from "./components/theme-provider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
      <Welcome />
    </ThemeProvider>
  </React.StrictMode>,
);
