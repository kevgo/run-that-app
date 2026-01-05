import { deepEqual } from "node:assert/strict";
import { execSync } from "node:child_process";
import * as textRunner from "text-runner";

export function apps(action: textRunner.actions.Args) {
	action.name("verify installable applications")
	const installableApps = loadInstallableApps();
	const documentedApps = loadDocumentedApps(action.region);
	deepEqual(installableApps, documentedApps);
}

export class App {
	name: string;
	url: string;
}

function loadDocumentedApps(region: textRunner.ast.NodeList): App[] {
	const result = []
	for (const liNode of region.nodesOfTypes("list_item_open")) {
		const appNodes = region.nodesFor(liNode)
		const linkNode = appNodes.nodeOfTypes("link_open")
		result.push({
			name: appNodes.nodesFor(linkNode).text(),
			url: linkNode.attributes["href"],
		})
	}
	return result
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
	return execSync("cargo run -- --apps", { encoding: "utf-8" })
}
