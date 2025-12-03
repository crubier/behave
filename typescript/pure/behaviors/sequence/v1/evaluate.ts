import { SequenceArgument, SequenceResult } from "./types";
import { Metrics } from "../../../commons/metrics"
import type { Evaluator } from "../../../core/evaluate"

export class SequenceEvaluator implements Evaluator<SequenceArgument<any>, SequenceResult, Metrics> {
    async validate(argument: SequenceArgument<any>): Promise<{}> {
        return {}
    }

    async estimate(argument: SequenceArgument<any>): Promise<Metrics> {
        return {
            duration: 0.00000001
        }
    }

    async expand(argument: SequenceArgument<any>): Promise<SequenceArgument<any>> {
        return { children: [] }
    }
}
