import { readLines } from "https://deno.land/std@0.164.0/io/buffer.ts";
import chalk from "npm:chalk";
let md =
  await Deno.readTextFile("README.md").then((md) =>
    md.split("## Websockets").at(-1)?.split("## Cosmetics").at(0)?.trim().split(
      "\n###",
    ).slice(1).map((x) => {
      let [name, ...test] = x.trim().split("\n");
      let text = test.join("\n").trim();
      let desc = text.split("```json").at(0)?.trim();
      let request = text.split("---").at(0)?.split("```json").at(1)?.split(
        "```",
      )
        .at(0)?.trim();
      let response = text.split("---").at(1)?.split("```json").at(1)?.split(
        "```",
      )
        .at(0)?.trim();
      return {
        name,
        desc,
        request: request ? JSON.parse(request) : undefined,
        response: response ? JSON.parse(response) : undefined,
      };
    })
  ) ?? [];

for (let i = 0; i < md.length; i++) {
  console.log(`${chalk.yellow("~")} ${i} ${md[i].name}`);
}

const socket = new WebSocket("ws://localhost:3000/ws");

let inreq = false;
let reqName = "Connecting";

let printCommandMessage = () => {
  Deno.stdout.write(
    new TextEncoder().encode(
      `${chalk.gray("::")} Which command do you want to send next? `,
    ),
  );
};

socket.onopen = async (event) => {
  console.log("Connected to server");
  let sendJson = (data: any) => {
    console.log(`${chalk.blue(">")} ${
      Deno.inspect(data, {
        colors: true,
      }).replaceAll("\n", " ").replaceAll("   ", " ").replaceAll("  ", " ")
    }`);
    socket.send(JSON.stringify(data));
  };
  sendJson(md[0].request);
  inreq = true;

  for await (const line of readLines(Deno.stdin)) {
    let cmd = parseInt(line ?? "0");
    let item = md[cmd];
    if (item.request) {
      sendJson(item.request);
      inreq = true;
      reqName = item.name;
    } else if (item) {
      console.log(`${chalk.red("~")} ${item.name} has no request`);
      printCommandMessage();
    } else {
      printCommandMessage();
    }
  }
};

socket.onmessage = (event) => {
  let data = JSON.parse(event.data);
  console.log(`${chalk.green("<")} ${
    Deno.inspect(data, {
      colors: true,
    }).replaceAll("\n", " ").replaceAll("   ", " ").replaceAll("  ", " ")
  }`);
  let success = false;
  for (let item of md) {
    if (data.t == item.response?.t) {
      if (
        (deepSameKeys(data, item.response) ||
          ((data.c == null || data.c == undefined) &&
            (item.response.c == undefined || item.response.c == null))) &&
        item.name == reqName
      ) {
        console.log(`${chalk.green("~")} ${item.name}`);
        success = true;
      }
    }
  }
  if (!success) {
    console.log(`${chalk.red("~")} Unknown response`);
  }
  if (inreq) {
    printCommandMessage();
  }
};

socket.onclose = (event) => {
  console.log();
  console.log(`${chalk.red("!!")} Connection closed`);
  Deno.exit(0);
};

/* https://stackoverflow.com/questions/41802259/javascript-deep-check-objects-have-same-keys */
const deepSameKeys = (o1: any, o2: any) => {
  // Both nulls = same
  if (o1 === null && o2 === null) {
    return true;
  }

  // Get the keys of each object
  const o1keys: Set<any> = o1 === null ? new Set() : new Set(Object.keys(o1));
  const o2keys: Set<any> = o2 === null ? new Set() : new Set(Object.keys(o2));
  if (o1keys.size !== o2keys.size) {
    // Different number of own properties = not the same
    return false;
  }

  // Look for differences, recursing as necessary
  for (const key of o1keys) {
    if (!o2keys.has(key)) {
      // Different keys
      return false;
    }

    // Get the values and their types
    const v1 = o1[key];
    const v2 = o2[key];
    const t1 = typeof v1;
    const t2 = typeof v2;
    if (t1 === "object") {
      if (t2 === "object" && !deepSameKeys(v1, v2)) {
        return false;
      }
    } else if (t2 === "object") {
      // We know `v1` isn't an object
      return false;
    }
  }

  // No differences found
  return true;
};
