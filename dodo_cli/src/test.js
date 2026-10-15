extensionsRuntimeBridge = {}
extensionsRuntimeBridge.runEventLoopTick = (ev) => {
    extensionsRuntime.postMessage("Event loop spinning. " + JSON.stringify(ev));
};
