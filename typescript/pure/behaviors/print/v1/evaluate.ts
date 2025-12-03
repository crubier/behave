import { PrintArgument, PrintResult } from "./types";
import { Metrics } from "../../../commons/metrics"
import type { Evaluator } from "../../../core/evaluate"

export class PrintEvaluator implements Evaluator<PrintArgument, PrintResult, Metrics> {
    async validate(argument: PrintArgument): Promise<{}> {
        return {}
    }

    async estimate(argument: PrintArgument): Promise<Metrics> {
        return {
            duration: 0.00000001
        }
    }

    async expand(argument: PrintArgument): Promise<PrintArgument> {
        return { text: argument.text }
    }
}
