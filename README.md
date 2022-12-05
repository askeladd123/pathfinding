![](res/v1.png)
# wasm_party
Here is the code I made while learning to use WebAssembly.
It runs and visualizes a pathfinding algorithm in the browser.

## Run online
Click this link: [github.io/pathfinding](https://askeladd123.github.io/pathfinding/).
Requires JavaScript and WebAssembly support; should work in most browsers.

---

## Run locally
To run locally, you need [wasm-pack](https://github.com/rustwasm/wasm-pack) to build. You need a *http server* to load the code, and to run it. I use [nvm http-server](https://www.npmjs.com/package/http-server). Run these commands in the root folder:
- `wasm-pack build --target web`
- `http-server`

The program should now run in [localhost:8080]().

## Potential problems
- **python http.server** didn't work for me