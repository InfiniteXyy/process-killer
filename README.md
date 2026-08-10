# Process Killer

一个使用 [GPUI](https://www.gpui.rs/) 和
[gpui-component](https://github.com/longbridge/gpui-component) 编写的纯 Rust 进程管理器。

## 功能

- 按进程名或 PID 实时筛选
- 使用 `:端口号` 或 `：端口号` 筛选监听该端口的进程
- 使用虚拟列表显示进程图标、CPU/内存占用率和本地网络端口
- 鼠标点击或键盘上下键选择，回车确认结束进程
- 点击列表表头按进程、端口、CPU 或内存升降序排列
- 在主页按 Esc 直接退出，在设置页按 Esc 返回主页
- 中文/English、浅色/深色/跟随系统主题
- 1、5、10、20 秒自动刷新间隔
- 设置持久化，以及独立的设置页面

## 开发

```powershell
cargo run
```

```powershell
cargo test
cargo build --release
```

## 落地页

直接打开 `public/index.html` 即可预览。推送到 `main` 后由 GitHub Actions 自动发布到 GitHub Pages。

