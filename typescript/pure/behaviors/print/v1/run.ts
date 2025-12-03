
import { PrintArgument, PrintInput, PrintResult, PrintStatus, PrintState, PrintOutput } from "./types";
import type { Runner } from "../../../core/run"

export class PrintRunner implements Runner<PrintArgument, PrintResult, PrintStatus, PrintState, PrintInput, PrintOutput> {

    async run(argument: PrintArgument): Promise<PrintResult> {
        return {}
    }

    async suspend(): Promise<PrintState> {
        return {}
    }

    async resume(argument: PrintArgument, state: PrintState): Promise<PrintResult> {
        return {}
    }

    async cancel(): Promise<PrintStatus> {
        return {
            status: "Cancelled"
        }
    }

    async receive(input: PrintInput): Promise<PrintOutput> { return {} }
} 