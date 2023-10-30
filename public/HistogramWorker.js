self.onmessage = function (e) {
    const { imageData, bucketSize } = e.data;



    // Include the logic for generating the histogram
    // Be aware that you won't have access to the DOM or other window-specific features

    // Sample histogram generation logic
    const histogramData = generateHistogram(imageData, bucketSize); // Implement this function based on your requirements

    // Post the data back to the main thread
    postMessage(histogramData);
};

function rgbDistance(rgb1, rgb2) {
    const [r1, g1, b1] = rgb1.split('-').map(Number);
    const [r2, g2, b2] = rgb2.split('-').map(Number);
    return Math.sqrt((r1 - r2) ** 2 + (g1 - g2) ** 2 + (b1 - b2) ** 2);
}

function generateHistogram(imageData, bucketSize) {
    const histogram = {};

    for (let i = 0; i < imageData.length; i += 4) {
        const key = `${imageData[i]}-${imageData[i + 1]}-${imageData[i + 2]}`;
        histogram[key] = (histogram[key] || 0) + 1;
    }

    const sortedKeys = Object.entries(histogram)
        .sort((a, b) => b[1] - a[1])
        .map(entry => entry[0]);

    const bucketedHistogram = {};

    while (sortedKeys.length > 0) {
        const currentKey = sortedKeys.shift();
        bucketedHistogram[currentKey] = histogram[currentKey];

        let i = 0;
        while (i < sortedKeys.length) {
            if (rgbDistance(currentKey, sortedKeys[i]) < bucketSize) {
                bucketedHistogram[currentKey] += histogram[sortedKeys[i]];
                sortedKeys.splice(i, 1); // Remove the key from sortedKeys
            } else {
                i++;
            }
        }
    }

    return bucketedHistogram;
}