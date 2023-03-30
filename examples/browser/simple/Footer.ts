import { subtract, value } from "./Utils";

export function Footer() {
  const el = document.createElement("div");

  el.innerText = `Goodbye World ${subtract(1, 2)} ${value}`;

  return el;
}