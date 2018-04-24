/* tslint:disable */
import * as wasm from './wasm_tutorial_init';

const __wbg_f_log_log_n_target = console.log;

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachedUint8Memory = null;
function getUint8Memory() {
    if (cachedUint8Memory === null ||
        cachedUint8Memory.buffer !== wasm.memory.buffer)
        cachedUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachedUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().slice(ptr, ptr + len));
}

let cachedUint32Memory = null;
function getUint32Memory() {
    if (cachedUint32Memory === null ||
        cachedUint32Memory.buffer !== wasm.memory.buffer)
        cachedUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachedUint32Memory;
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null)
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    return cachedGlobalArgumentPtr;
}

function getGlobalArgument(arg) {
    const idx = globalArgumentPtr() / 4 + arg;
    return getUint32Memory()[idx];
}

export function __wbg_f_log_log_n(arg0) {
    let len0 = getGlobalArgument(0);
    let v0 = getStringFromWasm(arg0, len0);
    __wbg_f_log_log_n_target(v0);
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length);
    getUint8Memory().set(arg, ptr);
    return [ptr, arg.length];
}

function setGlobalArgument(arg, i) {
    const idx = globalArgumentPtr() / 4 + i;
    getUint32Memory()[idx] = arg;
}

export function bincode_to_json(arg0) {
    const [ptr0, len0] = passArray8ToWasm(arg0);
    setGlobalArgument(len0, 0);
    const ret = wasm.bincode_to_json(ptr0);
    const len = getGlobalArgument(0);
    const realRet = getStringFromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 1);
    return realRet;
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

function getArrayU8FromWasm(ptr, len) {
    const mem = getUint8Memory();
    const slice = mem.slice(ptr, ptr + len);
    return new Uint8Array(slice);
}

export function get_add_message(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    setGlobalArgument(len0, 0);
    const ret = wasm.get_add_message(ptr0);
    const len = getGlobalArgument(0);
    const realRet = getArrayU8FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 1);
    return realRet;
}

export function get_update_message(arg0, arg1, arg2) {
    const [ptr2, len2] = passStringToWasm(arg2);
    setGlobalArgument(len2, 0);
    const ret = wasm.get_update_message(arg0, arg1 ? 1 : 0, ptr2);
    const len = getGlobalArgument(0);
    const realRet = getArrayU8FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 1);
    return realRet;
}

export function get_remove_message(arg0, arg1, arg2) {
    const [ptr2, len2] = passStringToWasm(arg2);
    setGlobalArgument(len2, 0);
    const ret = wasm.get_remove_message(arg0, arg1 ? 1 : 0, ptr2);
    const len = getGlobalArgument(0);
    const realRet = getArrayU8FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 1);
    return realRet;
}

export function get_all_message() {
    const ret = wasm.get_all_message();
    const len = getGlobalArgument(0);
    const realRet = getArrayU8FromWasm(ret, len);
    wasm.__wbindgen_free(ret, len * 1);
    return realRet;
}

export class ToDo {

    static __construct(ptr) {
        return new ToDo(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        wasm.__wbg_todo_free(ptr);
    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

