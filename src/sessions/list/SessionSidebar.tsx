import { MessageSquare, PanelLeftClose, PanelLeftOpen } from "lucide-react";

import { Button } from "@/shared/components/ui/button";

interface SessionSidebarProps {
  collapsed: boolean;
  onToggle: () => void;
}

export function SessionSidebar({ collapsed, onToggle }: SessionSidebarProps) {
  const toggleLabel = collapsed ? "展开会话栏" : "收起会话栏";
  const ToggleIcon = collapsed ? PanelLeftOpen : PanelLeftClose;

  return (
    <aside className="session-sidebar" aria-label="会话列表">
      <div className="session-sidebar-heading">
        <h2>会话</h2>
        <Button
          className="sidebar-toggle"
          variant="quiet"
          size="smallIcon"
          title={toggleLabel}
          aria-label={toggleLabel}
          aria-controls="session-sidebar-content"
          aria-expanded={!collapsed}
          onClick={onToggle}
        >
          <ToggleIcon aria-hidden="true" />
        </Button>
      </div>

      <div id="session-sidebar-content" className="session-sidebar-body">
        <div className="session-sidebar-empty" role="status">
          <MessageSquare aria-hidden="true" />
          <p>暂无会话</p>
        </div>
      </div>
    </aside>
  );
}
