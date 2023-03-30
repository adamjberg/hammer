import { Div } from "../dom/Div";

export function MyButton() {
  const el = Div();

  el.innerText = "button";

  return el;
}