import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface VestingInfoFields {
  rate: BN
  nextTimestamp: BN
}

export interface VestingInfoJSON {
  rate: string
  nextTimestamp: string
}

export class VestingInfo {
  readonly rate: BN
  readonly nextTimestamp: BN

  constructor(fields: VestingInfoFields) {
    this.rate = fields.rate
    this.nextTimestamp = fields.nextTimestamp
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.u64("rate"), borsh.u64("nextTimestamp")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new VestingInfo({
      rate: obj.rate,
      nextTimestamp: obj.nextTimestamp,
    })
  }

  static toEncodable(fields: VestingInfoFields) {
    return {
      rate: fields.rate,
      nextTimestamp: fields.nextTimestamp,
    }
  }

  toJSON(): VestingInfoJSON {
    return {
      rate: this.rate.toString(),
      nextTimestamp: this.nextTimestamp.toString(),
    }
  }

  static fromJSON(obj: VestingInfoJSON): VestingInfo {
    return new VestingInfo({
      rate: new BN(obj.rate),
      nextTimestamp: new BN(obj.nextTimestamp),
    })
  }

  toEncodable() {
    return VestingInfo.toEncodable(this)
  }
}
