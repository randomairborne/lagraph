import {dotfile} from "@randomairborne/rspkg";
import {instance} from "@viz-js/viz";

const df = document.getElementById("ta-x");
const svg = document.getElementById("svg-p");

function render() {
    const dot = dotfile(df.value);
    instance().then((viz) => {
        while (svg.hasChildNodes()) {
            svg.removeChild(svg.lastChild);
        }
        const new_svg = viz.renderSVGElement(dot);
        svg.appendChild(new_svg);
    });
}

df.addEventListener("input", render);
render();
