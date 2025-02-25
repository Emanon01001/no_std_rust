# no_std_rust

このプロジェクトは、`no_std` および `no_main` を利用して Windows 用のシンプルなコンソールアプリケーションを実装しています。  
[Cargo.toml](Cargo.toml) でパニック時に `abort` を設定しており、標準ライブラリへの依存を排除しています。

## 概要

- **目的**: Windows 環境で標準出力に "Hello, World!" を表示するシンプルなアプリケーション。
- **特徴**:  
  - `no_std` により標準ライブラリを使用せず、組み込みの機能で動作。
  - エントリーポイントは [`mainCRTStartup`](src/main.rs) 関数で、Windows 用のリンカオプション `/ENTRY:mainCRTStartup` を使用しています。
  - Windows API [`GetStdHandle`](src/main.rs), [`WriteConsoleA`](src/main.rs), [`ExitProcess`](src/main.rs) を直接呼び出しています。

## ビルド方法

1. **Rust のインストール**  
   Rust のツールチェーンがインストールされていることを確認してください。

2. **ビルド**  
   コマンドラインで以下のコマンドを実行してビルドします:
   ```sh
   cargo build --release
