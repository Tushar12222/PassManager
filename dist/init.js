import init, * as bindings from '/passmanager-7c5e362a42362eba.js';
window.send_message_to_background_script = function(message, data, callback) {
    chrome.runtime.sendMessage({message: message, data: data}, callback);
};
init('/passmanager-7c5e362a42362eba_bg.wasm');
window.wasmBindings = bindings;