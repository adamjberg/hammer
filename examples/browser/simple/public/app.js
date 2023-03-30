export function sub(a, b) {
  return a - b;
}
export function add(a, b) {
  return a + b;
}

export function subtract(a, b) {
  return sub(a, b)
}
export function Footer() {
  const el = document.createElement("div");

  el.innerText = `Goodbye World ${subtract(1, 2)}`;

  return el;
}export function Header() {
  const div = document.createElement("div");

  div.innerText = "Hello World";

  return div;
}
const root = document.getElementById("root");

root.append(Header());
root.append(Footer());
