(() => {
  // src/dom/Div.js
  function Div() {
    return document.createElement("div");
  }

  // src/components/MyButton.js
  function MyButton() {
    const el = Div();
    el.innerText = "button";
    return el;
  }

  // src/Nav.js
  function Nav() {
    const el = Div();
    el.innerText = "nav";
    return el;
  }

  // src/index.js
  var nav = Nav();
  document.body.append(nav);
  var btn = MyButton();
  document.body.append(btn);
  var text = Div();
  text.innerText = "Hello World";
  document.body.append(text);
})();
