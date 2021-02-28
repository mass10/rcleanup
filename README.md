# About

お掃除アプリケーションです。いくつかの条件が成り立つとき、ディレクトリを削除します。
* Cargo.toml が存在しているときの target ディレクトリ
* package.json が存在しているときの node_modules ディレクトリ

# Getting Started

次のコマンドを実行すると、rleanup がコンピューターにインストールされます。既にインストールされている場合はアップデートされます。

```bash
cargo install --git https://github.com/mass10/rcleanup --branch main
```

# How to Run

```bash
rcleanup {PATH_TO_DIRECTORY}
```

# Uninstall

次のコマンドを実行すると、rcleanup がコンピューターからアンインストールされます。

```bash
cargo uninstall rcleanup
```
