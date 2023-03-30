import { subtract, value } from "./Utils";

export function Header() {
  const div = document.createElement("div");

  div.innerText = `Hello World ${subtract(5, 3)} ${value}`;

  return div;
}