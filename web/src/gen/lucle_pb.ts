// @generated by protoc-gen-es v1.0.0 with parameter "target=ts"
// @generated from file lucle.proto (package luclerpc, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";

/**
 * @generated from enum luclerpc.DatabaseType
 */
export enum DatabaseType {
  /**
   * @generated from enum value: MYSQL = 0;
   */
  MYSQL = 0,

  /**
   * @generated from enum value: POSTGRESQL = 1;
   */
  POSTGRESQL = 1,

  /**
   * @generated from enum value: SQLITE = 2;
   */
  SQLITE = 2,
}
// Retrieve enum metadata with: proto3.getEnumType(DatabaseType)
proto3.util.setEnumType(DatabaseType, "luclerpc.DatabaseType", [
  { no: 0, name: "MYSQL" },
  { no: 1, name: "POSTGRESQL" },
  { no: 2, name: "SQLITE" },
]);

/**
 * @generated from message luclerpc.RepositoryPath
 */
export class RepositoryPath extends Message<RepositoryPath> {
  /**
   * @generated from field: string path = 1;
   */
  path = "";

  constructor(data?: PartialMessage<RepositoryPath>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.RepositoryPath";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "path", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): RepositoryPath {
    return new RepositoryPath().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): RepositoryPath {
    return new RepositoryPath().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): RepositoryPath {
    return new RepositoryPath().fromJsonString(jsonString, options);
  }

  static equals(a: RepositoryPath | PlainMessage<RepositoryPath> | undefined, b: RepositoryPath | PlainMessage<RepositoryPath> | undefined): boolean {
    return proto3.util.equals(RepositoryPath, a, b);
  }
}

/**
 * @generated from message luclerpc.ResponseResult
 */
export class ResponseResult extends Message<ResponseResult> {
  /**
   * @generated from field: string error = 1;
   */
  error = "";

  constructor(data?: PartialMessage<ResponseResult>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.ResponseResult";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "error", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ResponseResult {
    return new ResponseResult().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ResponseResult {
    return new ResponseResult().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ResponseResult {
    return new ResponseResult().fromJsonString(jsonString, options);
  }

  static equals(a: ResponseResult | PlainMessage<ResponseResult> | undefined, b: ResponseResult | PlainMessage<ResponseResult> | undefined): boolean {
    return proto3.util.equals(ResponseResult, a, b);
  }
}

/**
 * @generated from message luclerpc.StatusResult
 */
export class StatusResult extends Message<StatusResult> {
  /**
   * @generated from field: bool repoinit = 1;
   */
  repoinit = false;

  /**
   * @generated from field: string current_version = 2;
   */
  currentVersion = "";

  /**
   * @generated from field: repeated string versions = 3;
   */
  versions: string[] = [];

  /**
   * @generated from field: repeated string packages = 4;
   */
  packages: string[] = [];

  constructor(data?: PartialMessage<StatusResult>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.StatusResult";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "repoinit", kind: "scalar", T: 8 /* ScalarType.BOOL */ },
    { no: 2, name: "current_version", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "versions", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
    { no: 4, name: "packages", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): StatusResult {
    return new StatusResult().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): StatusResult {
    return new StatusResult().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): StatusResult {
    return new StatusResult().fromJsonString(jsonString, options);
  }

  static equals(a: StatusResult | PlainMessage<StatusResult> | undefined, b: StatusResult | PlainMessage<StatusResult> | undefined): boolean {
    return proto3.util.equals(StatusResult, a, b);
  }
}

/**
 * @generated from message luclerpc.Version
 */
export class Version extends Message<Version> {
  /**
   * @generated from field: string path = 1;
   */
  path = "";

  /**
   * @generated from field: string version = 2;
   */
  version = "";

  constructor(data?: PartialMessage<Version>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.Version";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "path", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "version", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Version {
    return new Version().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Version {
    return new Version().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Version {
    return new Version().fromJsonString(jsonString, options);
  }

  static equals(a: Version | PlainMessage<Version> | undefined, b: Version | PlainMessage<Version> | undefined): boolean {
    return proto3.util.equals(Version, a, b);
  }
}

/**
 * @generated from message luclerpc.Package
 */
export class Package extends Message<Package> {
  /**
   * @generated from field: string path = 1;
   */
  path = "";

  /**
   * @generated from field: string name = 2;
   */
  name = "";

  constructor(data?: PartialMessage<Package>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.Package";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "path", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Package {
    return new Package().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Package {
    return new Package().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Package {
    return new Package().fromJsonString(jsonString, options);
  }

  static equals(a: Package | PlainMessage<Package> | undefined, b: Package | PlainMessage<Package> | undefined): boolean {
    return proto3.util.equals(Package, a, b);
  }
}

/**
 * @generated from message luclerpc.Database
 */
export class Database extends Message<Database> {
  /**
   * @generated from field: luclerpc.DatabaseType db_type = 1;
   */
  dbType = DatabaseType.MYSQL;

  /**
   * @generated from field: optional string migration_path = 2;
   */
  migrationPath?: string;

  constructor(data?: PartialMessage<Database>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.Database";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "db_type", kind: "enum", T: proto3.getEnumType(DatabaseType) },
    { no: 2, name: "migration_path", kind: "scalar", T: 9 /* ScalarType.STRING */, opt: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Database {
    return new Database().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Database {
    return new Database().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Database {
    return new Database().fromJsonString(jsonString, options);
  }

  static equals(a: Database | PlainMessage<Database> | undefined, b: Database | PlainMessage<Database> | undefined): boolean {
    return proto3.util.equals(Database, a, b);
  }
}

/**
 * @generated from message luclerpc.Empty
 */
export class Empty extends Message<Empty> {
  constructor(data?: PartialMessage<Empty>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime = proto3;
  static readonly typeName = "luclerpc.Empty";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Empty {
    return new Empty().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Empty {
    return new Empty().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Empty {
    return new Empty().fromJsonString(jsonString, options);
  }

  static equals(a: Empty | PlainMessage<Empty> | undefined, b: Empty | PlainMessage<Empty> | undefined): boolean {
    return proto3.util.equals(Empty, a, b);
  }
}

