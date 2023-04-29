import React, { Component } from 'react';
import { Button } from 'react-bootstrap';


class FontLoader extends Component {
    constructor(props) {
        super(props);
        this.state = {
            fontName: '',
            fontFile: null,
            error: '',
        };
    }

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

    render() {
        const { fontName, error } = this.state;

        return (
            <div>
                <input
                    type="file"
                    accept=".ttf,.otf"
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
            </div>
        );
    }
}

export default FontLoader;