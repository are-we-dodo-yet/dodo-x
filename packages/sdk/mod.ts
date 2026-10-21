import { Command } from "@cliffy/command"
import { loadConfig } from "./config.ts"
import { saveDDX } from "./save-ddx.ts"
import { bundle } from "./bundle.ts"
import { relative, resolve } from "@std/path"

const rootCommand = new Command()
	.name("dodo-sdk")
	.description("The development toolchain for making dodo-x plugins")
	// make
	.command("make", "Turn your plugin directory into a .ddx file")
	.option("-c --config <path:string>", "Configuration file path", {
		default: "extension.toml",
	})
	.option("-o --output-file <path:string>", "Output ddx path", {
		default: "build.ddx",
	})
	.action(async (options, ..._args) => {
		await make(options)
	})

export async function main() {
	await rootCommand.parse()
}

async function make(options: { config: string; outputFile: string }) {
	const config = await loadConfig(options.config)

	const prelude = await bundle(config.prelude)
	const component = await bundle(config.component)

	await saveDDX({
		name: config.name,
		description: config.description,
		prelude,
		component,
		id: makeSafe(`${config.owner_email}___${config.name}`),
	}, options.outputFile)

	console.log(
		"My work here is done! Your output is available at " +
			relative(Deno.cwd(), resolve(options.outputFile)),
	)
}

if (import.meta.main) {
	await main()
}

function makeSafe(str: string): string {
	return str.replaceAll(/[^a-zA-Z]/g, "_")
}
