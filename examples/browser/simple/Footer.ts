import { subtract } from "./Utils";

export function Footer() {
  const el = document.createElement("div");

  el.innerText = `Goodbye World ${subtract(1, 2)}`;

  return el;
}