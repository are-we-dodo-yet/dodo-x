import { denoPlugin } from "@deno/esbuild-plugin"
import { build, type Location } from "esbuild"
import { assertNotNull } from "@are-we-dodo-yet/api"
import { fatalError } from "./error.ts"
import { basename } from "@std/path"

export async function bundle(entrypoint: string): Promise<Uint8Array> {
	const result = await build({
		plugins: [denoPlugin()],
		bundle: true,
		entryPoints: [entrypoint],
		write: false,
		outfile: "out.js",
		treeShaking: true,
		format: "esm",
	})

	if (result.errors.length > 0) {
		fatalError({
			heading: "I couldn't build your code.",
			description: [
				...result.errors.map((it) =>
					`${
						it.location
							? (printLocation(it.location) + ": ")
							: ""
					} ${it.text}`
				),
			],
		})
	}

	return assertNotNull(
		result.outputFiles.find((it) => basename(it.path) == "out.js"),
	).contents
}

function printLocation(location: Location) {
	return `${location.file}:${location.line}:${location.column}`
}
