import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface NoneJSON {
  kind: "None"
}

export class None {
  static readonly discriminator = 0
  static readonly kind = "None"
  readonly discriminator = 0
  readonly kind = "None"

  toJSON(): NoneJSON {
    return {
      kind: "None",
    }
  }

  toEncodable() {
    return {
      None: {},
    }
  }
}

export interface ConstantJSON {
  kind: "Constant"
}

export class Constant {
  static readonly discriminator = 1
  static readonly kind = "Constant"
  readonly discriminator = 1
  readonly kind = "Constant"

  toJSON(): ConstantJSON {
    return {
      kind: "Constant",
    }
  }

  toEncodable() {
    return {
      Constant: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.LockupKindKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("None" in obj) {
    return new None()
  }
  if ("Constant" in obj) {
    return new Constant()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(obj: types.LockupKindJSON): types.LockupKindKind {
  switch (obj.kind) {
    case "None": {
      return new None()
    }
    case "Constant": {
      return new Constant()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "None"),
    borsh.struct([], "Constant"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
