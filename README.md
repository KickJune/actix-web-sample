# Actix Web Sample

## 環境構築

### データベースの作成

psqlにログイン

```
psql postgres postgres
```

setup.sqlの中身をコピペして実行

### テーブルの作成

psqlにログイン

```
psql sample_db sample_user
```

schema.sqlの中身をコピペして実行

### Askamaプラグインの導入

Rust Roverで[Askama Template Support](https://plugins.jetbrains.com/plugin/16591-askama-template-support)をインストール