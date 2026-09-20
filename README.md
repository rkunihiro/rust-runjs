# runjs

Deno のエンジン (`deno_core` / V8) 上で JavaScript/TypeScript の「作業スクリプト」を実行する Rust 製 CLI。スクリプトから Rust の関数を呼び出すための `Native.*` API を備える。

## 使い方

```sh
cargo build --release
./target/release/runjs path/to/script.js
./target/release/runjs path/to/script.ts
./target/release/runjs path/to/script.js -- arg1 arg2   # Native.args() で取得可能
```

標準的な ECMAScript の組み込みオブジェクト (`Math`, `JSON`, `Date`, `Promise`, `Array`, `RegExp`, `console.*` など) は V8 が提供するものがそのまま使える。スクリプトは ES モジュールとして実行されるため、ローカルの `import "./other.ts"` が使える。対応するのは `file://`/ローカルパス指定のみ (bare/npm 形式やリモートの import は非対応)。

TypeScript は `deno_ast` により型情報を除去してトランスパイルされる (Deno の `--no-check` と同様の動作)。実際の型チェックは未実装で、`--check` を指定すると黙ってスキップせず明示的にエラーになる。

## ネイティブ API

スクリプトは `Native` グローバルを通じて Rust を呼び出せる:

- `Native.readTextFile(path)`
- `Native.writeTextFile(path, contents)`
- `Native.args()` — CLI の `--` 以降に渡した引数

ネイティブ op のエラーは JS の例外として現れる (`try`/`catch` で捕捉できる)。

### ネイティブ関数を追加する手順

1. `crates/runjs-runtime/src/ops/*.rs` に、`#[op2]` (非同期なら `#[op2(async)]`) を付けた素の Rust 関数を書く。
2. `crates/runjs-runtime/src/ext.rs` の `ops = [...]` リストに追加する。
3. `crates/runjs-runtime/src/js/00_bootstrap.js` の `Native` に1行のラッパーを追加する。
4. ユニットテスト (`ops/fs_ops.rs` を参照) や、`crates/runjs-cli/tests/integration_test.rs` から実行される `tests/fixtures/` 配下のフィクスチャスクリプトを追加する。

注意: ブートストラップ用の `.js` ファイルだけを編集しても再ビルドはトリガーされない (cargo が依存関係として追跡していないため) — `ext.rs` を touch するか、先に `cargo clean -p runjs-runtime` を実行すること。

## 対応プラットフォーム (v1)

macOS と Linux のみ、それぞれ OS ごとにネイティブビルドする (クロスコンパイルは行わない)。`v8` クレートはこれらのターゲット向けに事前ビルド済みバイナリをダウンロードする。それ以外のターゲットでは低速なソースからのビルドにフォールバックする。Windows 対応は今後のマイルストーン。

## テスト

```sh
cargo test
```

op 単位のユニットテスト (`runjs-runtime`) と、`tests/fixtures/` のフィクスチャに対するエンドツーエンドのCLIテスト (`runjs-cli`) を実行する。

