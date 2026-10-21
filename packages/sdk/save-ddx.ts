import { TarStream, type TarStreamFile, type TarStreamInput } from "@std/tar"
import { resolve } from "@std/path"

export interface DDXFile {
	name: string
	description: string
	id: string
	prelude: Uint8Array
	component: Uint8Array
}

export async function saveDDX(ddxFile: DDXFile, path: string) {
	const entries: TarStreamInput[] = [
		toTarStreamFile(
			"config.json",
			JSON.stringify({
				name: ddxFile.name,
				description: ddxFile.description,
				id: ddxFile.id,
			}),
		),
		toTarStreamFile("prelude.js", ddxFile.prelude),
		toTarStreamFile("component.js", ddxFile.component),
	]

	await Deno.mkdir(resolve(path, ".."), { recursive: true })

	await ReadableStream.from(entries).pipeThrough(new TarStream()).pipeTo(
		(await Deno.create(path)).writable,
	)
}

const textEncoder = new TextEncoder()

type Datable = string | Uint8Array | ArrayBuffer

function toReadable(a: Datable): ReadableStream<Uint8Array<ArrayBufferLike>> {
	return new Blob([toArrayBuffer(a)]).stream()
}

function toArrayBuffer(a: Datable): ArrayBuffer {
	if (typeof a == "string") {
		return textEncoder.encode(a).buffer
	} else if (a instanceof Uint8Array) {
		return new Uint8Array([...a]).buffer
	} else {
		return a
	}
}

function toTarStreamFile(path: string, data: Datable): TarStreamFile {
	const buff = toArrayBuffer(data)

	return {
		type: "file",
		path,
		size: buff.byteLength,
		readable: toReadable(buff),
	}
}
