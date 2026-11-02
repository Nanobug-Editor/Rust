# Rust

版本 **0.6.4** 面向 2026-11-02 的 Nanobug 内部测试版，使用公开宿主 SDK 构建。保留安装 ID 和权限；包内附带的 Windows 原生辅助程序使用静态运行库。

[English](https://github.com/Nanobug-Editor/Rust/blob/main/README.md) · **简体中文**

Rust 识别与语言服务、Cargo 配置和合并调试器。 插件 ID 为 `rust`，首次独立发布版本为 `0.6.1`；原有安装身份、设置和私有数据保持连续。

当前源码版本为 **0.6.4**。文档注释标记保持注释颜色，覆盖 `///` 的第三个斜杠和 `//!` 的感叹号；补齐带路径和泛型的调用、宏定义、生命周期、常量声明、路径关键字与复合运算符。检查记录见[高亮验证](docs/verification/highlighting-2026-10-30.md)。

语言服务启动时选择工作区 Cargo 根项目，或请求文档所属的最近独立插件项目。尚未打开的独立项目不进入该服务的分析；打开独立插件文档时使用其项目上下文，保留宿主 SDK 补全。就绪期限保持 120 秒。

本仓库是业务维护来源。[市场注册仓库](https://github.com/Nanobug-Editor/Plugin-Marketplace)负责审核，安装包保存在本仓库 Releases。每版都通过市场 PR 和人工审核，不以 Release 发布代替上架。

## 独立构建

使用 host-sdk.json 固定版本构建的 Nanobug 程序，并准备原生编译环境及清单所需目标。将 `$pluginHost` 设为该程序的绝对路径：

```powershell
# 通过宿主公开入口生成固定版本 ZIP，不复制 SDK 源码。
& $pluginHost --plugin-package . --output dist
```

WASM 项目另使用 `& $pluginHost --plugin-cargo Cargo.toml test --lib` 验证。声明式项目无需空 WASM。缺失工具会报错，不自动安装。发布前确认清单权限、目标平台、能力要求和 ZIP 的 SHA-256；不同字节必须提升版本重新审核。

## 维护与许可

迁移前的功能说明及历史验证见 [原始说明](https://github.com/Nanobug-Editor/Rust/blob/v0.6.1/docs/source-readme.md)，新的构建和发布依据以本页及当前源文件为准。迁移保持既有业务行为。仅发布合并的 Rust 插件。

Nanobug workspace 的 Cargo MIT 许可声明在 LICENSE 中明确保存；grammar、图标及原生依赖保留自身许可与随包 notice，不由根许可覆盖。将生成 ZIP 从原生插件管理器本地导入或待市场审核后安装，并按实际权限确认。
