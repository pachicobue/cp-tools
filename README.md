# cp-tools(cpt)

競プロ用コマンドラインツール

## 目標

### コア機能

- サンプルチェック
    - [x] 通常ジャッジ
    - [x] スペシャルジャッジ
    - [x] リアクティブ
    - [ ] Run twice
- Hackケース生成(WA/RE/TLE)
    - [x] 通常ジャッジ
    - [x] スペシャルジャッジ
    - [x] リアクティブ
    - [ ] Run twice

### その他ツール

- ライブラリ展開
    - [x] C++
        - `clang++ -E` コマンドベース

## 出来たらいいな

- ネットワークコマンド
    - サンプルダウンロード
    - 提出

## インストール

- `cargo install --path cpt-core`
- `cargo install --path cpt-extra`

## 使い方

Optional なパラメータがありがち。
（例）`--tl` によるTimeLimit指定（ちなみにMemoryLimit指定はできない）

詳細は `cpt --help` をチェック。

### 自動テスト機能

#### 通常テスト

**（注意）誤差を許容するテストは非サポート。スペシャルジャッジとしてツールを使用すること**

```sh
cpt test batch -c "./main.exe" -d test

(short version)
cpt t b -c "./main.exe" -d test
```

- パラメータ
    - `-c`: プログラム実行コマンド
        - プログラム
    - `-d`: テストケースのディレクトリパス
        - 中間ファイル（標準エラー出力など）もここに格納される

#### スペシャルジャッジ

```sh
cpt test batch -c "./main.exe" -j "./judge.exe" -d test

(short version)
cpt t s -c "./main.exe" -j "./judge.exe" -d test
```

- パラメータ
    - `-c`: プログラム実行コマンド
    - `-j`: ジャッジコマンド
        - ジャッジは２つの引数を受け取る
            - `<judge_command> <input_path> <output_path>`
               - `input_path`: テスト入力パス
               - `output_path`: プログラムによる出力パス
    - `-d`: テストケースのディレクトリパス

#### リアクティブ


```sh
cpt test reactive -c "./main.exe" -j "./judge.exe" -d test

(short version)
cpt t r -c "./main.exe" -j "./judge.exe" -d test
```

- パラメータ
    - `-c`: プログラム実行コマンド
    - `-j`: ジャッジコマンド
        - ジャッジはプログラムの標準出力を標準入力からインタラクティブに受け取る
        - ジャッジは１つの引数を受け取る
            - `<judge_command> <input_path>`
               - `input_path`: テスト入力パス
    - `-d`: テストケースのディレクトリパス

### Hackケース生成

#### 通常テスト

```sh
cpt hack batch -c "./main.exe" -i "./gen_input.exe" -d test

(short version)
cpt t b -c "./main.exe" -i "./gen_input.exe" -d test
```

- パラメータ
    - `-c`: プログラム実行コマンド
    - `-i`: 入力生成コマンド
    - `-o`: *(Optional)* 出力生成コマンド
        - 指定しない場合は `WA` の確認はできない
        - 出力生成は引数を受け取らず、入力データを標準入力から受け取る（プログラム本体と同じ）
    - `-d`: テストケース生成先ディレクトリパス

#### スペシャルジャッジ

```sh
cpt hack special -c "./main.exe" -i "./gen_input.exe" -j "./judge.exe" -d test

(short version)
cpt h s -c "./main.exe" -i "./gen_input.exe" -j "./judge.exe" -d test
```

- パラメータ
    - `-c`: プログラム実行コマンド
    - `-i`: 入力生成コマンド
    - `-j`: *(Optional)* ジャッジコマンド
        - 指定しない場合は `WA` の確認はできない
        - ジャッジは２つの引数を受け取る
            - `<judge_command> <input_path> <output_path>`
               - `input_path`: テスト入力パス
               - `output_path`: プログラムによる出力パス
        - 出力生成は引数を受け取らず、入力データを標準入力から受け取る（プログラム本体と同じ）
    - `-d`: テストケース生成先ディレクトリパス

#### リアクティブ

```sh
cpt hack reactive -c "./main.exe" -i "./gen_input.exe" -j "./judge.exe" -d test

(short version)
cpt t r -c "./main.exe" -j "./judge.exe" -d test
```

- パラメータ
    - `-c`: プログラム実行コマンド
    - `-j`: ジャッジコマンド
        - ジャッジはプログラムの標準出力を標準入力からインタラクティブに受け取る
        - ジャッジは１つの引数を受け取る
            - `<judge_command> <input_path>`
               - `input_path`: テスト入力パス
    - `-d`: テストケースのディレクトリパス

## Credits

[CREDITS.toml](CREDITS.toml) 参照（自動生成）。

本ツールは [oj](https://github.com/online-judge-tools/oj) の再開発。

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
