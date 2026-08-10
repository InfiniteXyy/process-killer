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

## macOS 安装说明

下载 `ProcessKiller.dmg` 后，如果遇到 **"已损坏，无法打开"** 的提示，这是因为应用使用了临时签名（ad-hoc signing），macOS Gatekeeper 会拦截从互联网下载的应用。

### 解决方法（任选其一）

**方法 1：右键打开（推荐）**
在 Finder 中右键点击 `ProcessKiller.app` → 选择「打开」→ 点击「打开」确认。

**方法 2：移除隔离属性**
```bash
xattr -cr /Applications/ProcessKiller.app
```
> 只需要执行一次。

**方法 3：全局允许**
```bash
sudo spctl --master-disable
```
> ⚠️ 这会关闭 Gatekeeper，降低系统安全性，不推荐。

### 开发者签名说明

当前 Release 使用临时签名（ad-hoc signing）。如果你有 Apple Developer 账户（$99/年），可以在 GitHub Actions 中配置以下 Secrets 来启用正式签名和公证：

| Secret | 说明 |
|--------|------|
| `APPLE_SIGNING_IDENTITY` | 证书名称，如 `"Developer ID Application: Your Name (TEAMID)"` |
| `APPLE_CERTIFICATE_BASE64` | `.p12` 证书文件的 Base64 编码 |
| `APPLE_CERTIFICATE_PASSWORD` | `.p12` 证书的密码 |
| `APPLE_KEYCHAIN_PASSWORD` | 临时 Keychain 密码（任意字符串） |
| `APPLE_NOTARY_KEY_ID` | App Store Connect API Key ID |
| `APPLE_NOTARY_ISSUER` | App Store Connect API Issuer ID |
| `APPLE_NOTARY_KEY` | App Store Connect API Key 内容 |

配置后，下一次 Release 将自动签名并公证，彻底解决 Gatekeeper 提示问题。

## 落地页

直接打开 `public/index.html` 即可预览。推送到 `main` 后由 GitHub Actions 自动发布到 GitHub Pages。

