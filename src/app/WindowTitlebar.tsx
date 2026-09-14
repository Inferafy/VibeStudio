import { CodeXml, Minus, Square, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { Button } from "@/shared/components/ui/button";

type WindowAction = "close" | "minimize" | "toggleMaximize";

function isTauriWindow(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function runWindowAction(action: WindowAction): Promise<void> {
  if (!isTauriWindow()) return;
  await getCurrentWindow()[action]();
}

function requestWindowAction(action: WindowAction): void {
  void runWindowAction(action).catch((error: unknown) => {
    console.error(`Window action ${action} failed`, error);
  });
}

export function WindowTitlebar() {
  return (
    <header className="projectbar" data-tauri-drag-region>
      <div className="brand" data-tauri-drag-region>
        <span className="brand-mark" aria-hidden="true">
          <CodeXml />
        </span>
        <strong className="brand-name">VibeStudio</strong>
      </div>

      <div className="projectbar-drag-region" data-tauri-drag-region />

      <div className="window-controls" aria-label="窗口控制">
        <Button
          className="window-control"
          variant="quiet"
          size="icon"
          title="最小化"
          aria-label="最小化"
          onClick={() => requestWindowAction("minimize")}
        >
          <Minus aria-hidden="true" />
        </Button>
        <Button
          className="window-control"
          variant="quiet"
          size="icon"
          title="最大化或还原"
          aria-label="最大化或还原"
          onClick={() => requestWindowAction("toggleMaximize")}
        >
          <Square aria-hidden="true" />
        </Button>
        <Button
          className="window-control window-close"
          variant="quiet"
          size="icon"
          title="关闭"
          aria-label="关闭"
          onClick={() => requestWindowAction("close")}
        >
          <X aria-hidden="true" />
        </Button>
      </div>
    </header>
  );
}
