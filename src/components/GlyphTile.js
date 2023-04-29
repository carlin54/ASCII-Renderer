import React from 'react';
import './GlyphTile.css';

function GlyphTile({ imageData, bgColor, fgColor }) {
    const style = {
        backgroundColor: bgColor,
        color: fgColor,
    };

    return (
        <div className="glyph-tile" style={style}>
            <img src={imageData} alt="glyph" />
        </div>
    );
}

export default GlyphTile;