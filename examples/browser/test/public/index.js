
export function MyButton() {
  const el = Div();

  el.innerText = "button";

  return el;
}export function Div() {
  return document.createElement('div');
}
export function Nav() {
  const el = Div()

  el.innerText = "nav";

  return el;
}
const nav = Nav();
document.body.append(nav);

const btn = MyButton();
document.body.append(btn)

const text = Div();
text.innerText = "Hello World";

document.body.append(text);