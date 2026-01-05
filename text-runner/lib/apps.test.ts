import { deepStrictEqual } from "node:assert/strict";
import { parseLine, App } from "../apps.ts";
import { suite, test } from "node:test"

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
