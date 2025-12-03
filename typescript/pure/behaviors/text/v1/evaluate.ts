import { TextArgument, TextResult } from "./types";
import { Metrics } from "../../../commons/metrics"
import type { Evaluator } from "../../../core/evaluate"

export class TextEvaluator implements Evaluator<TextArgument, TextResult, Metrics> {
    async validate(argument: TextArgument): Promise<{}> {
        return {}
    }

    async estimate(argument: TextArgument): Promise<Metrics> {
        return {
            duration: 0.00000001
        }
    }

    async expand(argument: TextArgument): Promise<TextArgument> {
        return {}
    }
}
