
import * as import_b from './wasm_tutorial_browser';
let wasm;

export let memory;
export const booted = fetch('/wasm_tutorial_browser_bg.wasm')
    .then(res => res.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes,{ './wasm_tutorial_browser': import_b,  })
    .then(obj => {
        wasm = obj.instance;
        memory = wasm.exports.memory;
    })
);
export function __wbg_todo_free(a) {
        wasm.exports.__wbg_todo_free(a);
}

export function bincode_to_json(a) {
    return wasm.exports.bincode_to_json(a);
}

export function get_add_message(a) {
    return wasm.exports.get_add_message(a);
}

export function get_update_message(a, b, c) {
    return wasm.exports.get_update_message(a, b, c);
}

export function get_remove_message(a, b, c) {
    return wasm.exports.get_remove_message(a, b, c);
}

export function get_all_message() {
    return wasm.exports.get_all_message();
}

export function __wbindgen_malloc(a) {
    return wasm.exports.__wbindgen_malloc(a);
}

export function __wbindgen_free(a, b) {
        wasm.exports.__wbindgen_free(a, b);
}

export function __wbindgen_global_argument_ptr() {
    return wasm.exports.__wbindgen_global_argument_ptr();
}