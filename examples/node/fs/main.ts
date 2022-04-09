import * as fs from "fs";

try {
  const data = fs.readFileSync('test.txt', 'utf8')
  console.log(data)
} catch (err) {
  console.error(err)
}