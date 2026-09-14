# Agent 原生命令与工具接入调研

调研日期：2026-09-14。

关联模块：[本地能力层：Agent 原生能力](../模块设计/本地能力层-Agent原生能力.md)、[外部工具层：Agent 接入](../模块设计/外部工具层-Agent接入.md)、[本地能力层：会话运行](../模块设计/本地能力层-会话运行.md)。

当前状态：调研完成；目标版本、Windows 启动和真实能力已由 [M0 双 Agent 接入验证](../验证记录/M0/2026-09-14-双Agent接入验证.md) 完成实测。

## 1. 结论

“让每个 Agent 使用自己的原生命令和工具”需要区分三类能力：

- 原生命令：Agent 或适配器在当前会话中声明的 `/命令`，例如压缩上下文、审查或查看状态。
- 原生工具：Agent 在执行任务时调用的文件、命令、搜索、MCP 等工具，通过结构化事件向宿主报告进度和结果。
- 宿主操作：发送、停止、切换模型、归档会话、退出窗口等客户端行为，应调用协议方法或 VibeStudio 自己的业务操作。

ACP 已提供统一接入骨架：Agent 动态上报可用命令，客户端显示候选；用户选择后，客户端把 `/命令 参数` 放进普通会话请求，由 Agent 或适配器解释。工具调用通过独立事件流报告，审批、终端和配置选项还有各自的协议方法。

由此得到 VibeStudio 的接入原则：

- 当前会话上报什么，界面才展示什么；不能按 Agent 名称维护一张假定永远有效的命令表。
- 适配器负责把通用 ACP 请求转换成 Agent 的 SDK、App Server 或 CLI 行为，VibeStudio 不模拟原生 TUI。
- 终端主题、键位、退出等只服务原生 TUI 的命令不进入 VibeStudio 命令菜单。
- 停止、权限、模型和思考等级优先使用专门协议能力，不能把 `/stop`、`/model` 当普通文字碰碰运气。
- 每个版本组合重新记录能力。适配器升级后，旧能力表不能直接沿用。

## 2. 证据快照

| 项目 | 固定版本 | 本次用途 |
| --- | --- | --- |
| ACP 规范 | [`20591858`](https://github.com/agentclientprotocol/agent-client-protocol/commit/205918585fc99d97aa10b0d3619d5678dfcda712) | 命令、工具、审批和配置协议 |
| VibeX | [`e30cb859`](https://github.com/Xircth/VibeX/commit/e30cb859c63e9436dc61cdc02d21c7756d4788ee) | 多 Agent ACP 宿主实现 |
| Claude ACP 适配器 | [`8823ea6f`](https://github.com/agentclientprotocol/claude-agent-acp/commit/8823ea6fcb6743d37c1aad91ccd44725164b8927) | Claude Code 命令和工具桥接 |
| Codex ACP 适配器 | [`472e60e4`](https://github.com/agentclientprotocol/codex-acp/commit/472e60e4e99234c47c6de67a6d9cc8a71ebda47e) | Codex App Server 到 ACP 的桥接 |
| CodeG | [`18046ea4`](https://github.com/xintaofei/codeg/commit/18046ea4ecc11a68f8f7bac9e54c99b28658dfe6) | 动态命令与会话控件实践 |
| Codex | [`e9633d7`](https://github.com/openai/codex/commit/e9633d7a0226eac91c7a791dc4f92cf8f25df2ae) | 原生 TUI 命令与 App Server 边界 |

OpenAI 官方资料：[Codex 开发者命令](https://learn.chatgpt.com/docs/developer-commands#built-in-slash-commands)、[Codex App Server](https://learn.chatgpt.com/docs/app-server#api-overview)、[审批](https://learn.chatgpt.com/docs/app-server#approvals)、[错误](https://learn.chatgpt.com/docs/app-server#errors)。

## 3. ACP 怎样承载命令和工具

### 3.1 原生命令

ACP 允许 Agent 在会话建立后发送 `available_commands_update`，每条命令带名称、说明和可选参数提示。列表可以在会话运行中再次更新。[ACP 命令规范](https://github.com/agentclientprotocol/agent-client-protocol/blob/205918585fc99d97aa10b0d3619d5678dfcda712/docs/protocol/v1/slash-commands.mdx#L6)

执行命令没有独立的“运行命令”接口。客户端把 `/命令 参数` 作为普通文字放入 `session/prompt`，Agent 识别前缀并处理。[命令执行](https://github.com/agentclientprotocol/agent-client-protocol/blob/205918585fc99d97aa10b0d3619d5678dfcda712/docs/protocol/v1/slash-commands.mdx#L75)

```text
Agent / 适配器
  └─ available_commands_update
       └─ Rust 保存当前会话命令快照
            └─ 输入框键入 / 后显示候选
                 └─ 用户选择并发送 /命令 参数
                      └─ session/prompt
                           └─ Agent / 适配器解释并返回会话事件
```

命令列表是可选能力。没有上报时，VibeStudio 可以允许用户继续输入普通文字，但不能展示未证实的命令菜单或宣称命令可用。

### 3.2 原生工具

工具调用由 Agent 执行，并通过 `tool_call` 和 `tool_call_update` 报告。事件包含调用标识、标题、类型、状态、内容、文件位置以及可选原始输入输出。状态至少包括等待、运行、完成和失败。[ACP 工具规范](https://github.com/agentclientprotocol/agent-client-protocol/blob/205918585fc99d97aa10b0d3619d5678dfcda712/docs/protocol/v1/tool-calls.mdx#L12)

需要权限时，Agent 向客户端发送 `session/request_permission`。按钮及其含义来自请求本身，可包含仅本次允许、长期允许、仅本次拒绝和长期拒绝。回合取消后，客户端必须把仍在等待的审批答复为取消。[权限请求](https://github.com/agentclientprotocol/agent-client-protocol/blob/205918585fc99d97aa10b0d3619d5678dfcda712/docs/protocol/v1/tool-calls.mdx#L108)

工具还可以附带差异、文件位置和终端标识。Agent 负责决定何时调用工具；VibeStudio 负责提供已声明的客户端能力、展示过程并把用户决定送回原会话。

### 3.3 会话配置

ACP 的会话配置项可表达模式、模型、模型参数和思考等级。Agent 提供选项、当前值、顺序及可选分类，客户端通过 `session/set_config_option` 修改；Agent 也能主动推送完整的新状态。[会话配置](https://github.com/agentclientprotocol/agent-client-protocol/blob/205918585fc99d97aa10b0d3619d5678dfcda712/docs/protocol/v1/session-config-options.mdx#L6)

配置项和原生命令需要分别展示。模型、权限或思考等级已有配置选择器时，可以从命令候选中隐藏同义入口，避免两处操作产生不同状态。

## 4. VibeX 的实现路径

VibeX 把 Agent 本体和 ACP 适配器分别登记，并锁定包、版本、启动程序、Node 要求、认证位置和配置位置。固定提交中，Claude Code 使用 `@agentclientprotocol/claude-agent-acp` 0.69.0，Codex 使用 `@agentclientprotocol/codex-acp` 1.7.0；两者都与对应运行依赖成对记录。[内置 Agent 配置](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/crates/agents/src/profiles.rs#L2415)

VibeX 锁定的 Rust `agent-client-protocol` 为 1.2.0。[Cargo.lock](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/Cargo.lock#L46) 这只说明该提交的实现基线，不能直接成为 VibeStudio 的版本结论。

运行链路如下：

1. Rust 连接适配器并声明客户端支持终端、布尔配置和表单提问。[客户端能力](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/crates/agents/src/manager.rs#L1864)
2. `AvailableCommandsUpdate` 被转换成内部命令对象并保存到当前会话状态。[命令映射](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/crates/agents/src/manager.rs#L4242)
3. 前端先读取会话快照，再订阅后续命令更新。[前端订阅](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/frontend/src/features/conversation/useConversationAvailableCommands.ts#L6)
4. 输入框把运行时命令转换成带来源的候选，界面可以给已知命令补图标和友好名称，但不会凭本地目录添加 Agent 未上报的命令。[命令来源](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/frontend/src/lib/conversation-rendering/commandSources.ts#L33)
5. 选中的结构化候选在发送前还原为真实 `/命令` 或 `$技能` 文字，再进入普通会话请求。[发送序列化](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/frontend/src/components/tasks/follow-up/sessionComposerStructuredTokens.ts#L1031)

VibeX 还实现 ACP 的终端创建、输出、等待、停止和释放请求，并把权限请求严格路由到原会话；当前对 ACP 文件读写回调返回“不支持”。[客户端请求处理](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/crates/agents/src/manager.rs#L3692) 这说明客户端能力必须如实声明，不能为了兼容而报告一个尚未实现的接口。

Windows 上的 npm 命令通常是 `.cmd` 或 `.bat`。VibeX 会按 `PATH` 和 `PATHEXT` 解析实际入口，必要时通过隐藏的 `cmd.exe /d /c` 启动；同时避免把批处理路径交给不支持 shell 的 Node 子进程。[Windows 启动](https://github.com/Xircth/VibeX/blob/e30cb859c63e9436dc61cdc02d21c7756d4788ee/crates/utils/src/process.rs#L48)

## 5. 两类适配器怎样保留原生能力

### 5.1 Claude Code

当前 Claude ACP 适配器直接查询 Claude Agent SDK 的 `supportedCommands()`，再发送 ACP 命令更新。[命令获取](https://github.com/agentclientprotocol/claude-agent-acp/blob/8823ea6fcb6743d37c1aad91ccd44725164b8927/src/acp-agent.ts#L7375)

适配器会过滤两类命令：一类依赖 Claude 原生终端，例如 `/doctor`、`/color`；另一类明确不适用于 ACP 客户端，例如登录、退出、键位帮助和发布说明。[终端命令边界](https://github.com/agentclientprotocol/claude-agent-acp/blob/8823ea6fcb6743d37c1aad91ccd44725164b8927/src/acp-agent.ts#L772) [过滤规则](https://github.com/agentclientprotocol/claude-agent-acp/blob/8823ea6fcb6743d37c1aad91ccd44725164b8927/src/acp-agent.ts#L9091)

可用命令仍按普通会话请求进入 Claude。部分本地命令不会产生普通用户消息回显，适配器为此维护额外的命令生命周期和历史清理逻辑。[本地命令处理](https://github.com/agentclientprotocol/claude-agent-acp/blob/8823ea6fcb6743d37c1aad91ccd44725164b8927/src/acp-agent.ts#L2623)

### 5.2 Codex

Codex App Server 提供结构化会话、回合、审批、停止、模型和工具接口。Codex TUI 的完整斜杠命令列表还包含大量 TUI 自身行为，例如键位、主题、复制、清屏和退出；这些命令不能直接照搬到第三方客户端。[官方命令表](https://learn.chatgpt.com/docs/developer-commands#built-in-slash-commands)

当前 `codex-acp` 适配器在 App Server 上增加了一层命令翻译。它发布 `plan`、`mcp`、`skills`、`status`、`review`、`review-branch`、`review-commit`、`compact`、`goal`、`rename` 和 `logout`，并把 Codex 技能发布为 `$技能名`。[CodexCommands](https://github.com/agentclientprotocol/codex-acp/blob/472e60e4e99234c47c6de67a6d9cc8a71ebda47e/src/CodexCommands.ts#L57)

其中 `/compact`、`/review`、`/status` 等会被适配器转换成 App Server 的真实调用或结构化回复；未识别命令才继续进入普通提示词路径。[命令处理](https://github.com/agentclientprotocol/codex-acp/blob/472e60e4e99234c47c6de67a6d9cc8a71ebda47e/src/CodexCommands.ts#L205)

这条路径符合 VibeStudio 已确认的 ACP 优先架构：VibeStudio 连接 `codex-acp`，由适配器使用 App Server。M0 不增加一条 VibeStudio 直连 Codex App Server 的并行实现。

## 6. CodeG 与其他客户端的取舍

CodeG 同样以 `available_commands_update` 为事实来源，后端过滤已经由模型、模式和权限控件承担的命令，再按名称去重；输入框只搜索当前会话提供的命令。[CodeG 命令更新](https://github.com/xintaofei/codeg/blob/18046ea4ecc11a68f8f7bac9e54c99b28658dfe6/src-tauri/src/acp/connection.rs#L13666) [输入候选](https://github.com/xintaofei/codeg/blob/18046ea4ecc11a68f8f7bac9e54c99b28658dfe6/src/components/chat/message-input.tsx#L731)

Nimbalyst 的另一种接法也验证了同一边界：直接连接 Codex App Server 时，它不提供 TUI 的命令列表，因此上下文压缩等操作改用真实接口按钮；Claude SDK 则从初始化消息获取命令。[Codex 能力](https://github.com/nimbalyst/nimbalyst/blob/d6e1d008d9ee264a7447f3533fa9f48f158a70b0/packages/runtime/src/ai/server/providers/OpenAICodexProvider.ts#L151) [Claude 命令](https://github.com/nimbalyst/nimbalyst/blob/d6e1d008d9ee264a7447f3533fa9f48f158a70b0/packages/runtime/src/ai/server/providers/ClaudeCodeProvider.ts#L2224)

两种实现都说明：客户端不能把某个 Agent 原生 TUI 的命令文档直接变成自己的命令菜单。

## 7. VibeStudio 接入契约

### 7.1 会话能力快照

每个会话独立保存以下事实：

- Agent、适配器及适配器实际携带或调用的运行版本。
- ACP 协议版本、初始化能力和扩展能力。
- 当前可用命令的完整列表，包含名称、说明和参数提示。
- 当前配置项，包含模式、模型、思考等级及其他 Agent 自定义项。
- 提示词支持的文字、图片、音频、资源等类型。
- 终端、文件回调、表单提问、权限和用量能力。

收到完整更新时替换该会话的对应快照。切换项目、会话或网格不能把 A 会话的命令和配置带到 B 会话。

### 7.2 命令输入

- 用户输入 `/` 后，展示当前会话上报的命令；名称、说明和参数提示直接采用 Agent 数据。
- 可为已知命令补充图标和中文展示名，但本地元数据不能创造新命令，也不能改变发送值。
- 选中后在输入框形成可识别的命令片段，发送时还原为 Agent 期望的 `/命令 参数`。
- `$技能` 与 `/命令` 分开标识来源。Agent 没有上报技能时不猜测磁盘目录。
- 命令在运行中更新或消失时，未发送候选立即更新；已经提交的回合按提交时的原始文本保留。

### 7.3 工具与宿主能力

- 工具调用按调用标识合并增量更新，状态只允许向协议给出的后续状态推进；迟到事件不能覆盖已完成结果。
- 权限和提问保存原请求标识、目标会话和有效期。按钮使用请求提供的选项，提交一次后立即禁用。
- M0 只声明已经实现的客户端能力。终端、文件读写和表单提问分别验证，不能打包成一个“支持工具”结论。
- 停止使用 `session/cancel` 或适配器明确提供的取消能力；模型、模式和思考等级使用配置接口。
- 适配器扩展只有经过能力协商和实测后才能进入业务路径，未知扩展保留诊断并做通用展示。

## 8. M0 新增验证矩阵

| 验证项 | 通过条件 | 证据 |
| --- | --- | --- |
| 版本成对记录 | 记录适配器及其实际运行的 Agent/SDK 版本 | 版本输出、包清单、启动记录 |
| 新会话命令发现 | 会话建立后收到真实命令列表或明确无此能力 | 原始更新摘要、规范化快照 |
| 恢复会话命令发现 | 恢复历史后命令列表仍属于原会话，且不会与历史事件串线 | 会话标识、事件顺序 |
| 命令参数 | 名称、说明和参数提示能进入当前会话输入候选 | 命令快照、界面无关的结构化结果 |
| 只读命令 | 至少执行一个不会改写项目的命令并得到可辨识结果 | 请求文字、返回事件 |
| 状态命令 | 至少验证一个会改变会话状态的命令或专门配置接口 | 调用前后状态 |
| TUI 专属命令 | 原生终端专属命令不进入 ACP 候选 | 命令列表对照 |
| 工具生命周期 | 至少观察一次工具等待、运行和完成或失败 | 调用标识、状态序列 |
| 权限往返 | 决策选项来自 Agent，答复回到原请求，失效后不能再次提交 | 请求与答复标识 |
| 客户端能力 | 每项声明的终端、文件、表单能力都有真实处理结果 | 初始化能力与反向请求记录 |
| 配置项 | 模式、模型、思考等级按 Agent 实际数据读取和修改 | 配置前后完整快照 |
| 取消 | 取消结束目标回合，并清理仍在等待的审批和工具状态 | 停止原因、清理记录 |

## 9. 已确定与后续确认

已确定：

- MVP 继续使用 ACP 统一接入，Claude/Codex 的内部 SDK 或 App Server 由适配器负责。
- 原生命令、配置项和工具能力按会话动态获取，缺失时不显示。
- VibeStudio 的通用按钮调用真实协议能力，不依赖未上报的斜杠命令。
- 版本记录必须覆盖适配器实际使用的 Agent 运行依赖。
- M0 已锁定两个适配器的 Windows 启动入口，并验证新建、加载、恢复、动态命令、配置、工具、审批、用量和取消；详情见 [M0 双 Agent 接入验证](../验证记录/M0/2026-09-14-双Agent接入验证.md)。

后续实现仍需确认：

- 配置变化、上下文压缩和本地命令是否会产生完整、可归属的状态事件。
- 终端、文件和表单提问等客户端反向能力完成实现后的真实往返。
- 适配器升级后兼容性检查和回退规则。
