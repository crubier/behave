// @generated by protoc-gen-es v2.2.3 with parameter "json_types=true"
// @generated from file behave/core/behavior/v1/behavior.proto (package behavior.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file behave/core/behavior/v1/behavior.proto.
 */
export declare const file_behave_core_behavior_v1_behavior: GenFile;

/**
 * The request message containing the user's name
 *
 * @generated from message behavior.v1.SayHelloRequest
 */
export declare type SayHelloRequest = Message<"behavior.v1.SayHelloRequest"> & {
  /**
   * @generated from field: string name = 1;
   */
  name: string;
};

/**
 * The request message containing the user's name
 *
 * @generated from message behavior.v1.SayHelloRequest
 */
export declare type SayHelloRequestJson = {
  /**
   * @generated from field: string name = 1;
   */
  name?: string;
};

/**
 * Describes the message behavior.v1.SayHelloRequest.
 * Use `create(SayHelloRequestSchema)` to create a new message.
 */
export declare const SayHelloRequestSchema: GenMessage<SayHelloRequest, SayHelloRequestJson>;

/**
 * The response message containing the greeting
 *
 * @generated from message behavior.v1.SayHelloResponse
 */
export declare type SayHelloResponse = Message<"behavior.v1.SayHelloResponse"> & {
  /**
   * @generated from field: string message = 1;
   */
  message: string;
};

/**
 * The response message containing the greeting
 *
 * @generated from message behavior.v1.SayHelloResponse
 */
export declare type SayHelloResponseJson = {
  /**
   * @generated from field: string message = 1;
   */
  message?: string;
};

/**
 * Describes the message behavior.v1.SayHelloResponse.
 * Use `create(SayHelloResponseSchema)` to create a new message.
 */
export declare const SayHelloResponseSchema: GenMessage<SayHelloResponse, SayHelloResponseJson>;

/**
 * The service definition
 *
 * @generated from service behavior.v1.BehaviorService
 */
export declare const BehaviorService: GenService<{
  /**
   * A simple RPC
   *
   * @generated from rpc behavior.v1.BehaviorService.SayHello
   */
  sayHello: {
    methodKind: "unary";
    input: typeof SayHelloRequestSchema;
    output: typeof SayHelloResponseSchema;
  },
}>;
