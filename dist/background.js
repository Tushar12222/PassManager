console.log("Background script running......");

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    console.log("Received message:", request.message);
    console.log("Received data:", request.data);

    if (request.message == "Save") {
        // Saving data to chrome storage
        chrome.storage.sync.set({ "key": request.data }, function () {
            if (chrome.runtime.lastError) {
                console.log('Error: ' + chrome.runtime.lastError.message);
                sendResponse({ success: false });
            } else {
                console.log('Value is set to ' + JSON.stringify(request.data));
                // Sends back success
                sendResponse({ success: true });
            }
        });
    } else if (request.message == "Load") {
        chrome.storage.sync.get(null, function (items) {
            var allKeys = Object.keys(items);
            var listOfObjects = allKeys.map(function (key) {
                return items[key];
            });
            console.log(listOfObjects);
            sendResponse({ success: true, message: listOfObjects });
        });
    }

    // Indicates that the response will be sent asynchronously
    return true;
});


// window.send_message_to_background_script = function(message, data, callback) {
//     chrome.runtime.sendMessage({message: message, data: data}, callback);
// };