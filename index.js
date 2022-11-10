import * as wasm from "hello-wasm-pack";

var data = wasm.Data.new()

function tick(){

    data.tick()
    window.requestAnimationFrame(tick)
}
window.requestAnimationFrame(tick)
