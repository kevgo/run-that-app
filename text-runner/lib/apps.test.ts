import { deepStrictEqual } from "node:assert/strict";
import { parseApps, parseLine, App } from "../apps.ts";
import { suite, test } from "node:test"

suite("parseApps", () => {
	test("many apps", () => {
		const give = `
actionlint https://github.com/actionlint/actionlint
alphavet https://github.com/alphavet/alphavet
`
		const have = parseApps(give);
		const want: App[] = [
			{
				name: "actionlint",
				url: "https://github.com/actionlint/actionlint",
			}, {
				name: "alphavet",
				url: "https://github.com/alphavet/alphavet",
			},
		]
		deepStrictEqual(have, want);
	})
})

suite("parseLine", () => {
	test("name and url", () => {
		const give = "actionlint https://github.com/actionlint/actionlint"
		const have = parseLine(give);
		const want: App = {
			name: "actionlint",
			url: "https://github.com/actionlint/actionlint",
		}
		deepStrictEqual(have, want);
	})
})
