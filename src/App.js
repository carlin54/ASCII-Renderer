import React, { useState } from 'react';
import { Container, Row, Col } from 'react-bootstrap';
import ImageLoader from './components/ImageLoader.js';
import FontLoader from './components/FontLoader.js';
import ColorPickerQueue from './components/ColorPickerQueue.js';


const App = () => {
  const [imageData, fontData] = useState(null);

  const handleImageLoaded = (data) => {
    //setImageData(data);
  };
    const handleFontLoaded = async (file, fontName) => {
        //await init();
        const reader = new FileReader();
        reader.onload = async (event) => {
            const arrayBuffer = event.target.result;
            //const fontInfo = await font_info(new Uint8Array(arrayBuffer));
            //console.log(fontInfo);
        };
        reader.readAsArrayBuffer(file);
    };

    return (
        <dir>
            <Container className="mt-5">

                <ImageLoader onImageLoaded={handleImageLoaded} />
            </Container>

            <Container className="mt-5">
                <FontLoader onFontLoaded={handleFontLoaded} />

            </Container>


            <Container className="mt-5">
                <h1 className="mb-4">Run Job</h1>
                {

                }
            </Container>

        </dir>
    );
};

export default App;