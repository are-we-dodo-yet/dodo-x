import * as v from "@valibot/valibot"
import { resolve } from "@std/path"
import { fatalError } from "./error.ts"
import * as TOML from "@std/toml"

export const ConfigSchema = v.object({
	name: v.string("Names must be a reasonable title of your plugin"),
	description: v.string(
		"Descriptions must be a brief description of what your plugin does.",
	),

	prelude: v.string(
		"Prelude must be a path to the script to be prepended to user-code at runtime.",
	),
	component: v.string(
		"Component must be a path to the script which will run in a specially-priviledged realm outside of user-code.",
	),
	owner_email: v.pipe(
		v.string("Owner email must be a string"),
		v.email(
			"Owner email must be a valid email address (in a common format)",
		),
	),
})

export async function loadConfig(
	path: string,
): Promise<v.InferOutput<typeof ConfigSchema>> {
	let text

	try {
		text = await Deno.readTextFile(path)
	} catch {
		fatalError({
			heading:
				"I could not load your configuration file, as the file does not exist.",
			description: [
				"You told me to look for " + path,
				`When resolved, that path is ${resolve(path)}`,
			],
		})
	}

	let content

	try {
		content = TOML.parse(text)
	} catch (error) {
		fatalError({
			heading:
				"I could not load your configuration file, as it is not valid TOML",
			description: (error as Error).toString().split("\n"),
		})
	}

	const parsed = v.safeParse(ConfigSchema, content)

	if (parsed.issues) {
		fatalError({
			heading:
				"I could not load your configuration file, because it did not look the way I expected",
			description: v.summarize(parsed.issues).split("\n"),
		})
	}

	return parsed.output
}
