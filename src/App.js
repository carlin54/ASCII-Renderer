import React, { useState } from 'react';
import { Container, Row, Col } from 'react-bootstrap';
import ImageLoader from './components/ImageLoader.js';
import FontLoader from './components/FontLoader.js';
import ColorPickerQueue from './components/ColorPickerQueue.js';
import init, {font_info} from './pkg/ASCII_Renderer.js'

const App = () => {
  const [imageData, setImageData] = useState(null);

  const handleImageLoaded = (data) => {
    setImageData(data);
  };
    const handleFontLoaded = async (file, fontName) => {
        await init();
        const reader = new FileReader();
        reader.onload = async (event) => {
            const arrayBuffer = event.target.result;
            const fontInfo = await font_info(new Uint8Array(arrayBuffer));
            console.log(fontInfo);
        };
        reader.readAsArrayBuffer(file);
    };

    const handleFontColorAdded = (color) => {
        console.log('Font color added:', color);
    };

    const handleFontBackgroundColorAdded = (color) => {
        console.log('Font background color added:', color);
    };
    return (
        <dir>
            <Container className="mt-5">
                <h1 className="mb-4">Image Loader</h1>
                <ImageLoader onImageLoaded={handleImageLoaded} />
                <FontLoader onFontLoaded={handleFontLoaded} />
            </Container>

            <Container className="mt-5">
                <h1 className="mb-4">Font and Color Selection</h1>
                <Row>
                    <Col>
                        <ImageLoader />
                    </Col>
                    <Col>
                        <FontLoader />
                    </Col>
                </Row>
                <Row className="mt-5">
                    <Col>
                        <ColorPickerQueue
                            title="Font Colors"
                            onColorAdded={handleFontColorAdded}
                        />
                    </Col>
                    <Col>
                        <ColorPickerQueue
                            title="Font Background Colors"
                            onColorAdded={handleFontBackgroundColorAdded}
                        />
                    </Col>
                </Row>
            </Container>
        </dir>
    );
};

export default App;