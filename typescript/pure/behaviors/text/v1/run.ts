
import { TextArgument, TextInput, TextResult, TextStatus, TextState, TextOutput } from "./types";
import type { Runner } from "../../../core/run"

export class TextRunner implements Runner<TextArgument, TextResult, TextStatus, TextState, TextInput, TextOutput> {

    async run(argument: TextArgument): Promise<TextResult> {
        return {
            text: "Hello"
        }
    }

    async suspend(): Promise<TextState> {
        return {}
    }

    async resume(argument: TextArgument, state: TextState): Promise<TextResult> {
        return {
            text: "Hello"
        }
    }

    async cancel(): Promise<TextStatus> {
        return {
            status: "Cancelled"
        }
    }

    async receive(input: TextInput): Promise<TextOutput> { return {} }
} 