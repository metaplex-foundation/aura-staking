import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface DepositEntryFields {
  lockup: types.LockupFields
  amountDepositedNative: BN
  amountInitiallyLockedNative: BN
  isUsed: boolean
  votingMintConfigIdx: number
  reserved: Array<number>
}

export interface DepositEntryJSON {
  lockup: types.LockupJSON
  amountDepositedNative: string
  amountInitiallyLockedNative: string
  isUsed: boolean
  votingMintConfigIdx: number
  reserved: Array<number>
}

export class DepositEntry {
  readonly lockup: types.Lockup
  readonly amountDepositedNative: BN
  readonly amountInitiallyLockedNative: BN
  readonly isUsed: boolean
  readonly votingMintConfigIdx: number
  readonly reserved: Array<number>

  constructor(fields: DepositEntryFields) {
    this.lockup = new types.Lockup({ ...fields.lockup })
    this.amountDepositedNative = fields.amountDepositedNative
    this.amountInitiallyLockedNative = fields.amountInitiallyLockedNative
    this.isUsed = fields.isUsed
    this.votingMintConfigIdx = fields.votingMintConfigIdx
    this.reserved = fields.reserved
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        types.Lockup.layout("lockup"),
        borsh.u64("amountDepositedNative"),
        borsh.u64("amountInitiallyLockedNative"),
        borsh.bool("isUsed"),
        borsh.u8("votingMintConfigIdx"),
        borsh.array(borsh.u8(), 6, "reserved"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new DepositEntry({
      lockup: types.Lockup.fromDecoded(obj.lockup),
      amountDepositedNative: obj.amountDepositedNative,
      amountInitiallyLockedNative: obj.amountInitiallyLockedNative,
      isUsed: obj.isUsed,
      votingMintConfigIdx: obj.votingMintConfigIdx,
      reserved: obj.reserved,
    })
  }

  static toEncodable(fields: DepositEntryFields) {
    return {
      lockup: types.Lockup.toEncodable(fields.lockup),
      amountDepositedNative: fields.amountDepositedNative,
      amountInitiallyLockedNative: fields.amountInitiallyLockedNative,
      isUsed: fields.isUsed,
      votingMintConfigIdx: fields.votingMintConfigIdx,
      reserved: fields.reserved,
    }
  }

  toJSON(): DepositEntryJSON {
    return {
      lockup: this.lockup.toJSON(),
      amountDepositedNative: this.amountDepositedNative.toString(),
      amountInitiallyLockedNative: this.amountInitiallyLockedNative.toString(),
      isUsed: this.isUsed,
      votingMintConfigIdx: this.votingMintConfigIdx,
      reserved: this.reserved,
    }
  }

  static fromJSON(obj: DepositEntryJSON): DepositEntry {
    return new DepositEntry({
      lockup: types.Lockup.fromJSON(obj.lockup),
      amountDepositedNative: new BN(obj.amountDepositedNative),
      amountInitiallyLockedNative: new BN(obj.amountInitiallyLockedNative),
      isUsed: obj.isUsed,
      votingMintConfigIdx: obj.votingMintConfigIdx,
      reserved: obj.reserved,
    })
  }

  toEncodable() {
    return DepositEntry.toEncodable(this)
  }
}
