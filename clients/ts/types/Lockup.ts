import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface LockupFields {
  startTs: BN
  endTs: BN
  cooldownEndsTs: BN | null
  kind: types.LockupKindKind
  period: types.LockupPeriodKind
  reserved: Array<number>
}

export interface LockupJSON {
  startTs: string
  endTs: string
  cooldownEndsTs: string | null
  kind: types.LockupKindJSON
  period: types.LockupPeriodJSON
  reserved: Array<number>
}

export class Lockup {
  readonly startTs: BN
  readonly endTs: BN
  readonly cooldownEndsTs: BN | null
  readonly kind: types.LockupKindKind
  readonly period: types.LockupPeriodKind
  readonly reserved: Array<number>

  constructor(fields: LockupFields) {
    this.startTs = fields.startTs
    this.endTs = fields.endTs
    this.cooldownEndsTs = fields.cooldownEndsTs
    this.kind = fields.kind
    this.period = fields.period
    this.reserved = fields.reserved
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.i64("startTs"),
        borsh.i64("endTs"),
        borsh.option(borsh.i64(), "cooldownEndsTs"),
        types.LockupKind.layout("kind"),
        types.LockupPeriod.layout("period"),
        borsh.array(borsh.u8(), 5, "reserved"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new Lockup({
      startTs: obj.startTs,
      endTs: obj.endTs,
      cooldownEndsTs: obj.cooldownEndsTs,
      kind: types.LockupKind.fromDecoded(obj.kind),
      period: types.LockupPeriod.fromDecoded(obj.period),
      reserved: obj.reserved,
    })
  }

  static toEncodable(fields: LockupFields) {
    return {
      startTs: fields.startTs,
      endTs: fields.endTs,
      cooldownEndsTs: fields.cooldownEndsTs,
      kind: fields.kind.toEncodable(),
      period: fields.period.toEncodable(),
      reserved: fields.reserved,
    }
  }

  toJSON(): LockupJSON {
    return {
      startTs: this.startTs.toString(),
      endTs: this.endTs.toString(),
      cooldownEndsTs:
        (this.cooldownEndsTs && this.cooldownEndsTs.toString()) || null,
      kind: this.kind.toJSON(),
      period: this.period.toJSON(),
      reserved: this.reserved,
    }
  }

  static fromJSON(obj: LockupJSON): Lockup {
    return new Lockup({
      startTs: new BN(obj.startTs),
      endTs: new BN(obj.endTs),
      cooldownEndsTs:
        (obj.cooldownEndsTs && new BN(obj.cooldownEndsTs)) || null,
      kind: types.LockupKind.fromJSON(obj.kind),
      period: types.LockupPeriod.fromJSON(obj.period),
      reserved: obj.reserved,
    })
  }

  toEncodable() {
    return Lockup.toEncodable(this)
  }
}
