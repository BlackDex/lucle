// @generated by protoc-gen-connect-es v0.11.0 with parameter "target=ts"
// @generated from file lucle.proto (package luclerpc, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { Database, Empty, Message, Package, RepositoryPath, ResponseResult, StatusResult, Version } from "./lucle_pb.js";
import { MethodKind } from "@bufbuild/protobuf";

/**
 * @generated from service luclerpc.Lucle
 */
export const Lucle = {
  typeName: "luclerpc.Lucle",
  methods: {
    /**
     * @generated from rpc luclerpc.Lucle.install
     */
    install: {
      name: "install",
      I: Database,
      O: Empty,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc luclerpc.Lucle.ServerStreamingEcho
     */
    serverStreamingEcho: {
      name: "ServerStreamingEcho",
      I: Empty,
      O: Message,
      kind: MethodKind.ServerStreaming,
    },
  }
} as const;

/**
 * @generated from service luclerpc.Repo
 */
export const Repo = {
  typeName: "luclerpc.Repo",
  methods: {
    /**
     * @generated from rpc luclerpc.Repo.init
     */
    init: {
      name: "init",
      I: RepositoryPath,
      O: ResponseResult,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc luclerpc.Repo.status
     */
    status: {
      name: "status",
      I: RepositoryPath,
      O: StatusResult,
      kind: MethodKind.ServerStreaming,
    },
    /**
     * @generated from rpc luclerpc.Repo.set_current_version
     */
    set_current_version: {
      name: "set_current_version",
      I: Version,
      O: ResponseResult,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc luclerpc.Repo.register_version
     */
    register_version: {
      name: "register_version",
      I: Version,
      O: ResponseResult,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc luclerpc.Repo.unregister_version
     */
    unregister_version: {
      name: "unregister_version",
      I: Version,
      O: ResponseResult,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc luclerpc.Repo.register_package
     */
    register_package: {
      name: "register_package",
      I: Package,
      O: ResponseResult,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc luclerpc.Repo.unregister_package
     */
    unregister_package: {
      name: "unregister_package",
      I: Package,
      O: ResponseResult,
      kind: MethodKind.Unary,
    },
  }
} as const;

