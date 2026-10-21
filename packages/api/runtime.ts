export function postMessage(data: string) {
	globalThis.extensionsRuntime.postMessage(data)
}
