import { parentPort } from "worker_threads";

let queue: any = [];

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

(async () => {
    parentPort?.on("message", queue.push);
    while (true) {
        if (!queue.length) {
            sleep(1000);
            continue;
        }

        for (const msg of queue) {
            parentPort?.postMessage(msg);
        }
    }
})()