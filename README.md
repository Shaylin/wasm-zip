# Doggy Bag

## 🚴 Usage

### 🐑 Unit Test with `cargo test`
```
cargo test
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🪲 Test with Test Web App

In the parent directory of this project (i.e. one folder up), run:
```
npm init wasm-app my-test-wasm-app
cd my-test-wasm-app
npm remove hello-wasm-pack --save
npm install ../doggy-bag/pkg/ --save
```

Then edit the first line of the import index.js file of the test app to be:
```javascript
import * as wasm from "doggy-bag";
```

The following lines should call any exported functions from the web assembly such as:
```javascript
wasm.generate_zip_blob();
```

Finally, run the app with:
```
npm run start
```