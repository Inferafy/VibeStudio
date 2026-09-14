import {
  FileCode2,
  MessageSquare,
  PanelsTopLeft,
  Sparkles,
} from "lucide-react";
import { useState } from "react";

import { WindowTitlebar } from "@/app/WindowTitlebar";
import { SessionSidebar } from "@/sessions/list/SessionSidebar";

import "@/app/app-shell.css";

export function App() {
  const [isSidebarCollapsed, setSidebarCollapsed] = useState(false);

  return (
    <div className="workbench" data-sidebar-collapsed={isSidebarCollapsed}>
      <WindowTitlebar />

      <nav className="viewbar" aria-label="项目视图">
        <div className="view-tabs">
          <button className="view-tab" type="button" disabled>
            <FileCode2 aria-hidden="true" />
            <span>项目视图</span>
          </button>
          <button
            className="view-tab is-active"
            type="button"
            aria-current="page"
          >
            <Sparkles aria-hidden="true" />
            <span>Vibe Coding</span>
          </button>
        </div>
      </nav>

      <div className="workspace">
        <SessionSidebar
          collapsed={isSidebarCollapsed}
          onToggle={() => setSidebarCollapsed((current) => !current)}
        />

        <main className="main-workspace" aria-label="VibeStudio 会话工作区">
          <div className="content-toolbar">
            <nav className="content-tabs" aria-label="工作区内容">
              <button
                className="content-tab is-active"
                type="button"
                aria-current="page"
              >
                会话
              </button>
              <button className="content-tab" type="button" disabled>
                画布
              </button>
            </nav>

            <div className="content-mode" aria-hidden="true">
              <PanelsTopLeft />
            </div>
          </div>

          <section className="workspace-empty" aria-label="未选择会话">
            <MessageSquare aria-hidden="true" />
            <p>未选择会话</p>
          </section>
        </main>
      </div>
    </div>
  );
}
