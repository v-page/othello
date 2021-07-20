import * as wasm from './othello_bg.wasm';

/**
*/
export function othello_init() {
    wasm.othello_init();
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @returns {string}
*/
export function othello_board() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.othello_board(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {number} player_num
* @returns {number}
*/
export function othello_count_cell(player_num) {
    var ret = wasm.othello_count_cell(player_num);
    return ret;
}

/**
* @returns {number}
*/
export function othello_turn() {
    var ret = wasm.othello_turn();
    return ret;
}

/**
* @returns {string}
*/
export function othello_placeable_list() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.othello_placeable_list(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {number} x
* @param {number} y
* @returns {number}
*/
export function othello_set(x, y) {
    var ret = wasm.othello_set(x, y);
    return ret;
}

