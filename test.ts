import { readLines } from "https://deno.land/std@0.164.0/io/buffer.ts";
import chalk from "npm:chalk";
let md = await Deno.readTextFile("README.md").then((md) =>
  md.split("## Websockets").at(-1)!.split("## Cosmetics").at(0)!.trim().split(
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
      test: desc?.includes("TEST_MODE"),
      request: request ? JSON.parse(request) : undefined,
      response: response ? JSON.parse(response) : undefined,
    };
  })
)!;

const testcosmetics = {
  "cosmetics": [
    {
      "data": "&a",
      "description": "Prefix: invis_test1",
      "id": 0,
      "name": "invis_test1",
      "required_flags": 2,
      "type": 1,
    },
    {
      "data": "§e",
      "description": "Prefix: supporter2",
      "id": 1,
      "name": "supporter2",
      "required_flags": 32,
      "type": 1,
    },
  ],
  "users": {
    "41a9b6aa-168a-4be8-8df8-cac17daf7384": {
      "flags": 32,
    },
  },
};
await Deno.writeTextFile("cosmetics.json", JSON.stringify(testcosmetics));
await fetch("http://localhost:3000/cosmetics", {
  method: "POST",
  headers: {
    "Authorization": `Bearer ${Deno.env.get("BROADCAST_SECRET")}`,
  },
});

for (let i = 0; i < md.length; i++) {
  console.log(`${chalk.yellow("~")} ${i} ${md[i].name}`);
}

const socket = new WebSocket("ws://localhost:3000/ws");

let requests: any = {
  reqName: "Connecting",
  inreq: false,
};

const testmode = Deno.env.get("TESTMODE") != null;

async function doReq(data: any, name: string) {
  console.log(`${chalk.blue(">")} ${
    Deno.inspect(data, {
      colors: true,
    }).replaceAll("\n", " ").replaceAll("   ", " ").replaceAll("  ", " ")
  }`);
  socket.send(JSON.stringify(data));
  requests.reqName = name;
  return new Promise((res, rej) => {
    requests.res = res;
    requests.rej = rej;
    requests.inreq = true;
  });
}

let printCommandMessage = () => {
  Deno.stdout.write(
    new TextEncoder().encode(
      `${chalk.gray("::")} Which command do you want to send next? `,
    ),
  );
};

socket.onopen = async () => {
  console.log(`${chalk.green("!!")} Connection opened`);

  await doReq(md[0].request, md[0].name);
  if (testmode) {
    for (let i = 1; i < md.length; i++) {
      if (md[i].test) {
        await doReq(md[i].request, md[i].name);
      }
    }
    console.log(`${chalk.green("!!")} All tests passed!`);
    Deno.exit(0);
  } else {
    printCommandMessage();
    for await (const line of readLines(Deno.stdin)) {
      let cmd = parseInt(line ?? "0");
      let item = md[cmd];
      if (item.request) {
        await doReq(item.request, item.name);
      } else if (item) {
        console.log(`${chalk.red("~")} ${item.name} has no request`);
      }
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
  for (let item of md) {
    if (data.t == item.response?.t) {
      if (
        (deepSameKeys(data, item.response) ||
          ((data.c == null || data.c == undefined) &&
            (item.response.c == undefined || item.response.c == null))) &&
        item.name == requests.reqName
      ) {
        console.log(`${chalk.green("~")} ${item.name}`);
        requests.res();
        return;
      }
    }
  }
  if (testmode) {
    requests.rej("Unknown response!");
  } else {
    console.log(`${chalk.red("~")} Unknown response`);
    requests.res();
  }
};

socket.onclose = () => {
  console.log(`\n${chalk.red("!!")} Connection closed`);
  Deno.exit(0);
};

/* https://stackoverflow.com/questions/41802259/javascript-deep-check-objects-have-same-keys */
const deepSameKeys = (o1: any, o2: any) => {
  if (o1 === null && o2 === null) {
    return true;
  }
  const o1keys: Set<any> = o1 === null ? new Set() : new Set(Object.keys(o1));
  const o2keys: Set<any> = o2 === null ? new Set() : new Set(Object.keys(o2));
  if (o1keys.size !== o2keys.size) {
    return false;
  }
  for (const key of o1keys) {
    if (!o2keys.has(key)) {
      return false;
    }
    const v1 = o1[key];
    const v2 = o2[key];
    const t1 = typeof v1;
    const t2 = typeof v2;
    if (t1 === "object") {
      if (t2 === "object" && !deepSameKeys(v1, v2)) {
        return false;
      }
    } else if (t2 === "object") {
      return false;
    }
  }
  return true;
};
