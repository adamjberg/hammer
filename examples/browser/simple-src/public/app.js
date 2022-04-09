function Header() {
  const div = document.createElement("div");

  div.innerText = "Hello World";

  return div;
}function Text(text) {
  const el = document.createElement("div");

  el.innerText = text;
  
  return el;
}
function Footer() {
  return Text("Goodbye World");
}
const root = document.getElementById("root");

root.append(Header());
root.append(Footer());
