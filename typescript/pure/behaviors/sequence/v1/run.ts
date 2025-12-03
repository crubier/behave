
import { SequenceArgument, SequenceInput, SequenceResult, SequenceStatus, SequenceState, SequenceOutput } from "./types";
import type { Runner } from "../../../core/run"
import { StatusEnum } from "../../../commons/types";

export class SequenceRunner implements Runner<SequenceArgument<any>, SequenceResult, SequenceStatus<any>, SequenceState, SequenceInput, SequenceOutput> {

    async run(argument: SequenceArgument<any>): Promise<SequenceResult> {
        return {}
    }

    async suspend(): Promise<SequenceState> {
        return {}
    }

    async resume(argument: SequenceArgument<any>, state: SequenceState): Promise<SequenceResult> {
        return {}
    }

    async cancel(): Promise<SequenceStatus<any>> {
        return {
            status: { status: StatusEnum.STATUS_CANCELLED }
        }
    }

    async receive(input: SequenceInput): Promise<SequenceOutput> { return {} }
} 