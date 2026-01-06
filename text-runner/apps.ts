import { deepEqual } from "node:assert/strict";
import { execSync } from "node:child_process";
import * as textRunner from "text-runner";

export function apps(action: textRunner.actions.Args) {
  action.name("verify installable applications");
  const installableApps = loadInstallableApps();
  const documentedApps = loadDocumentedApps(action.region);
  deepEqual(installableApps, documentedApps);
}

export class App {
  name: string;
  url: string;
}

function loadDocumentedApps(region: textRunner.ast.NodeList): App[] {
  const result = [];
  for (const listItemOpener of region.nodesOfTypes("list_item_open")) {
    const listItemNodes = region.nodesFor(listItemOpener);
    const linkOpener = listItemNodes.nodeOfTypes("link_open");
    const linkNodes = listItemNodes.nodesFor(linkOpener);
    result.push({
      name: linkNodes.text(),
      url: linkOpener.attributes["href"],
    });
  }
  return result;
}

function loadInstallableApps(): App[] {
  return parseApps(queryApps());
}

export function parseApps(output: string): App[] {
  return output.trim().split("\n").map(parseLine);
}

export function parseLine(line: string): App {
  const [name, url] = line.split(" ");
  return { name, url };
}

function queryApps(): string {
  return execSync("cargo run -- --apps", { encoding: "utf-8" });
}
