## Running Example

Ensure you have setup and installed all the project dependencies.

```
npm install -g tauri
git clone https://github.com/tauri-apps/tauri
cd examples/react/next.js
yarn
cargo install tauri-cli
```

### Development

```
yarn dev & tauri dev
```

### Production

```
yarn build
yarn next export
tauri build
```
