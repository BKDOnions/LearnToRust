# Rust学习Repo

[感谢大佬的翻译](https://kaisery.github.io/trpl-zh-cn/title-page.html)

Beware: This repo only has a chinese version due to my lazy

## 关于安装Rust

此处遇到的问题：

+ 默认的rustup.init的安装通道是针对msvc编译环境的，出于对存储的节省和环境的依赖，建议选择gnu通道
+ 默认的安装位置：
  + Windows: %USERPROFILE%/.cargo， 通俗点就是C:/Users/{username}/.cargo
  + Linux: ~/.cargo
  + Mac OS: 没钱所以没了解
+ 如何修改默认安装路径
  + Windows: 在环境变量中添加两个变量`RUSTUP_HOME`和`CARGO_HOME`，分别指向rustup和cargo的根目录，理论上修改上述两个变量已经能解决基本的存储位置问题，具体细节的环境变量请参考官方文档
  + Linux: 同样是修改系统环境变量，推荐修改/etc/environment进行持久化设置，再通过`source /etc/environments`命令去立即生效设置。
  + Intellij idea未找到在Linux环境下除默认安装方式外的安装方案，正常设置下版本读取不出来，但是可以通过其他方法暂时修改(依然无法自动下载standard library)。



