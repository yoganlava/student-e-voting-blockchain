const { parentPort } = require("worker_threads");

let queue = [];

function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

// self.onmessage = (m) => {
//     console.log("m");
//     console.log(m);
// };

(async () => {
    parentPort.on("message", (message) => {
        console.log("[Queue Worker] Recieved message");
        console.log(message);
        queue.push(message);
    });
    while (true) {
        if (!queue.length) {
            await sleep(500);
            continue;
        }

        while (queue.length) {
            parentPort?.postMessage(queue.shift());
        }
    }
})();
