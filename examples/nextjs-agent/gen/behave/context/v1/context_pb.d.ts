// @generated by protoc-gen-es v2.2.3 with parameter "json_types=true"
// @generated from file behave/context/v1/context.proto (package behave.context.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file behave/context/v1/context.proto.
 */
export declare const file_behave_context_v1_context: GenFile;

/**
 * Context defines the context in which the agent is operating
 *
 * @generated from message behave.context.v1.Context
 */
export declare type Context = Message<"behave.context.v1.Context"> & {
  /**
   * @generated from field: behave.context.v1.HardwareContext hardware = 1;
   */
  hardware?: HardwareContext;

  /**
   * @generated from field: behave.context.v1.DataContext data = 2;
   */
  data?: DataContext;

  /**
   * @generated from field: behave.context.v1.NetworkContext network = 3;
   */
  network?: NetworkContext;

  /**
   * @generated from field: behave.context.v1.EnvironmentContext environment = 4;
   */
  environment?: EnvironmentContext;
};

/**
 * Context defines the context in which the agent is operating
 *
 * @generated from message behave.context.v1.Context
 */
export declare type ContextJson = {
  /**
   * @generated from field: behave.context.v1.HardwareContext hardware = 1;
   */
  hardware?: HardwareContextJson;

  /**
   * @generated from field: behave.context.v1.DataContext data = 2;
   */
  data?: DataContextJson;

  /**
   * @generated from field: behave.context.v1.NetworkContext network = 3;
   */
  network?: NetworkContextJson;

  /**
   * @generated from field: behave.context.v1.EnvironmentContext environment = 4;
   */
  environment?: EnvironmentContextJson;
};

/**
 * Describes the message behave.context.v1.Context.
 * Use `create(ContextSchema)` to create a new message.
 */
export declare const ContextSchema: GenMessage<Context, ContextJson>;

/**
 * Context can include info about hardware available on the agent
 *
 * @generated from message behave.context.v1.HardwareContext
 */
export declare type HardwareContext = Message<"behave.context.v1.HardwareContext"> & {
  /**
   * @generated from field: bool has_camera = 1;
   */
  hasCamera: boolean;
};

/**
 * Context can include info about hardware available on the agent
 *
 * @generated from message behave.context.v1.HardwareContext
 */
export declare type HardwareContextJson = {
  /**
   * @generated from field: bool has_camera = 1;
   */
  hasCamera?: boolean;
};

/**
 * Describes the message behave.context.v1.HardwareContext.
 * Use `create(HardwareContextSchema)` to create a new message.
 */
export declare const HardwareContextSchema: GenMessage<HardwareContext, HardwareContextJson>;

/**
 * Context can include data that the agent has access to
 *
 * @generated from message behave.context.v1.DataContext
 */
export declare type DataContext = Message<"behave.context.v1.DataContext"> & {
  /**
   * @generated from field: bool has_vps_map = 1;
   */
  hasVpsMap: boolean;
};

/**
 * Context can include data that the agent has access to
 *
 * @generated from message behave.context.v1.DataContext
 */
export declare type DataContextJson = {
  /**
   * @generated from field: bool has_vps_map = 1;
   */
  hasVpsMap?: boolean;
};

/**
 * Describes the message behave.context.v1.DataContext.
 * Use `create(DataContextSchema)` to create a new message.
 */
export declare const DataContextSchema: GenMessage<DataContext, DataContextJson>;

/**
 * Context can include info about the network connection
 *
 * @generated from message behave.context.v1.NetworkContext
 */
export declare type NetworkContext = Message<"behave.context.v1.NetworkContext"> & {
  /**
   * @generated from field: bool is_online = 1;
   */
  isOnline: boolean;
};

/**
 * Context can include info about the network connection
 *
 * @generated from message behave.context.v1.NetworkContext
 */
export declare type NetworkContextJson = {
  /**
   * @generated from field: bool is_online = 1;
   */
  isOnline?: boolean;
};

/**
 * Describes the message behave.context.v1.NetworkContext.
 * Use `create(NetworkContextSchema)` to create a new message.
 */
export declare const NetworkContextSchema: GenMessage<NetworkContext, NetworkContextJson>;

/**
 * Context can include info about the environment
 *
 * @generated from message behave.context.v1.EnvironmentContext
 */
export declare type EnvironmentContext = Message<"behave.context.v1.EnvironmentContext"> & {
  /**
   * @generated from field: string is_raining = 1;
   */
  isRaining: string;
};

/**
 * Context can include info about the environment
 *
 * @generated from message behave.context.v1.EnvironmentContext
 */
export declare type EnvironmentContextJson = {
  /**
   * @generated from field: string is_raining = 1;
   */
  isRaining?: string;
};

/**
 * Describes the message behave.context.v1.EnvironmentContext.
 * Use `create(EnvironmentContextSchema)` to create a new message.
 */
export declare const EnvironmentContextSchema: GenMessage<EnvironmentContext, EnvironmentContextJson>;

