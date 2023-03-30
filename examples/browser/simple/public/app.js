
export function Footer() {
  const el = document.createElement("div");

  el.innerText = `Goodbye World ${subtract(1, 2)} ${value}`;

  return el;
}export function sub(a, b) {
  return a - b;
}
export const value = "123";

export function add(a, b) {
  return a + b;
}

export function subtract(a, b) {
  return sub(a, b)
}
export function Header() {
  const div = document.createElement("div");

  div.innerText = `Hello World ${subtract(5, 3)} ${value}`;

  return div;
}
const root = document.getElementById("root");

root.append(Header());
root.append(Footer());
