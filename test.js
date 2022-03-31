const { onUpdate } = require(".");

try {
  onUpdate((n) => console.log("js:", n));
  console.log("setup");
} catch (e) {
  console.log("error", e);
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function demo() {
  for (let i = 0; i < 5; i++) {
    console.log(`Waiting ${i} seconds...`);
    await sleep(i * 1000);
  }
  console.log("Done");
}

demo();
