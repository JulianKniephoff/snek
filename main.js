/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + ({}[chunkId]||chunkId) + ".js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/snek_bg.wasm": function() {
/******/ 			return {
/******/ 				"./snek": {
/******/ 					"__wbindgen_cb_forget": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_cb_forget"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_error_cc95a3d302735ca3": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_error_cc95a3d302735ca3"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_property_CSSStyleDeclaration": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_set_property_CSSStyleDeclaration"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D": function(p0i32,p1i32,p2f64,p3f64,p4i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_draw_image_with_image_bitmap_CanvasRenderingContext2D"](p0i32,p1i32,p2f64,p3f64,p4i32);
/******/ 					},
/******/ 					"__widl_f_begin_path_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_begin_path_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_stroke_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_stroke_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_line_cap_CanvasRenderingContext2D": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_set_line_cap_CanvasRenderingContext2D"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_line_to_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_line_to_CanvasRenderingContext2D"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__widl_f_move_to_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_move_to_CanvasRenderingContext2D"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__widl_f_clear_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_clear_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_fill_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_fill_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_stroke_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_stroke_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_restore_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_restore_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_save_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_save_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_scale_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_scale_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3i32);
/******/ 					},
/******/ 					"__widl_f_translate_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_translate_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3i32);
/******/ 					},
/******/ 					"__widl_f_create_element_Document": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_create_element_Document"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__widl_f_get_elements_by_tag_name_Document": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_get_elements_by_tag_name_Document"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_body_Document": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_body_Document"](p0i32);
/******/ 					},
/******/ 					"__widl_f_add_event_listener_with_callback_EventTarget": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_add_event_listener_with_callback_EventTarget"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__widl_f_get_context_HTMLCanvasElement": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_get_context_HTMLCanvasElement"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__widl_f_width_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_width_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_width_HTMLCanvasElement": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_set_width_HTMLCanvasElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_height_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_height_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_height_HTMLCanvasElement": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_set_height_HTMLCanvasElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_item_HTMLCollection": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_item_HTMLCollection"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_style_HTMLElement": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_style_HTMLElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_key_KeyboardEvent": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_key_KeyboardEvent"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_append_child_Node": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_append_child_Node"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_new_OffscreenCanvas": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_new_OffscreenCanvas"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_get_context_OffscreenCanvas": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_get_context_OffscreenCanvas"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__widl_f_transfer_to_image_bitmap_OffscreenCanvas": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_transfer_to_image_bitmap_OffscreenCanvas"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_now_Performance": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_now_Performance"](p0i32);
/******/ 					},
/******/ 					"__widl_instanceof_Window": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_instanceof_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_request_animation_frame_Window": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_request_animation_frame_Window"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_document_Window": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_document_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_inner_width_Window": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_inner_width_Window"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_inner_height_Window": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_inner_height_Window"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_device_pixel_ratio_Window": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_device_pixel_ratio_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_performance_Window": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__widl_f_performance_Window"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_43c5f57b77232284": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_newnoargs_43c5f57b77232284"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_7ac13208e630ddeb": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_call_7ac13208e630ddeb"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_886f15c1b20b061b": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_new_886f15c1b20b061b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_a2b503e0ee1234e4": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_call_a2b503e0ee1234e4"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_ddd2d80076091e5f": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_self_ddd2d80076091e5f"](p0i32);
/******/ 					},
/******/ 					"__wbg_crypto_4b7669ff1793d881": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_crypto_4b7669ff1793d881"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_6de85818bd2ad699": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_getRandomValues_6de85818bd2ad699"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_95cef5eed1acafda": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_getRandomValues_95cef5eed1acafda"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_require_86edd37cfda5f13d": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_require_86edd37cfda5f13d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_571502126f344d60": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbg_randomFillSync_571502126f344d60"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_jsval_eq": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_jsval_eq"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper91": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_closure_wrapper91"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper93": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_closure_wrapper93"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper95": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_closure_wrapper95"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_defer_start": function() {
/******/ 						return installedModules["./pkg/snek.js"].exports["__wbindgen_defer_start"]();
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							var error = new Error('Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')');
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["./pkg/snek_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/snek_bg.wasm":"d243204ade6e070ce453"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./src/index.js":
/*!**********************!*\
  !*** ./src/index.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ../pkg/snek */ \"./pkg/snek.js\"));\n\n\n//# sourceURL=webpack:///./src/index.js?");

/***/ })

/******/ });