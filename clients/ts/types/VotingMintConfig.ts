import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface VotingMintConfigFields {
  mint: PublicKey
  grantAuthority: PublicKey
  baselineVoteWeightScaledFactor: BN
  maxExtraLockupVoteWeightScaledFactor: BN
  lockupSaturationSecs: BN
  digitShift: number
  reserved1: Array<number>
  reserved2: Array<BN>
}

export interface VotingMintConfigJSON {
  mint: string
  grantAuthority: string
  baselineVoteWeightScaledFactor: string
  maxExtraLockupVoteWeightScaledFactor: string
  lockupSaturationSecs: string
  digitShift: number
  reserved1: Array<number>
  reserved2: Array<string>
}

export class VotingMintConfig {
  readonly mint: PublicKey
  readonly grantAuthority: PublicKey
  readonly baselineVoteWeightScaledFactor: BN
  readonly maxExtraLockupVoteWeightScaledFactor: BN
  readonly lockupSaturationSecs: BN
  readonly digitShift: number
  readonly reserved1: Array<number>
  readonly reserved2: Array<BN>

  constructor(fields: VotingMintConfigFields) {
    this.mint = fields.mint
    this.grantAuthority = fields.grantAuthority
    this.baselineVoteWeightScaledFactor = fields.baselineVoteWeightScaledFactor
    this.maxExtraLockupVoteWeightScaledFactor =
      fields.maxExtraLockupVoteWeightScaledFactor
    this.lockupSaturationSecs = fields.lockupSaturationSecs
    this.digitShift = fields.digitShift
    this.reserved1 = fields.reserved1
    this.reserved2 = fields.reserved2
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("mint"),
        borsh.publicKey("grantAuthority"),
        borsh.u64("baselineVoteWeightScaledFactor"),
        borsh.u64("maxExtraLockupVoteWeightScaledFactor"),
        borsh.u64("lockupSaturationSecs"),
        borsh.i8("digitShift"),
        borsh.array(borsh.u8(), 7, "reserved1"),
        borsh.array(borsh.u64(), 7, "reserved2"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new VotingMintConfig({
      mint: obj.mint,
      grantAuthority: obj.grantAuthority,
      baselineVoteWeightScaledFactor: obj.baselineVoteWeightScaledFactor,
      maxExtraLockupVoteWeightScaledFactor:
        obj.maxExtraLockupVoteWeightScaledFactor,
      lockupSaturationSecs: obj.lockupSaturationSecs,
      digitShift: obj.digitShift,
      reserved1: obj.reserved1,
      reserved2: obj.reserved2,
    })
  }

  static toEncodable(fields: VotingMintConfigFields) {
    return {
      mint: fields.mint,
      grantAuthority: fields.grantAuthority,
      baselineVoteWeightScaledFactor: fields.baselineVoteWeightScaledFactor,
      maxExtraLockupVoteWeightScaledFactor:
        fields.maxExtraLockupVoteWeightScaledFactor,
      lockupSaturationSecs: fields.lockupSaturationSecs,
      digitShift: fields.digitShift,
      reserved1: fields.reserved1,
      reserved2: fields.reserved2,
    }
  }

  toJSON(): VotingMintConfigJSON {
    return {
      mint: this.mint.toString(),
      grantAuthority: this.grantAuthority.toString(),
      baselineVoteWeightScaledFactor:
        this.baselineVoteWeightScaledFactor.toString(),
      maxExtraLockupVoteWeightScaledFactor:
        this.maxExtraLockupVoteWeightScaledFactor.toString(),
      lockupSaturationSecs: this.lockupSaturationSecs.toString(),
      digitShift: this.digitShift,
      reserved1: this.reserved1,
      reserved2: this.reserved2.map((item) => item.toString()),
    }
  }

  static fromJSON(obj: VotingMintConfigJSON): VotingMintConfig {
    return new VotingMintConfig({
      mint: new PublicKey(obj.mint),
      grantAuthority: new PublicKey(obj.grantAuthority),
      baselineVoteWeightScaledFactor: new BN(
        obj.baselineVoteWeightScaledFactor
      ),
      maxExtraLockupVoteWeightScaledFactor: new BN(
        obj.maxExtraLockupVoteWeightScaledFactor
      ),
      lockupSaturationSecs: new BN(obj.lockupSaturationSecs),
      digitShift: obj.digitShift,
      reserved1: obj.reserved1,
      reserved2: obj.reserved2.map((item) => new BN(item)),
    })
  }

  toEncodable() {
    return VotingMintConfig.toEncodable(this)
  }
}
