import HistogramWorkerHandler from './HistogramWorkerHandler';

import React, { Component } from 'react';
import { FormGroup } from 'react-bootstrap';
import { Row, Col, Tooltip, OverlayTrigger } from 'react-bootstrap';
import Button from 'react-bootstrap/Button';
import { Bar } from 'react-chartjs-2';
import { Chart, registerables } from 'chart.js';
Chart.register(...registerables);


class ErrorBoundary extends Component {
    state = { hasError: false };

    static getDerivedStateFromError(error) {
        return { hasError: true };
    }

    componentDidCatch(error, info) {
        console.error("Caught an error:", error, info);
    }

    render() {
        if (this.state.hasError) {
            return <h1>Something went wrong.</h1>;
        }
        return this.props.children;
    }
}

class ImageLoader extends Component {
    constructor(props) {
        super(props);
        this.state = {
            imageUrl: '',
            imageFile: null,
            error: '',
            histogramData: null,
            selectedColor: null,
            bucketSize: 32,
            showBucketSizeTooltip: false,
            mouseX: 0,
            mouseY: 0,
            imageData: null,
        };
        this.imageRef = React.createRef();
        this.chartInstance = null;
    }

    handleMouseMove = (e) => {
        this.setState({
            mouseX: e.clientX,
            mouseY: e.clientY
        });
    };

    toggleBucketSizeTooltip = () => {
        this.setState({ showBucketSizeTooltip: !this.state.showBucketSizeTooltip });
    };

    handleImageChange = (e) => {
        const file = e.target.files[0];
        if (!file) return;

        if (file.type.startsWith('image/')) {
            this.setState({
                imageFile: file,
                imageUrl: URL.createObjectURL(file),
                error: '',
                img_src: null
            });

            this.loadImage(file);
            this.props.onImageLoaded && this.props.onImageLoaded(file);
        } else {
            this.setState({ error: 'Please select an image file' });
        }
    };

    loadImage = (file) => {
        const reader = new FileReader();

        reader.onload = (e) => {
            const img = new Image();
            img.onload = () => { // Make sure image is loaded
                const canvas = document.createElement('canvas');
                canvas.width = img.width;
                canvas.height = img.height;
                const ctx = canvas.getContext('2d');
                ctx.drawImage(img, 0, 0);

                try {
                    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
                    this.setState({imageData}, () => {
                        this.generateHistogramData();
                    });
                } catch (error) {
                    this.setState({error: 'Error processing the image.'});
                    console.error('Error in getImageData:', error);
                }
            };

            img.onerror = () => {
                this.setState({ error: 'Error loading the image.' });
            };

            img.src = e.target.result; // Set source after defining onload
        };
        reader.readAsDataURL(file);
    }

    // Make sure to terminate the worker when the component unmounts
    componentWillUnmount() {
        if (this.histogramWorkerHandler) {
            this.histogramWorkerHandler.terminate();
        }
    }

    generateHistogramData = () => {

        //if (!this.histogramWorkerHandler) {
        this.histogramWorkerHandler = new HistogramWorkerHandler();
        //}

        this.histogramWorkerHandler.generateHistogram(
            this.state.imageData,
            this.state.bucketSize,
            (histogramData) => {
                this.setState({ histogramData });
            }
        );

    }

    handleHover = (event, chartElement) => {
        if (chartElement.length > 0) {
            const data = {
                labels: Object.keys(this.state.histogramData || {}),
                datasets: [
                    {
                        data: Object.values(this.state.histogramData || {}),
                        backgroundColor: Object.keys(this.state.histogramData || {}).map(
                            (key) => `rgb(${key.split('-').join(',')})`
                        ),
                    },
                ],
            };
            const index = chartElement[0].index;
            this.setState({ selectedColor: '255-255-255' });
        }
    };

    handleBucketSizeChange = (newSize) => {
        this.setState({ bucketSize: newSize }, () => {
            // Re-generate histogram data with new bucket size
            this.generateHistogramData();
        });
    };


    renderHistogram() {

        const { bucketSize, showBucketSizeTooltip, histogramData, selectedColor } = this.state;
        const tooltipBucketSizeStyle = {
            visibility: showBucketSizeTooltip ? 'visible' : 'hidden',
            backgroundColor: 'black',
            color: 'white',
            textAlign: 'center',
            borderRadius: '6px',
            padding: '5px',
            position: 'fixed',
            left: `${this.state.mouseX + 15}px`, // 15px to offset from cursor
            top: `${this.state.mouseY + 15}px`,
            zIndex: 1000,
        };

        const data = {
            labels: Object.keys(histogramData || {}),
            datasets: [
                {
                    data: Object.values(histogramData || {}),
                    backgroundColor: Object.keys(histogramData || {}).map(
                        (key) => `rgb(${key.split('-').join(',')})`
                    ),
                },
            ],
        };

        const options = {
            scales: {
                x: {
                    type: 'category',
                },
            },
            onHover: this.handleHover,
            plugins: {
                title: {
                    display: true,
                    text: 'Pixel Colour Histogram'
                },
                legend: {
                    display: false, // This hides the legend
                },
            }
        };

        return (
            <>
                <Row className="mb-3">
                    <Col>
                        <FormGroup>
                        <div
                            onMouseMove={this.handleMouseMove}
                            onMouseEnter={() => this.toggleBucketSizeTooltip(true)}
                            onMouseLeave={() => this.toggleBucketSizeTooltip(false)}
                        >
                            <label>
                                Bucket Size:
                                <input
                                    type="number"
                                    value={bucketSize}
                                    onChange={(e) => this.handleBucketSizeChange(e.target.value)}
                                    style={{ marginLeft: '10px' }}
                                />
                            </label>
                            <div style={tooltipBucketSizeStyle}>
                                Adjust the bin size for the pixels to be grouped together on the histogram.
                            </div>
                        </div>
                        </FormGroup>
                    </Col>
                </Row>
                <Row className="mb-3">
                    <Col>
                        {histogramData && (
                            <Bar ref={ref => {
                                if (ref && ref.chart && this.chartInstance !== ref.chart) {
                                    if (this.chartInstance) {
                                        this.chartInstance.destroy();
                                    }
                                    this.chartInstance = ref.chart;
                                }
                            }} data={data} options={options} />
                        )}
                    </Col>
                </Row>

                {}
            </>
        )
    }
    render() {
        const { imageUrl, error, imageData } = this.state;

        return (
            <div>
                <h1 className="mb-4">Select Image</h1>
                <Row className="mb-3">
                    <Col>
                        <input
                            type="file"
                            accept="image/*"
                            onChange={this.handleImageChange}
                            id="image-input"
                            style={{ display: 'none' }}
                        />
                        <label htmlFor="image-input">
                            <Button variant="primary" as="span">
                                Upload Image
                            </Button>
                        </label>
                    </Col>
                </Row>
                <Row className="mb-3">
                    <Col>
                        {imageUrl && (
                            <img
                                src={imageUrl}
                                alt="Uploaded"
                                className="img-fluid mt-3"
                                style={{ maxHeight: '300px' }}
                            />
                        )}
                        {error && (
                            <p className="text-danger mt-3">
                                {error}
                            </p>
                        )}
                    </Col>
                </Row>

                {imageUrl &&
                    this.renderHistogram()
                }

            </div>
        );
    }
}

export default ImageLoader;