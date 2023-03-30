import { MyButton } from "./components/MyButton";
import { Div } from "./dom/Div"
import { Nav } from "./Nav";

const nav = Nav();
document.body.append(nav);

const btn = MyButton();
document.body.append(btn)

const text = Div();
text.innerText = "Hello World";

document.body.append(text);