import React, { useState } from 'react';
import { Button, ListGroup, ListGroupItem, Row, Col } from 'react-bootstrap';
import { SketchPicker } from 'react-color';

const ColorPickerQueue = ({ title, onColorAdded }) => {
    const [colors, setColors] = useState([]);
    const [showPicker, setShowPicker] = useState(false);
    const [currentColor, setCurrentColor] = useState('#000000');

    const handleColorChange = (color) => {
        setCurrentColor(color.hex);
    };

    const addColorToQueue = () => {
        setColors([...colors, currentColor]);
        onColorAdded && onColorAdded(currentColor);
    };

    const deleteColorFromQueue = (index) => {
        setColors(colors.filter((_, i) => i !== index));
    };

    return (
        <div>
            <h5>{title}</h5>
            <Button variant="primary" onClick={() => setShowPicker(!showPicker)}>
                {showPicker ? 'Close' : 'Open'} Color Picker
            </Button>
            {showPicker && (
                <SketchPicker
                    color={currentColor}
                    onChangeComplete={handleColorChange}
                />
            )}
            <Button className="mt-3" variant="success" onClick={addColorToQueue}>
                Add Color to Queue
            </Button>
            <ListGroup className="mt-3">
                {colors.map((color, index) => (
                    <ListGroupItem key={index} style={{ backgroundColor: color }}>
                        <Row>
                            <Col>{color}</Col>
                            <Col xs="auto">
                                <Button
                                    variant="danger"
                                    size="sm"
                                    onClick={() => deleteColorFromQueue(index)}
                                >
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