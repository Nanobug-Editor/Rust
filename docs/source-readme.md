> 迁移前说明：下列版本和宿主内路径是历史记录；独立构建与本版发布以根 README 和 host-sdk.json 为准。

# Rust 插件

版本 0.6.0 使用协议 7。语言识别、Tree-sitter 高亮和图标由资源声明提供；一个 `rust.wasm` 同时提供 Rust Analyzer 配置、Cargo 运行配置/目标和调试会话。内部按 `language`、`cargo`、`debugger` 模块分工，宿主通过公开契约选择提供者。

运行配置使用公开 `run.configurations 1.0`，提供 Cargo 分组的 run、build、debug 模板，默认参数为 `run --release`、`build --release` 和 `run`。程序只读，子命令与选项都是可编辑的独立参数，空格与引号不会被重新拆分。添加不运行命令；保存和每个执行入口重新进行插件校验。缺少 Cargo 或根目录 Cargo.toml 时模板置灰；工具查找使用公开 `process 1.7`，不运行版本探测。

更多设置包含工作目录和环境变量。Run 由 Cargo 自行构建并运行一次，不重复添加宿主构建；Build 仅编译。Debug 查询真实 metadata，明确选中包和 bin 后准备真实产物，支持 dev/debug、release 配置；多目标时请提供 `--package`／`--bin`，其他 profile 明确拒绝。`--` 后的参数只传给目标程序。表单使用原生滚动控件，保留输入、焦点与中文 IME。

Rust Analyzer 服务使用包内固定声明：优先检查 HOME 下 Scoop、VS Code 和 VS Code Insiders 的扩展目录，再检查系统 PATH。候选程序必须通过 `--version` 探测；缺少可用程序时报告服务准备失败。仅按声明路径查找工具，不自动安装工具，语言服务仍使用独立的声明式权限。

插件设置中的“Rust Analyzer 可执行文件”支持用户设置与明确保存的项目覆盖。默认自动查找；保存绝对路径后只使用该文件，错误不会被自动回退掩盖。使用重置恢复自动查找。

语言钩子通过 `workspace.files` 1.1 查找当前工作区内的 `Cargo.toml` 与同目录 `manifest.json`，保留根 Cargo 项目，排除 `target`、`vendor`、`node_modules` 和 `.git` 并遵守忽略规则。不可读的无关目录不会停用语言分析。Windows 路径按 Rust Analyzer 的文档 URI 规则规范化，避免未保存内容被误判为外部库。

通过 `host.sdk` 获取宿主发布的 SDK 与 Cargo 配置路径；`cargo.configPath`、实验性原生诊断及完整和分节 `rust-analyzer` 配置由本插件提供。无需在独立插件项目写入 `.cargo`、复制 SDK 或引用宿主业务源码。服务声明请求 `serverStatusNotification`，等待 `experimental/serverStatus` 的 `/quiescent` 为 `true`，最多 120 秒。

使用已构建宿主按 `nanobug-plugin.json` 编译组件并自动归档清单与运行资源，默认 ZIP 位于插件项目根部：

```powershell
./target/debug/editor-app.exe --plugin-package plugins/rust
```

单独编译时使用公开 SDK 入口：

```powershell
editor-app.exe --plugin-cargo plugins/rust/Cargo.toml build --target wasm32-wasip2 --release
```

打包包含 `rust.wasm`、清单、README、grammar、查询、图标和许可证，Windows x86_64 另外包含 `native/me-debug-bridge.exe`；不包含 Rust 源码、Cargo 缓存或 SDK 副本。安装或更新后已打开文件重新选择 provider；停用、切换 provider 或卸载会释放本插件的服务。
运行目标发现与构建由本包自己的 WASM 组件实现 run.targets 1.0。发现只读 Cargo metadata，返回每个实际包／bin 的 debug 与 release 可移植绑定；确认后有序准备，按 Cargo 编译产物消息中的 manifest_path、bin 和 profile 匹配确切可执行文件。虚拟工作区按 default-members 发现，包名相似或同名 bin 不互相替换；缺少 Cargo、项目更名或绑定失效会明确报告，不启动另一程序。输出和停止由宿主通用准备界面管理，构建不会自行运行产物。

## 调试与旧包迁移

调试使用公开 `debug.session 1.1`、`plugin.services 1.1`、`process 1.7` 和 `dependencies 1.2`。Windows x86_64 支持启动 Rust/MSVC 或 GNU 产物、源码断点、暂停、继续、步入/步过/步出、调用栈和局部变量；暂不提供附加。macOS/Linux 包只提供语言和 Cargo，当前不提供 CodeLLDB 适配器。

CodeLLDB 1.12.3 固定下载和 SHA-256 校验。安装/普通启动不下载或启动调试器；第一次调试才在后台准备私有依赖，已验证缓存会复用，不改全局环境或安装 Rust 编译器。准备中 Cargo 可继续运行。下载失败可以再次调试重试；取消准备不会启动目标。可在用户或项目设置中关闭“启用 Rust 调试”，继续使用语言与 Cargo。

目标经 DAP `runInTerminal` 启动一次。插件创建并通过公开 `PresentTerminal` 展示受管 PTY，目标 stdin/stdout 与适配器 DAP stdout、诊断 stderr 分离。调试控制、断点、变量和调用栈在对应运行 Tab 中显示。停止、禁用或插件替换回收适配器、PTY 和目标树，已结束 Tab 保留输出。继续、步进及新停止事件撤销旧暂停引用。

安装确认会列出将替代的旧 `rust-debugger`（0.1.x–0.2.x）。批准后保留旧用户/项目启用偏好为 `debugger.enabled`，仅把明确选中旧包的 `debug.session` 选择改为 `rust`，其他提供者选择保持有效。旧安装记录标记已替代，原私有数据、逻辑快照和依赖收据保留；不会把旧数据暴露给新插件。失败恢复旧记录和设置，旧实例继续可用。已有 Rust Analyzer 路径及 Cargo 配置不变。

适配器来源及许可：[CodeLLDB 1.12.3](https://github.com/vadimcn/codelldb/releases/tag/v1.12.3)、[MIT license](https://github.com/vadimcn/codelldb/blob/v1.12.3/LICENSE)。
