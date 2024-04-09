import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface LockingInfoFields {
  amount: BN
  endTimestamp: BN | null
  vesting: types.VestingInfoFields | null
}

export interface LockingInfoJSON {
  amount: string
  endTimestamp: string | null
  vesting: types.VestingInfoJSON | null
}

export class LockingInfo {
  readonly amount: BN
  readonly endTimestamp: BN | null
  readonly vesting: types.VestingInfo | null

  constructor(fields: LockingInfoFields) {
    this.amount = fields.amount
    this.endTimestamp = fields.endTimestamp
    this.vesting =
      (fields.vesting && new types.VestingInfo({ ...fields.vesting })) || null
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("amount"),
        borsh.option(borsh.u64(), "endTimestamp"),
        borsh.option(types.VestingInfo.layout(), "vesting"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new LockingInfo({
      amount: obj.amount,
      endTimestamp: obj.endTimestamp,
      vesting:
        (obj.vesting && types.VestingInfo.fromDecoded(obj.vesting)) || null,
    })
  }

  static toEncodable(fields: LockingInfoFields) {
    return {
      amount: fields.amount,
      endTimestamp: fields.endTimestamp,
      vesting:
        (fields.vesting && types.VestingInfo.toEncodable(fields.vesting)) ||
        null,
    }
  }

  toJSON(): LockingInfoJSON {
    return {
      amount: this.amount.toString(),
      endTimestamp: (this.endTimestamp && this.endTimestamp.toString()) || null,
      vesting: (this.vesting && this.vesting.toJSON()) || null,
    }
  }

  static fromJSON(obj: LockingInfoJSON): LockingInfo {
    return new LockingInfo({
      amount: new BN(obj.amount),
      endTimestamp: (obj.endTimestamp && new BN(obj.endTimestamp)) || null,
      vesting: (obj.vesting && types.VestingInfo.fromJSON(obj.vesting)) || null,
    })
  }

  toEncodable() {
    return LockingInfo.toEncodable(this)
  }
}
