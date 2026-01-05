import { deepEqual } from "node:assert/strict";
import { execSync } from "node:child_process";
import * as textRunner from "text-runner";

export function apps(action: textRunner.actions.Args) {
	const supportedApps = loadSupportedApps();
	const documentedApps = loadDocumentedApps(action.region);
	deepEqual(supportedApps, documentedApps, "options section");
}

export class App {
	name: string;
	url: string;
}

function loadDocumentedApps(region: textRunner.ast.NodeList) {
	const result = []
	for (const liNode of region.nodesOfTypes("list_item_open")) {
		const appNodes = region.nodesFor(liNode)
		const linkNode = appNodes.nodeOfTypes("link_open")
		const url = linkNode.attributes["href"]
		const linkNodes = region.nodesFor(linkNode)
		const name = linkNodes.text()
		result.push({ name, url })
	}
	return result
}

function loadSupportedApps() {
	return parseApps(queryApps());
}

export function parseApps(output: string): App[] {
	return output.split("\n").map(parseLine);
}

export function parseLine(line: string): App {
	const [name, url] = line.split(" ");
	return { name, url };
}

function queryApps(): string {
	return execSync("cargo run -- --apps", { encoding: "utf-8" }).trim();
}
