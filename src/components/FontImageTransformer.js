import {Component, useState} from "react";
import init, {font_info} from './pkg/ASCII_Renderer.js'

class FontImageTransformer extends Component {
    constructor(props) {
        super(props);
        this.state = {
            fontFile: null,
            fontBackgroundColours: [],
            fontForegroundColours: [],
            fontScale: {"x": 12.0, "y": 12.0},
            inputImageFile: null,
            outputImage: null,
            status: 'stopped',
            progress: 0.0,
            eta: 0.0,
            error: ''
        };
    }

    start() {

    }


    render() {
        return (<>

        </>)
    }


}