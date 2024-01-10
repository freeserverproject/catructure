# catructure
指定されたブロックがstructure内で使用されてないか確認するツール

## Features
- [x] .nbt(structure)を読み込む
- [x] blockをブロックリストに追加する
  - [x] コンフィグファイル(.toml)
  - [ ] Contaier持ちのブロックの制限
  - [x] entityの制限
  - [x] paletteの対応
  - [x] palettesの対応

## How to use
help
```sh
catructure --help
```

check
```sh
catructure check <StructureFilepath>
# Example
catructure check assets/entrance_connector.nbt
```

## その他
とりあえずCLIで書いてるけどどの様な感じで使用するかを決めろバカ
