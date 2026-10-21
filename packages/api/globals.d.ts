declare namespace extensionsRuntimeBridge {
	function runEventLoopTick(tick: EventLoopUpdateParameters): void
}
declare namespace extensionsRuntime {
	function postMessage(message: string): void
}

type EventLoopUpdateParameters = {
	timestamp: number
}
