import * as wasm from "rsstv";

let decoder = wasm.SSTVDecoderWASM.new();

let image_in = document.getElementById("image to encode");
image_in.addEventListener("change", newFiles);

function newFiles() {
    let file = image_in.files[-1];

    let preview = document.createElement("img");

    preview.src = URL.createObjectURL(file)
}
