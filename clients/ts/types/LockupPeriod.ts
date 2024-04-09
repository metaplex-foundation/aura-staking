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

export interface ThreeMonthsJSON {
  kind: "ThreeMonths"
}

export class ThreeMonths {
  static readonly discriminator = 1
  static readonly kind = "ThreeMonths"
  readonly discriminator = 1
  readonly kind = "ThreeMonths"

  toJSON(): ThreeMonthsJSON {
    return {
      kind: "ThreeMonths",
    }
  }

  toEncodable() {
    return {
      ThreeMonths: {},
    }
  }
}

export interface SixMonthsJSON {
  kind: "SixMonths"
}

export class SixMonths {
  static readonly discriminator = 2
  static readonly kind = "SixMonths"
  readonly discriminator = 2
  readonly kind = "SixMonths"

  toJSON(): SixMonthsJSON {
    return {
      kind: "SixMonths",
    }
  }

  toEncodable() {
    return {
      SixMonths: {},
    }
  }
}

export interface OneYearJSON {
  kind: "OneYear"
}

export class OneYear {
  static readonly discriminator = 3
  static readonly kind = "OneYear"
  readonly discriminator = 3
  readonly kind = "OneYear"

  toJSON(): OneYearJSON {
    return {
      kind: "OneYear",
    }
  }

  toEncodable() {
    return {
      OneYear: {},
    }
  }
}

export interface FlexJSON {
  kind: "Flex"
}

export class Flex {
  static readonly discriminator = 4
  static readonly kind = "Flex"
  readonly discriminator = 4
  readonly kind = "Flex"

  toJSON(): FlexJSON {
    return {
      kind: "Flex",
    }
  }

  toEncodable() {
    return {
      Flex: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.LockupPeriodKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("None" in obj) {
    return new None()
  }
  if ("ThreeMonths" in obj) {
    return new ThreeMonths()
  }
  if ("SixMonths" in obj) {
    return new SixMonths()
  }
  if ("OneYear" in obj) {
    return new OneYear()
  }
  if ("Flex" in obj) {
    return new Flex()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(obj: types.LockupPeriodJSON): types.LockupPeriodKind {
  switch (obj.kind) {
    case "None": {
      return new None()
    }
    case "ThreeMonths": {
      return new ThreeMonths()
    }
    case "SixMonths": {
      return new SixMonths()
    }
    case "OneYear": {
      return new OneYear()
    }
    case "Flex": {
      return new Flex()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "None"),
    borsh.struct([], "ThreeMonths"),
    borsh.struct([], "SixMonths"),
    borsh.struct([], "OneYear"),
    borsh.struct([], "Flex"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
