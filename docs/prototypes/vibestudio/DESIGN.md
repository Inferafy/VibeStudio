---
name: VibeStudio Prototype Workspace
description: User-pinned Codeg/Codex desktop workspace language for a static visual prototype.
colors:
  surface: "#ffffff"
  chrome: "#fafafa"
  sidebar: "#f6f6f6"
  surface-subtle: "#f8f8f8"
  surface-hover: "#ededed"
  surface-selected: "#e8e8e8"
  border: "#e5e5e5"
  border-control: "#d1d1d1"
  text: "#202020"
  text-secondary: "#575757"
  text-muted: "#6b6b6b"
  text-disabled: "#8e8e8e"
  focus: "#3768c9"
  focus-subtle: "#f3f6fc"
  success: "#34724a"
  success-surface: "#edf6ef"
  warning: "#875c1b"
  warning-surface: "#fcf7eb"
  danger: "#b54141"
  danger-surface: "#fcf1f1"
  claude: "#a35940"
  claude-surface: "#f8eee9"
  code-keyword: "#2b599e"
  code-string: "#337149"
  code-type: "#8c5123"
typography:
  title:
    fontFamily: "Segoe UI, Microsoft YaHei, system-ui, sans-serif"
    fontSize: "16px"
    fontWeight: 600
    lineHeight: 1.5
    letterSpacing: "0"
  body:
    fontFamily: "Segoe UI, Microsoft YaHei, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.55
    letterSpacing: "0"
  label:
    fontFamily: "Segoe UI, Microsoft YaHei, system-ui, sans-serif"
    fontSize: "13px"
    fontWeight: 500
    lineHeight: 1.55
    letterSpacing: "0"
  small:
    fontFamily: "Segoe UI, Microsoft YaHei, system-ui, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: "0"
  code:
    fontFamily: "Cascadia Code, Consolas, Liberation Mono, monospace"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.8
    letterSpacing: "0"
rounded:
  control: "6px"
  composer: "8px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
  xxl: "32px"
  section: "40px"
components:
  button-primary:
    backgroundColor: "{colors.text}"
    textColor: "{colors.surface}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "4px 12px"
    height: "32px"
  button-quiet:
    backgroundColor: "transparent"
    textColor: "{colors.text}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "4px 12px"
    height: "32px"
  search-field:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "0 8px"
    height: "32px"
  composer:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.composer}"
    padding: "12px"
  session-panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    typography: "{typography.label}"
    rounded: "0"
    padding: "12px 16px"

---

# Design System: VibeStudio Prototype Workspace

## Overview

**Creative North Star: "用户钉选的 Codeg/Codex 桌面工作区"**

这是一个静态原型中已经落地的桌面工作区语言：白色内容区域承载会话阅读，中性灰色外壳组织项目、模式和会话列表，细分隔线维持清楚的边界。界面密度偏实用，文字、路径和代码都服务于扫描与比较。

单会话、2x2 网格和 3x3 网格共用同一组基础样式。状态色只在审批、错误、完成和焦点等语义位置出现；模型标识和会话内容均为合成展示内容。该文档记录原型现状，不能作为生产设计批准。

**Key Characteristics:**
- 白色内容面与中性灰工作台
- 1px 细边界与低装饰密度
- 系统 UI 字体配合代码字体
- 受限的蓝色焦点与语义状态色
- 共享的会话面板和固定底部编辑器

## Colors

Palette character is neutral-first: surfaces and borders carry hierarchy, while blue, green, amber, red and Claude brown remain semantic accents.

### Primary
- **工作焦点蓝** (`#3768c9`): 用于键盘焦点轮廓和当前聚焦会话的细边界。

### Secondary
- **完成绿** (`#34724a`): 用于完成状态点、状态文字及对应的浅色底。
- **审批琥珀** (`#875c1b`): 用于需要审批的状态和审批提示。

### Tertiary
- **错误红** (`#b54141`): 用于连接中断等错误状态。
- **Claude 棕** (`#a35940`): 用于 Claude 标识及其浅色标识底。

### Neutral
- **内容白** (`#ffffff`): 主内容、编辑器和按钮底色。
- **工作台灰** (`#fafafa`): 顶部项目条。
- **侧栏灰** (`#f6f6f6`): 会话侧栏、用户消息和部分控件底色。
- **微灰面** (`#f8f8f8`): 代码块与选中内容的轻层次。
- **悬停灰** (`#ededed`): 行和图标控件悬停底色。
- **选中灰** (`#e8e8e8`): 当前项目或会话的选中底色。
- **边界灰** (`#e5e5e5`): 工作区、工具栏和面板分隔线。
- **控件边界** (`#d1d1d1`): 搜索框与编辑器边框。
- **正文黑** (`#202020`): 主要文字和深色主按钮。
- **次级文字** (`#575757`): 次要正文和工具信息。
- **弱化文字** (`#6b6b6b`): 时间、路径、占位文字和辅助说明。
- **禁用文字** (`#8e8e8e`): 禁用标签和不可用模式。

### Named Rules
**The Semantic Accent Rule.** Accent colors appear where they explain state or focus; the workspace remains neutral at rest.

## Typography

**Display Font:** No display face is used. Titles use `Segoe UI` with `Microsoft YaHei` and system fallbacks.
**Body Font:** `Segoe UI`, with `Microsoft YaHei`, `system-ui`, `sans-serif` fallbacks.
**Label/Mono Font:** Labels stay in the UI stack; paths and code use `Cascadia Code`, `Consolas`, `Liberation Mono`, `monospace`.

**Character:** The type system is compact and legible for a Windows desktop workbench. Letter spacing remains `0`; code receives a wider line height for scanning.

### Hierarchy
- **Title** (600, `16px`, `1.5`): session and sidebar headings.
- **Body** (400, `14px`, `1.55`): general workspace text and transcript copy.
- **Label** (500 where emphasis is needed, `13px`): tabs, controls and panel headings.
- **Small** (400, `12px`, `1.2`): metadata, time, state text and helper copy.
- **Code** (400, `13px`, `1.8`): commands, paths, inline code and code blocks.

### Named Rules
**The Two-Voice Rule.** Use the system UI stack for workspace language and the code stack for paths, commands and code.

## Layout

The workbench fills the viewport with three rows: a 44px project bar, a 38px view bar, and the remaining workspace. The desktop workspace reserves a 272px session sidebar, reducing to 232px below 1180px and to a 56px session rail below 640px. The main content stays white.

Single-session reading is centered at a maximum width of 840px. The 2x2 grid uses equal columns and rows with a 312px panel floor. The 3x3 grid uses equal columns and rows at wide sizes, then becomes 2 columns below 1180px and 1 column below 860px. At mobile widths, the project and content toolbars use 12px side padding, the transcript uses 16px side padding, and the session rail remains 56px wide.

The spacing rhythm is 4px, 8px, 12px, 16px, 20px, 24px, 32px and 40px. Fixed controls use 30px or 26px square sizes; icons are 16px. Composer controls wrap into two aligned rows when a panel is narrow. Panels at or below 400px use compact conversation padding and omit the optional approval directory line to keep approval actions and the composer inside the panel.

## Elevation & Depth

The prototype uses flat tonal layering. No `box-shadow` tokens are defined. Depth comes from white content, gray chrome, selected/hover surfaces, 1px separators, and a blue focus outline. The fixed composer is separated by placement and border rather than a floating shadow.

### Named Rules
**The Flat Workspace Rule.** Keep resting surfaces flat; use tone, border and focus treatment to explain hierarchy.

## Shapes

Controls, tabs, rows and panels use restrained geometry. The shared control radius is 6px; the composer uses 8px; the send control is circular. Panels remain square-edged and use 1px right and bottom separators. Overflowing titles, paths and long content are clipped or wrapped according to their role.

## Components

### Buttons
- **Shape:** 6px radius, 32px minimum height, 4px 12px padding.
- **Primary:** `#202020` background, white text, medium label weight; hover shifts to `#353535`.
- **Quiet / default:** white or transparent background with a 1px control border where a button needs a visible boundary; hover uses `#f8f8f8`.
- **Focus / disabled:** focus-visible uses a 1px blue outline with 1px offset; disabled send controls use selected gray and reduced icon opacity.

### Chips / States
- **Style:** 6px status dot and 12px state text; approval, error and done use amber, red and green semantic colors.
- **State:** state labels stay muted for ordinary running status and use semantic color only for the relevant condition.

### Cards / Containers
- **Corner Style:** session panels are square; code blocks and compact containers use 6px.
- **Background:** white panels sit inside a neutral-gray sidebar and are separated by `#e5e5e5`.
- **Shadow Strategy:** no shadow; see Elevation & Depth.
- **Border:** 1px right and bottom panel separators; 1px borders for code blocks.
- **Internal Padding:** panel headings use 8px 12px; panel conversations use 12px 16px.

### Inputs / Fields
- **Style:** search fields use a white background, 1px `#d1d1d1` border, 6px radius and 32px height. Composer fields use a white background, 1px `#d1d1d1` border, 8px radius and a 48px text area in single view.
- **Focus:** focus-visible uses the shared blue 1px outline with 1px offset.
- **Read-only:** prototype textareas and search input show static content affordances; no editable behavior is implied.

### Navigation
- **Style:** the project bar is 44px high with neutral chrome and a 1px bottom border. The view bar is 38px high on white. Active tabs use dark text, medium weight and either a selected gray surface or a 2px dark underline; disabled tabs use `#8e8e8e`.
- **Mobile:** project tabs remain horizontally scrollable; window controls disappear and the session sidebar becomes the 56px rail.

### Session Composer
The composer is anchored at the bottom of the single-session and grid panel columns. Its left group contains the add-context icon, a fixed Agent icon and a permission selector. The Agent icon has a name tooltip, no dropdown arrow and no switching affordance.

The right group contains the model selector, thinking-level selector, context-use pie and circular send/stop control, in that order. Model and thinking-level controls remain visible in every layout. Grid controls reduce to 26px. If both groups cannot fit in one row, the right group wraps onto the next row and stays right-aligned.

The context-use pie is 16px wide, uses muted text and selected-surface colors, and reports the synthetic percentage in its tooltip. Its sectors are drawn in external CSS; the prototype retains its script and inline-style restrictions. Permissions, thinking levels and context percentages are static visual fixtures.

## Do's and Don'ts

### Do:
- **Do** keep content white and let neutral chrome, selected surfaces and 1px borders organize the workbench.
- **Do** use the shared UI and code font stacks, with `0` letter spacing.
- **Do** preserve the 56px mobile session rail and the 312px grid panel floor.
- **Do** keep approval and reconnect blocks above their anchored composers when those states are displayed.
- **Do** keep the Agent icon fixed and permissions alongside it; keep model, thinking level and context usage next to send on the right.
- **Do** reserve blue, green, amber and red for observed focus and semantic state roles.

### Don't:
- **Don't** add shadows, decorative gradients or a second visual language to this prototype. A functional context-use pie may use a conic gradient to show its sectors.
- **Don't** use synthetic conversation text or model labels as product claims.
- **Don't** treat static buttons, read-only fields or disabled controls as functional interactions.
- **Don't** claim a complete contrast or accessibility audit; the detector was degraded because parsers were missing.
- **Don't** extend these provisional prototype rules into production approval without a new review.
