
## About

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) 
using Rust and WebAssembly.

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### Install dependencies with `npm`

```bash
cd www && npm install
```

### Run locally

```bash
npm run start
```

### 🔬 Test in Headless Browsers with `wasm-pack test`
For firefox
```
wasm-pack test --headless --firefox
```

OR

For chrome
```
wasm-pack test --chrome --headless
```

## Known Issues
- If the error in the image below occurs when running `npm run start` on MacOS, it can be fixed by running 
  `export NODE_OPTIONS=--openssl-legacy-provider`.

  ![ssl issue image](error.png).

- Testing with Chrome may fail due to issues with chromedriver version compatibility. You can see more info about this [here](https://github.com/rustwasm/wasm-pack/issues/611). 
To fix:
  - Check the version of chrome you have by opening Chrome and going to `Settings > About Chrome`
  - Check the compatible Chrome version by looking [here](https://developer.chrome.com/docs/chromedriver/downloads/version-selection#for_versions_115_and_newer)
  - Download and replace the the executable which is usually located in /Users/<`USERNAME`>/Library/Caches/.wasm-pack/chromedriver-1b467be6b1263401 (replace USERNAME with the appropriate username for your machine).


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.

<div align="center">

<sub>Built with 🦀🕸 </a></sub>

</div>
