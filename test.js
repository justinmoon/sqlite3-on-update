const { onUpdate } = require(".");

try {
  onUpdate((n) => console.log("js:", n));
} catch (e) {
  console.log("error", e);
}
