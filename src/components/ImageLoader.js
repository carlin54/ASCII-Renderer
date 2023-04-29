import React, { Component } from 'react';
import Button from 'react-bootstrap/Button';

class ImageLoader extends Component {
    constructor(props) {
        super(props);
        this.state = {
            imageUrl: '',
            imageFile: null,
            error: '',
        };
    }

    handleImageChange = (e) => {
        const file = e.target.files[0];
        if (!file) return;

        if (file.type.startsWith('image/')) {
            this.setState({
                imageFile: file,
                imageUrl: URL.createObjectURL(file),
                error: '',
            });

            this.props.onImageLoaded && this.props.onImageLoaded(file);
        } else {
            this.setState({ error: 'Please select an image file' });
        }
    };

    render() {
        const { imageUrl, error } = this.state;

        return (
            <div>
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
            </div>
        );
    }
}

export default ImageLoader;
