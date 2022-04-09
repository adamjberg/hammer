function Header() {
  const div = document.createElement("div");

  div.innerText = "Hello World";

  return div;
}function Footer() {
  const el = document.createElement("div");

  el.innerText = "Goodbye World";

  return el;
}
const root = document.getElementById("root");

root.append(Header());
root.append(Footer());
