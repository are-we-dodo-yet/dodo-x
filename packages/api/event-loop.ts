export function installEventLoop() {
	globalThis.extensionsRuntimeBridge = {
		...(globalThis.extensionsRuntime ?? {}),
		runEventLoopTick(_ev) {
			// Do nothing
		},
	}
}
