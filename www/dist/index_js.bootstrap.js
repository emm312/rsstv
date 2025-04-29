"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || []).push([["index_js"],{

/***/ "../pkg/rsstv.js":
/*!***********************!*\
  !*** ../pkg/rsstv.js ***!
  \***********************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   SSTVDecoderWASM: () => (/* reexport safe */ _rsstv_bg_js__WEBPACK_IMPORTED_MODULE_0__.SSTVDecoderWASM),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _rsstv_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   __wbindgen_init_externref_table: () => (/* reexport safe */ _rsstv_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_init_externref_table),\n/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _rsstv_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)\n/* harmony export */ });\n/* harmony import */ var _rsstv_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./rsstv_bg.wasm */ \"../pkg/rsstv_bg.wasm\");\n/* harmony import */ var _rsstv_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rsstv_bg.js */ \"../pkg/rsstv_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_rsstv_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_rsstv_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n\n(0,_rsstv_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_rsstv_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n_rsstv_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.__wbindgen_start();\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/../pkg/rsstv.js?");

/***/ }),

/***/ "../pkg/rsstv_bg.js":
/*!**************************!*\
  !*** ../pkg/rsstv_bg.js ***!
  \**************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   SSTVDecoderWASM: () => (/* binding */ SSTVDecoderWASM),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   __wbindgen_init_externref_table: () => (/* binding */ __wbindgen_init_externref_table),\n/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8ArrayMemory0 = null;\n\nfunction getUint8ArrayMemory0() {\n    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {\n        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8ArrayMemory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));\n}\n\nlet cachedFloat32ArrayMemory0 = null;\n\nfunction getFloat32ArrayMemory0() {\n    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {\n        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);\n    }\n    return cachedFloat32ArrayMemory0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArrayF32ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 4, 4) >>> 0;\n    getFloat32ArrayMemory0().set(arg, ptr / 4);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);\n}\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1, 1) >>> 0;\n    getUint8ArrayMemory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nfunction getArrayF32FromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);\n}\n\nconst SSTVDecoderWASMFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_sstvdecoderwasm_free(ptr >>> 0, 1));\n/**\n * A struct providing easy to use JS bindings for the `MartinM1` struct\n *\n * TODO: Generalise this type to work with any `impl SSTVMode`\n */\nclass SSTVDecoderWASM {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(SSTVDecoderWASM.prototype);\n        obj.__wbg_ptr = ptr;\n        SSTVDecoderWASMFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        SSTVDecoderWASMFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_sstvdecoderwasm_free(ptr, 0);\n    }\n    /**\n     * @returns {SSTVDecoderWASM}\n     */\n    static new() {\n        const ret = wasm.sstvdecoderwasm_new();\n        return SSTVDecoderWASM.__wrap(ret);\n    }\n    /**\n     * @param {Float32Array} buf\n     * @returns {Uint8Array | undefined}\n     */\n    decode(buf) {\n        const ptr0 = passArrayF32ToWasm0(buf, wasm.__wbindgen_malloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ret = wasm.sstvdecoderwasm_decode(this.__wbg_ptr, ptr0, len0);\n        let v2;\n        if (ret[0] !== 0) {\n            v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();\n            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);\n        }\n        return v2;\n    }\n    /**\n     * @param {Uint8Array} image\n     * @returns {Float32Array}\n     */\n    encode(image) {\n        const ptr0 = passArray8ToWasm0(image, wasm.__wbindgen_malloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ret = wasm.sstvdecoderwasm_encode(this.__wbg_ptr, ptr0, len0);\n        var v2 = getArrayF32FromWasm0(ret[0], ret[1]).slice();\n        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);\n        return v2;\n    }\n}\n\nfunction __wbindgen_init_externref_table() {\n    const table = wasm.__wbindgen_export_0;\n    const offset = table.grow(4);\n    table.set(0, undefined);\n    table.set(offset + 0, undefined);\n    table.set(offset + 1, null);\n    table.set(offset + 2, true);\n    table.set(offset + 3, false);\n    ;\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack://create-wasm-app/../pkg/rsstv_bg.js?");

/***/ }),

/***/ "../pkg/rsstv_bg.wasm":
/*!****************************!*\
  !*** ../pkg/rsstv_bg.wasm ***!
  \****************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./rsstv_bg.js */ \"../pkg/rsstv_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"e2b56fdecf837e171171\", {\n\t\"./rsstv_bg.js\": {\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw,\n\t\t\"__wbindgen_init_externref_table\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_init_externref_table\n\t}\n});\n\n//# sourceURL=webpack://create-wasm-app/../pkg/rsstv_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var rsstv__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rsstv */ \"../pkg/rsstv.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([rsstv__WEBPACK_IMPORTED_MODULE_0__]);\nrsstv__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\nlet decoder = rsstv__WEBPACK_IMPORTED_MODULE_0__.SSTVDecoderWASM.new();\n\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/./index.js?");

/***/ })

}]);