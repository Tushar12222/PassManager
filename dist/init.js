import init, * as bindings from '/passmanager-eb31c8ac1e34e051.js';
window.send_message_to_background_script = function(message, data, callback) {
    chrome.runtime.sendMessage({message: message, data: data}, callback);
};
init('/passmanager-eb31c8ac1e34e051_bg.wasm');
window.wasmBindings = bindings;