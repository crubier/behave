// @generated by protoc-gen-es v2.2.3 with parameter "json_types=true"
// @generated from file behave/actions/print/v1/print.proto (package behave.actions.print.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file behave/actions/print/v1/print.proto.
 */
export declare const file_behave_actions_print_v1_print: GenFile;

/**
 * @generated from message behave.actions.print.v1.PrintArgument
 */
export declare type PrintArgument = Message<"behave.actions.print.v1.PrintArgument"> & {
  /**
   * @generated from field: string message = 1;
   */
  message: string;
};

/**
 * @generated from message behave.actions.print.v1.PrintArgument
 */
export declare type PrintArgumentJson = {
  /**
   * @generated from field: string message = 1;
   */
  message?: string;
};

/**
 * Describes the message behave.actions.print.v1.PrintArgument.
 * Use `create(PrintArgumentSchema)` to create a new message.
 */
export declare const PrintArgumentSchema: GenMessage<PrintArgument, PrintArgumentJson>;

/**
 * @generated from message behave.actions.print.v1.PrintResult
 */
export declare type PrintResult = Message<"behave.actions.print.v1.PrintResult"> & {
};

/**
 * @generated from message behave.actions.print.v1.PrintResult
 */
export declare type PrintResultJson = {
};

/**
 * Describes the message behave.actions.print.v1.PrintResult.
 * Use `create(PrintResultSchema)` to create a new message.
 */
export declare const PrintResultSchema: GenMessage<PrintResult, PrintResultJson>;

/**
 * @generated from message behave.actions.print.v1.PrintState
 */
export declare type PrintState = Message<"behave.actions.print.v1.PrintState"> & {
};

/**
 * @generated from message behave.actions.print.v1.PrintState
 */
export declare type PrintStateJson = {
};

/**
 * Describes the message behave.actions.print.v1.PrintState.
 * Use `create(PrintStateSchema)` to create a new message.
 */
export declare const PrintStateSchema: GenMessage<PrintState, PrintStateJson>;

/**
 * @generated from message behave.actions.print.v1.PrintInput
 */
export declare type PrintInput = Message<"behave.actions.print.v1.PrintInput"> & {
};

/**
 * @generated from message behave.actions.print.v1.PrintInput
 */
export declare type PrintInputJson = {
};

/**
 * Describes the message behave.actions.print.v1.PrintInput.
 * Use `create(PrintInputSchema)` to create a new message.
 */
export declare const PrintInputSchema: GenMessage<PrintInput, PrintInputJson>;

/**
 * @generated from message behave.actions.print.v1.PrintOutput
 */
export declare type PrintOutput = Message<"behave.actions.print.v1.PrintOutput"> & {
  /**
   * @generated from field: string message = 1;
   */
  message: string;
};

/**
 * @generated from message behave.actions.print.v1.PrintOutput
 */
export declare type PrintOutputJson = {
  /**
   * @generated from field: string message = 1;
   */
  message?: string;
};

/**
 * Describes the message behave.actions.print.v1.PrintOutput.
 * Use `create(PrintOutputSchema)` to create a new message.
 */
export declare const PrintOutputSchema: GenMessage<PrintOutput, PrintOutputJson>;

