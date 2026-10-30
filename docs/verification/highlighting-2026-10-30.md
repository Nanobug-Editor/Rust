# Rust 0.6.3 高亮验证

状态：插件单测、打包、静态准入、深浅主题颜色回放与只读高亮回放通过；原生浅色窗口已核对。用户按 Esc 停止后续桌面操作，未继续原生深色交互。未发布 Release 或市场更新。

## 原因与范围

0.6.2 的真实 ZIP 经宿主 `Package` 校验后，`///` 第三个 `/`、`//!` 的 `!` 被全局运算符查询捕获。文档标记是独立节点，与运算符范围相同；宿主采用较短范围优先、相同范围后声明优先，因此整条注释捕获不能覆盖它们。0.6.3 在通用运算符后显式捕获文档标记，保留 grammar 与宿主通用范围优先级。

同时补齐实际回放确认的带路径／泛型调用、宏定义及带路径宏调用、生命周期与标签、常量声明、`crate`／`super`、复合运算符。现有字符串、字节／C 字符串、原始字符串、字符、浮点数及中文 CRLF 作为对照。常量使用声明结构识别，不根据名字大小写猜测。

grammar 仍为原有 ABI 15 WASM；节点和符号依据 [tree-sitter-rust v0.24.0 grammar](https://github.com/tree-sitter/tree-sitter-rust/blob/v0.24.0/grammar.js) 核对。没有修改语言服务启动、权限或 API 能力范围。

## 可重复入口

宿主验证用 `NANOBUG_TEST_PACKAGE_INPUTS` 指向包含 `id`、`version`、实际 ZIP 绝对路径 `archive` 和 `sha256` 的 `packages` 数组账本。它走既有 hash 校验和 `Package::read`，不改固定历史夹具。

```powershell
# Rust 独立 worktree；Cargo 默认使用该 worktree 自身 target。
cargo fmt --check
& $pluginHost --plugin-cargo Cargo.toml test --lib --locked --offline -j 2
& $pluginHost --plugin-package . --output C:/Projects/RustProjects/Editor/dist/rust
& $pluginHost --plugin-check C:/Projects/RustProjects/Editor/dist/rust/rust-0.6.3.zip --format json

# Editor 任务 worktree；显式运行真实包高亮验收。
$env:NANOBUG_TEST_PACKAGE_INPUTS = '<外部候选账本的绝对路径>'
cargo test -p editor-app --bin editor-app rust_package_ -j 2 -- --ignored --nocapture --test-threads=1
```

宿主固定源码为 `409397184e5b26c19b899e1d4d919786f27d0ea5`。打包可执行文件 `C:/Projects/RustProjects/Editor/target/release/editor-app.exe` 的 SHA-256 为 `596de23a308d7df693a64c402941604f5e8ce0f81bb19f5458d29273028160b2`；其公开静态检查报告的 SDK 摘要 `4fcb7f89448e48854fd1281808277c3859aea83343322c9de46e088adc0e1c20` 与主 checkout 的 94 项 SDK 资源按相同顺序及长度前缀计算一致。`host-sdk.json` 同步记录该基线。不同 checkout 换行会影响字节摘要，应以实际打包宿主的内嵌资源为准。

## 验证结果

0.6.2 原包回放：两项高亮测试失败，分别观察到文档标记颜色错误，以及两种主题中的 21 项命名／运算符遗漏。块注释、普通注释、字面量对照正常。

- 0.6.3 插件单测：10 passed / 0 failed / 0 ignored；格式与差异检查通过。
- 实际 ZIP：`C:/Projects/RustProjects/Editor/dist/rust/rust-0.6.3.zip`，SHA-256 `a872d5cce2c609e2d873eccbb3baf5c52138a7e4c0592908811da3d939934327`；公开静态准入 `passed=true`、`executed=false`、无诊断。静态检查不替代运行验收。
- Editor 的三个 `rust_package_` ignored 用例显式运行后：3 passed / 0 failed / 0 ignored。覆盖每个注释字符、两种主题中的代表性语法、所有声明 capture 的可见样式，以及只读代码区的 UTF-8 原文范围。
- 全 capture 检查额外发现宿主未映射 `namespace`。宿主通用主题已补该类别，单测先失败后通过；未添加 Rust 专属宿主分支。旧宿主仍可使用 0.6.3 的注释与其他查询修复，模块名颜色需要新的宿主主题映射。
- 通过公开 `--plugin-dev` 在独立 profile 安装并激活实际包，候选摘要与上面 ZIP 一致。原生浅色窗口中 `///`、`//!`、文档块、嵌套注释完整保持绿色，调用、生命周期及复合运算符可见。初始只含源码的验收目录未含 Cargo 清单，因此出现找不到项目的语言服务消息；这些消息不作为语法高亮失败或原文件监听警告的复现。
- 原生深色交互及新宿主模块名颜色的实际截图未完成：用户物理 Esc 停止桌面控制后没有继续。深浅主题的最终样式解析已由上述真实 WASM/GPUI 颜色测试覆盖。未替换用户正在运行的宿主或已安装插件。

普通插件测试使用独立 worktree 自身 `target/`；公开打包器按项目根身份使用宿主管理的私有 `MeEditor/plugin-build/1a9615c56001d337/`，没有共用 Editor 主项目编译输出。grammar、原生桥和权限未改变。

日志、截图及候选账本保存在项目外 `C:/Users/Tmiracle/.codex/task-artifacts/rust-warning-20261030/`。
