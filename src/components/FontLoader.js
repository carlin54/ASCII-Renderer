import React, { Component } from 'react';
import { Button } from 'react-bootstrap';
import { Row, Col } from 'react-bootstrap';
import ColorPickerQueue from './ColorPickerQueue.js';

class FontLoader extends Component {
    constructor(props) {
        super(props);
        this.state = {
            fontName: '',
            fontFile: null,
            error: '',
            fontForegroundColors: [],
            fontBackgroundColors: [],
            isRendering: false,
        };

    }
    handleFontForegroundColorsChanged = (colors) => {
        console.log("handleFontColoursChanged", colors);
        this.setState({ fontForegroundColors: colors });
    };

    handleFontBackgroundColorsChanged = (colors) => {
        console.log("handleBackgroundColoursChanged", colors);
        this.setState({ fontBackgroundColors: colors });
    };

    handleFontChange = (e) => {
        const file = e.target.files[0];
        if (!file) return;

        if (file.type === 'font/ttf' || file.type === 'font/otf') {
            const fontName = file.name.split('.')[0];
            this.setState({
                fontName,
                fontFile: file,
                error: '',
            });

            this.props.onFontLoaded && this.props.onFontLoaded(file, fontName);
        } else {
            this.setState({ error: 'Please select a TTF or OTF font file' });
        }
    };

    handleStartRender() {

    }

    handleStopRender() {

    }

    canRender() {
        const { fontFile, fontForegroundColors, fontBackgroundColors} = this.state;
        console.log('fontForegroundColors', this.state.fontForegroundColors);
        console.log('fontBackgroundColors', this.state.fontBackgroundColors);
        return fontFile && fontForegroundColors.length > 0 && fontBackgroundColors.length > 0;
    }

    renderRenderButton() {
        const { isRendering } = this.state;
        return (
            <>
            {
                isRendering ? (
                    <Button variant="danger" onClick={this.handleStopRender}>
                        Stop
                    </Button>
                ) : (
                    <Button variant="primary" onClick={this.handleStartRender}>
                        Render
                    </Button>
                )
            }
            </>
        )

    }

    render() {
        const { fontName, fontFile, error } = this.state;

        return (
            <div>
                <h1 className="mb-4">Select Font</h1>
                <input
                    type="file"
                    accept=".ttf"
                    onChange={this.handleFontChange}
                    id="font-input"
                    style={{ display: 'none' }}
                />
                <label htmlFor="font-input">
                    <Button variant="primary" as="span">
                        Upload Font
                    </Button>
                </label>
                {fontName && (
                    <p className="mt-3">
                        Loaded font: {fontName}
                    </p>
                )}
                {error && (
                    <p className="text-danger mt-3">
                        {error}
                    </p>
                )}

                { fontFile && (
                    <Row className="mt-5">
                        <Col>
                            <ColorPickerQueue
                                title="Font Foreground Colors"
                                onColorsChanged={this.handleFontForegroundColorsChanged}
                            />
                        </Col>
                        <Col>
                            <ColorPickerQueue
                                title="Font Background Colors"
                                onColorsChanged={this.handleFontBackgroundColorsChanged}
                            />
                        </Col>
                    </Row>

                )}


                { this.canRender() && (
                    <Button variant="primary" as="span" className='mt-3' on>
                        Render
                    </Button>
                )}
            </div>
        );
    }
}

export default FontLoader;