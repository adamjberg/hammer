export function Text(text: string) {
  const el = document.createElement("div");

  el.innerText = text;
  
  return el;
}