# VibeStudio 静态界面原型

- single.html：单会话。
- grid.html：2×2 网格。
- grid-3x3.html：3×3 网格。

直接在浏览器打开 HTML 即可。页面没有应用 JavaScript、网络请求或业务交互；输入、任务、模型选择和运行状态都是视觉模拟内容。浏览器本身的滚动、焦点和提示可正常使用。

样式按主题、基础控件、工作台布局拆分。图标来自本地 Lucide 和 Simple Icons 资源，许可见 assets/LICENSES.txt。

输入区左侧为添加上下文、固定 Agent 图标、权限选择；右侧依次为模型、思考等级、上下文用量饼图、发送或停止按钮。小面板自动换行并保留右对齐；权限、思考等级和上下文用量均为静态演示。

参考：

- https://github.com/xintaofei/codeg
- https://raw.githubusercontent.com/xintaofei/codeg/main/docs/images/workspace-light.png
- https://learn.chatgpt.com/docs/projects?surface=app

此目录只用于原型，不替代已确认的需求、技术设计和架构设计。
