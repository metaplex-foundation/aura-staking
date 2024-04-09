import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateRegistrarArgs {
  registrarBump: number
}

export interface CreateRegistrarAccounts {
  registrar: PublicKey
  realm: PublicKey
  governanceProgramId: PublicKey
  realmGoverningTokenMint: PublicKey
  realmAuthority: PublicKey
  payer: PublicKey
  systemProgram: PublicKey
  rent: PublicKey
}

export const layout = borsh.struct([borsh.u8("registrarBump")])

export function createRegistrar(
  args: CreateRegistrarArgs,
  accounts: CreateRegistrarAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: true },
    { pubkey: accounts.realm, isSigner: false, isWritable: false },
    {
      pubkey: accounts.governanceProgramId,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: accounts.realmGoverningTokenMint,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.realmAuthority, isSigner: true, isWritable: false },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([132, 235, 36, 49, 139, 66, 202, 69])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      registrarBump: args.registrarBump,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
