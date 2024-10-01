import { dotfile } from "@randomairborne/rspkg";
import { instance } from "@viz-js/viz";

const btn = document.getElementById("sb-x");
const df = document.getElementById("ta-x");

btn.addEventListener("click", () => {
  const dot = dotfile(df.value);
  instance().then((viz) => {
    document.body.appendChild(viz.renderSVGElement(dot));
  });
});
