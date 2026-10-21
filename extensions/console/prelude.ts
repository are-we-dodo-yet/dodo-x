import { prettyPrint } from "./pretty-print.ts"
import { postMessage } from "@are-we-dodo-yet/api"

console.log = (...args) => {
	postMessage(args.map(prettyPrint).join(" "))
}
