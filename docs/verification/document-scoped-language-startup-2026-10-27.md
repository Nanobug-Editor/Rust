# Rust 0.6.2 文档范围语言启动验证

日期：2026-10-27，Windows x86_64。

状态：独立插件单测、公开入口打包、静态包准入和宿主本机原生交互已通过。实际安装与根项目／独立插件同时分析的补充验收见下文。未提交、推送、发布 Release 或修改市场记录。

## 问题与修复

`rust` 0.6.1 的语言钩子会把根 `Cargo.toml` 和工作区下所有配对 `Cargo.toml`、`manifest.json` 的独立插件加入 `linkedProjects`。当前 Nanobug 工作区有 35 个这样的项目，服务器在逐个准备固定测试夹具时超过清单的 120 秒就绪期限。

本地独立 stdio 探针保留同一 Rust Analyzer、客户端实验能力、SDK Cargo 配置和诊断配置：35 个项目时，初始化在 0.281 秒完成，135.062 秒仍未收到 `quiescent=true`；仅选择根项目时，14.219 秒收到该通知。探针只用于隔离配置原因，不替代新包的原生宿主验收。

0.6.2 根据公开 `language::Context.document.path` 选择包含请求文档的最近独立插件项目，否则使用工作区根 Cargo 项目。未指定文档时只选择根项目。每个服务只链接选中项目，继续使用宿主公开 SDK；嵌套独立插件通过 `project_root` 获得自己的上下文。120 秒期限、插件 ID、设置及权限保持原值。清单最低要求提升为 `language.lsp >=1.5,<2`，以明确启用宿主的文档范围发现路径。

## 生产数据流核对

在宿主源码 `2b043e2d74c91cbb10f44ff8e247cf677425fd48` 核对：

- `crates/editor-app/src/app/language_servers/contexts.rs` 从 `tab.path().strip_prefix(self.workspace.root())` 构造 `DocumentContext.path`，并把反斜杠替换为 `/`。这是实际启动使用的工作区相对路径。
- `crates/editor-app/src/extensions.rs::demand_language_document` 将原文档上下文交给 `DemandReason::LanguageDocument`；运行时 `manager/activation/roles.rs` 和 `language_preparation.rs` 将其传入语言钩子，没有改成绝对路径。
- `extensions.rs::language_provider_has_contexts` 仅对最低要求排除 1.5 之前版本的插件启用此路径。新清单满足此条件。
- `manager/language/contexts.rs` 的共享键包含提供者、`ProjectContext` 和规范化的服务根目录；同为 `cargo` 的上下文键不会使根工作区和独立插件根误用同一服务。复用还检查完整 `same_plan`。
- 原生打开已有文件先规范化路径；`FindFiles` 返回规范化目录下的相对路径。Windows 盘符不进入项目相对路径；保留实际目录大小写，也保留大小写敏感文件系统的语义。

## 可重复检查

工作目录：`C:/Projects/RustProjects/Rust-for-Nanobug`，`main`，初始提交 `2a8059c02011238ceac56f7d3669c2decd326c9d`，初始工作区干净。

使用的生产宿主：`C:/Projects/RustProjects/Editor/target/release/editor-app.exe`。

- 可执行文件 SHA-256：`55b2f9c818f4e29516dfd22c5d16c3051d92c37e07b99812a9e555253b40dd39`。
- SDK SHA-256：`8d488db82a03cd433952171b085695ae015885a26c32f6d57f1ac4691d2bdf27`。按宿主 `sdk_export.rs` 的相同顺序、长度前缀和字节算法核对 94 个源码资源，当前固定源码的摘要与可执行文件静态报告一致。
- `cargo metadata --no-deps --format-version 1` 确认独立仓库根与默认输出目录分别为本仓库和其 `target`；没有借用宿主 Cargo 测试输出。

```powershell
$pluginHost = 'C:/Projects/RustProjects/Editor/target/release/editor-app.exe'
cargo fmt --check
& $pluginHost --plugin-cargo Cargo.toml test --lib --locked --offline
& $pluginHost --plugin-package . --output dist
& $pluginHost --plugin-check dist/rust-0.6.2.zip --format json
git diff --check
```

结果：10 项插件单测通过，0 失败、0 忽略；格式和差异检查通过。新增 `workspace_startup_does_not_link_unopened_plugin_projects` 曾在原实现观察到失败：实际包含 `plugins/unused/Cargo.toml`，期望仅根项目。其他回归覆盖最近嵌套项目、目录前缀边界、未保存文件名、无根 Cargo 工作区、SDK 配置和同项目跨文档代际复用。

公开打包入口成功生成 `dist/rust-0.6.2.zip`；静态准入 `passed=true`、`executed=false`、`diagnostics=[]`。包 SHA-256：`89d4a20475bae56bb59084b7b90fb378069800b1408607552173eba605b98d92`。

`Cargo.lock` 除本包版本外，增加当前固定 SDK 所需的 `unicode-segmentation` 1.13.3；没有更新其他现有依赖版本。清单、语言资源声明、Cargo 版本及双语说明同步为 0.6.2。没有自动安装工具或依赖 SDK 源码副本。

## 宿主本机原生补验

本轮宿主修复任务用重新构建的 Release（SHA-256 `8edb734d9f4ebb5dbc885c3ba985717de50fb769377ee84c4fa299a65a694b4b`，SDK 摘要保持上值）在原用户 profile 中通过插件管理器安装本候选，实际注册版本为 0.6.2，权限不变。

- Editor 根工作区语言服务器正常就绪，启动提示消失；对 `editor_ui::aligned_pointer::for_window` 执行 `Ctrl+I`，返回实际签名、源码注释和定义位置。
- 保持根项目文档打开，再打开 `crates/plugin-runtime/tests/fixtures/sdk-completion/src/lib.rs`。独立语言服务器单独启动并就绪，根服务保持运行；对 `plugin_protocol::ui::Action` 执行 `Ctrl+I`，返回实际枚举及宿主 `MeEditor/plugin-sdk/<摘要>/src/ui.rs` 定义路径，未在访客目录加入 SDK 源码。
- 关闭访客文档后，其独立服务退出，根服务继续存在。测试期间 Cargo 对固定访客锁文件的自动刷新已恢复，不属于交付改动。

本次窗口采用浅色中文。完整宿主验收见相邻 Editor 仓库的 `docs/plugins/verification/plugin-language-repair-2026-10-27.md`；没有将未执行的其他语言生态或操作系统场景描述为通过。
