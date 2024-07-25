# 粘贴器

一个小工具，通过模拟键盘输入的方式在任何禁止粘贴（例如批改网网页端）或没有粘贴功能（例如某些虚拟机）的地方输入想粘贴的内容。

## 使用

首先将需要输入的内容粘贴到上方大输入框里。随后点击“开始”按钮，并在倒计时结束前点击一下需要粘贴的位置，将输入光标放置在需要粘贴处。倒计时结束后，程序自动输入。也可以点击“取消”按钮提前取消输入操作。

使用熟练后，可以将倒计时改短以提升输入效率。

## 系统支持

支持 Windows，macOS 以及 Linux。

### Windows

一般而言，Windows 平台不需要安装额外依赖或进行额外的操作。然而，出于安全考虑，系统会限制普通进程操作“完整性等级”更高的进程。因此，如果输入没有生效，请尝试使用管理员身份运行本程序。

### macOS

需要授予程序相应权限。参考[这篇文章](https://web.archive.org/web/20231005204542/https://support.apple.com/guide/mac-help/allow-accessibility-apps-to-access-your-mac-mh43185/mac)。

### Linux

Linux 情况比较复杂，请参阅 [`enigo` 文档](https://github.com/enigo-rs/enigo?tab=readme-ov-file#features)。视桌面环境而定，你可能需要安装依赖和/或启用额外的 feature 自行编译。
