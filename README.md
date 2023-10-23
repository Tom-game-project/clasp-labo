# clasp を使ったGAS開発

## claspの扱いかた

下準備

```bash
npm init -y
npm install @types/google-apps-script
```

AppsScriptへのログイン

```bash
clasp login
```

プロジェクトの作成

```bash
mkdir project
cd project
clasp create
```

すでに存在しているプロジェクトにリンクさせる方法(clone)

```bash
clasp clone "project_ID"
```

GAS環境へ変更を反映させる(push)

```bash
clasp push
```

