import * as wasm from "../pkg";

wasm.init();

document.onkeydown = e => wasm.send_key_down(e.keyCode);
document.onkeyup = e => wasm.send_key_up(e.keyCode);

const frame = () => {
	requestAnimationFrame(frame);
	wasm.frame();
};

frame();