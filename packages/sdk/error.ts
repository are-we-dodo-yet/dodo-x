import { styleDescription, styleError } from "./styles.ts"

export function fatalError(
	params: { heading: string; description: string[] },
): never {
	console.log()
	console.log(styleError("  " + params.heading))
	console.log()
	for (const line of params.description) {
		console.log(`     ${styleDescription(line)}`)
	}
	console.log()
	Deno.exit(1)
}
