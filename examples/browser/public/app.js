function Header() {
  const div = document.createElement("div");

  div.innerText = "Hello World";

  return div;
}
const root = document.getElementById("root");

root.append(Header());
