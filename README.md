# aupfile

[AupDotNet v0.2.0](https://github.com/karoterra/AupDotNet/tree/v0.2.0) のRust移植。

## 使用例

```rust
use aupfile::AviUtlProject;

fn main() -> aupfile::Result<()> {
    let mut project = AviUtlProject::open("input.aup")?;
    project.edit_handle.output_filename = "output.avi".to_owned();
    project.save("output.aup")?;
    Ok(())
}
```

拡張編集データを編集する場合は、対象 FilterProject を型付きデータへ変換します。

```rust
use aupfile::AviUtlProject;
use aupfile::exedit::{Effect, EffectKind};

fn main() -> aupfile::Result<()> {
    let mut project = AviUtlProject::open("input.aup")?;
    let exedit = project
        .decode_exedit()?
        .ok_or(aupfile::AupError::InvalidModel("ExEdit project was not found"))?;

    let draw = Effect::from_kind(EffectKind::StandardDraw);
    println!("{}", draw.effect_type.name);
    println!("objects: {}", exedit.objects.len());
    Ok(())
}
```

より詳しい例は [`examples`](examples) を参照してください。

## 互換性

`testdata` は AupDotNet v0.2.0 の fixture です。`.aup` と拡張編集データの再読み込み後のバイト安定性、および upstream が生成する 10 個の `.exo` の完全一致を CI で検証します。

## ライセンス

MIT License。

## 謝辞

NOTICE.mdに元ライブラリの著作権表示を記載しています。

