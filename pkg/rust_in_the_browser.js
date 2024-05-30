import * as wasm from "./rust_in_the_browser_bg.wasm";
import { __wbg_set_wasm } from "./rust_in_the_browser_bg.js";
__wbg_set_wasm(wasm);
export * from "./rust_in_the_browser_bg.js";
