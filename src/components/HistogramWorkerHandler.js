export default class HistogramWorkerHandler {
    constructor() {
        this.worker = new Worker('HistogramWorker.js');
        this.worker.onmessage = this.handleMessage.bind(this);
        this.worker.onerror = this.handleError.bind(this);
    }

    generateHistogram(imageData, bucketSize, callback) {
        this.callback = callback;
        this.worker.postMessage({ imageData, bucketSize });
    }

    handleMessage(e) {
        if (this.callback) {
            this.callback(e.data);
        }
    }

    handleError(error) {
        console.error('Worker error:', error);
    }

    terminate() {
        this.worker.terminate();
    }
}
