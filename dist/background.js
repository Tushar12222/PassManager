console.log("Background script running......");

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    console.log("Received message:", request.message);
    console.log("Received data:", request.data);

    if (request.message == "Save") {
        // Saving data to chrome storage
        let object = {};
        object[request.data.website] = request.data;
        chrome.storage.sync.set(object, function () {
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
    } else if (request.message == "Delete") {
        chrome.storage.sync.remove([request.data], function() {
            var error = chrome.runtime.lastError;
            if (error) {
                console.error(error);
                sendResponse({ success: false});
            } else {
                console.log('Removed items for the key: ' + request.data);
                sendResponse({ success: true});
            }
        });
    };

    // Indicates that the response will be sent asynchronously
    return true;
});


