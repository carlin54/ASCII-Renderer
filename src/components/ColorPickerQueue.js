import React, { useState } from 'react';
import { Button, ListGroup, ListGroupItem, Row, Col } from 'react-bootstrap';
import { SketchPicker } from 'react-color';

const ColorPickerQueue = ({ title, onColorsChanged }) => {
    const [colors, setColors] = useState([]);
    const [showPicker, setShowPicker] = useState(false);
    const [currentColor, setCurrentColor] = useState('#000000');

    const handleColorChange = (color) => {
        setCurrentColor(color.hex);
    };

    const addColor = () => {
        setColors((prevColors) => {
            const newColors = [...prevColors, currentColor];
            onColorsChanged && onColorsChanged(newColors);
            return newColors;
        });
    };

    const removeColor = (index) => {
        setColors((prevColors) => {
            const newColors = prevColors.filter((_, i) => i !== index);
            onColorsChanged && onColorsChanged(newColors);
            return newColors;
        });
    };

    return (
        <div>
            <h5>{title}</h5>
            <Row className="align-items-center">
                <Col>
                    <Button variant="primary" onClick={() => setShowPicker(!showPicker)}>
                        {showPicker ? 'Close' : 'Open'} Color Picker
                    </Button>
                </Col>
            </Row>

            {showPicker && (
                <Row className="align-items-center">
                    <Col>
                        <SketchPicker color={currentColor} onChangeComplete={handleColorChange} />
                    </Col>
                </Row>
            )}
            <Row className="align-items-center">
                <Col>
                    <Button className="mt-3" variant="success" onClick={addColor}>
                        Add Color to Queue
                    </Button>
                </Col>
            </Row>
            <ListGroup className="mt-3">
                {colors.map((color, index) => (
                    <ListGroupItem key={index} style={{ backgroundColor: color }}>
                        <Row>
                            <Col>{color}</Col>
                            <Col xs="auto">
                                <Button variant="danger" size="sm" onClick={() => removeColor(index)}>
                                    Delete
                                </Button>
                            </Col>
                        </Row>
                    </ListGroupItem>
                ))}
            </ListGroup>
        </div>
    );
};

export default ColorPickerQueue;
