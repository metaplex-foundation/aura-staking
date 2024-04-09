import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface VoterFields {
  voterAuthority: PublicKey
  registrar: PublicKey
  deposits: Array<types.DepositEntryFields>
  voterBump: number
  voterWeightRecordBump: number
  reserved: Array<number>
}

export interface VoterJSON {
  voterAuthority: string
  registrar: string
  deposits: Array<types.DepositEntryJSON>
  voterBump: number
  voterWeightRecordBump: number
  reserved: Array<number>
}

export class Voter {
  readonly voterAuthority: PublicKey
  readonly registrar: PublicKey
  readonly deposits: Array<types.DepositEntry>
  readonly voterBump: number
  readonly voterWeightRecordBump: number
  readonly reserved: Array<number>

  static readonly discriminator = Buffer.from([
    241, 93, 35, 191, 254, 147, 17, 202,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("voterAuthority"),
    borsh.publicKey("registrar"),
    borsh.array(types.DepositEntry.layout(), 32, "deposits"),
    borsh.u8("voterBump"),
    borsh.u8("voterWeightRecordBump"),
    borsh.array(borsh.u8(), 6, "reserved"),
  ])

  constructor(fields: VoterFields) {
    this.voterAuthority = fields.voterAuthority
    this.registrar = fields.registrar
    this.deposits = fields.deposits.map(
      (item) => new types.DepositEntry({ ...item })
    )
    this.voterBump = fields.voterBump
    this.voterWeightRecordBump = fields.voterWeightRecordBump
    this.reserved = fields.reserved
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<Voter | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<Voter | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): Voter {
    if (!data.slice(0, 8).equals(Voter.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Voter.layout.decode(data.slice(8))

    return new Voter({
      voterAuthority: dec.voterAuthority,
      registrar: dec.registrar,
      deposits: dec.deposits.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.DepositEntry.fromDecoded(item)
      ),
      voterBump: dec.voterBump,
      voterWeightRecordBump: dec.voterWeightRecordBump,
      reserved: dec.reserved,
    })
  }

  toJSON(): VoterJSON {
    return {
      voterAuthority: this.voterAuthority.toString(),
      registrar: this.registrar.toString(),
      deposits: this.deposits.map((item) => item.toJSON()),
      voterBump: this.voterBump,
      voterWeightRecordBump: this.voterWeightRecordBump,
      reserved: this.reserved,
    }
  }

  static fromJSON(obj: VoterJSON): Voter {
    return new Voter({
      voterAuthority: new PublicKey(obj.voterAuthority),
      registrar: new PublicKey(obj.registrar),
      deposits: obj.deposits.map((item) => types.DepositEntry.fromJSON(item)),
      voterBump: obj.voterBump,
      voterWeightRecordBump: obj.voterWeightRecordBump,
      reserved: obj.reserved,
    })
  }
}
