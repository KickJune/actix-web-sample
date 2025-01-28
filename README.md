# Actix Web Sample

## 環境構築

### データベースとユーザーの作成

- 以下のコマンドでpsqlにログイン

```
psql postgres postgres
```

- パスワードを聞かれたら、postgresユーザーのパスワードを入力
- setup.sqlの中身をコピペして実行

### テーブルの作成

- 以下のコマンドでpsqlにログイン

```
psql sample_db sample_user
```
- パスワードを聞かれたら、`sample_pass`と入力
- schema.sqlの中身をコピペして実行
- データベースの構造が変わった時はこの手順を再実行してください


### Twigプラグインの設定

- Rust Roverで`*.tera`のファイルをTwigで開くように設定