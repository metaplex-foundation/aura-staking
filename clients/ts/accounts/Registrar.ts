import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface RegistrarFields {
  governanceProgramId: PublicKey
  realm: PublicKey
  realmGoverningTokenMint: PublicKey
  realmAuthority: PublicKey
  reserved1: Array<number>
  votingMints: Array<types.VotingMintConfigFields>
  timeOffset: BN
  bump: number
  reserved2: Array<number>
  reserved3: Array<BN>
}

export interface RegistrarJSON {
  governanceProgramId: string
  realm: string
  realmGoverningTokenMint: string
  realmAuthority: string
  reserved1: Array<number>
  votingMints: Array<types.VotingMintConfigJSON>
  timeOffset: string
  bump: number
  reserved2: Array<number>
  reserved3: Array<string>
}

export class Registrar {
  readonly governanceProgramId: PublicKey
  readonly realm: PublicKey
  readonly realmGoverningTokenMint: PublicKey
  readonly realmAuthority: PublicKey
  readonly reserved1: Array<number>
  readonly votingMints: Array<types.VotingMintConfig>
  readonly timeOffset: BN
  readonly bump: number
  readonly reserved2: Array<number>
  readonly reserved3: Array<BN>

  static readonly discriminator = Buffer.from([
    193, 202, 205, 51, 78, 168, 150, 128,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("governanceProgramId"),
    borsh.publicKey("realm"),
    borsh.publicKey("realmGoverningTokenMint"),
    borsh.publicKey("realmAuthority"),
    borsh.array(borsh.u8(), 32, "reserved1"),
    borsh.array(types.VotingMintConfig.layout(), 4, "votingMints"),
    borsh.i64("timeOffset"),
    borsh.u8("bump"),
    borsh.array(borsh.u8(), 7, "reserved2"),
    borsh.array(borsh.u64(), 11, "reserved3"),
  ])

  constructor(fields: RegistrarFields) {
    this.governanceProgramId = fields.governanceProgramId
    this.realm = fields.realm
    this.realmGoverningTokenMint = fields.realmGoverningTokenMint
    this.realmAuthority = fields.realmAuthority
    this.reserved1 = fields.reserved1
    this.votingMints = fields.votingMints.map(
      (item) => new types.VotingMintConfig({ ...item })
    )
    this.timeOffset = fields.timeOffset
    this.bump = fields.bump
    this.reserved2 = fields.reserved2
    this.reserved3 = fields.reserved3
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<Registrar | null> {
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
  ): Promise<Array<Registrar | null>> {
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

  static decode(data: Buffer): Registrar {
    if (!data.slice(0, 8).equals(Registrar.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Registrar.layout.decode(data.slice(8))

    return new Registrar({
      governanceProgramId: dec.governanceProgramId,
      realm: dec.realm,
      realmGoverningTokenMint: dec.realmGoverningTokenMint,
      realmAuthority: dec.realmAuthority,
      reserved1: dec.reserved1,
      votingMints: dec.votingMints.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.VotingMintConfig.fromDecoded(item)
      ),
      timeOffset: dec.timeOffset,
      bump: dec.bump,
      reserved2: dec.reserved2,
      reserved3: dec.reserved3,
    })
  }

  toJSON(): RegistrarJSON {
    return {
      governanceProgramId: this.governanceProgramId.toString(),
      realm: this.realm.toString(),
      realmGoverningTokenMint: this.realmGoverningTokenMint.toString(),
      realmAuthority: this.realmAuthority.toString(),
      reserved1: this.reserved1,
      votingMints: this.votingMints.map((item) => item.toJSON()),
      timeOffset: this.timeOffset.toString(),
      bump: this.bump,
      reserved2: this.reserved2,
      reserved3: this.reserved3.map((item) => item.toString()),
    }
  }

  static fromJSON(obj: RegistrarJSON): Registrar {
    return new Registrar({
      governanceProgramId: new PublicKey(obj.governanceProgramId),
      realm: new PublicKey(obj.realm),
      realmGoverningTokenMint: new PublicKey(obj.realmGoverningTokenMint),
      realmAuthority: new PublicKey(obj.realmAuthority),
      reserved1: obj.reserved1,
      votingMints: obj.votingMints.map((item) =>
        types.VotingMintConfig.fromJSON(item)
      ),
      timeOffset: new BN(obj.timeOffset),
      bump: obj.bump,
      reserved2: obj.reserved2,
      reserved3: obj.reserved3.map((item) => new BN(item)),
    })
  }
}
