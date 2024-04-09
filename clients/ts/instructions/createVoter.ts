import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateVoterArgs {
  voterBump: number
  voterWeightRecordBump: number
}

export interface CreateVoterAccounts {
  registrar: PublicKey
  voter: PublicKey
  voterAuthority: PublicKey
  voterWeightRecord: PublicKey
  payer: PublicKey
  systemProgram: PublicKey
  rent: PublicKey
  instructions: PublicKey
}

export const layout = borsh.struct([
  borsh.u8("voterBump"),
  borsh.u8("voterWeightRecordBump"),
])

export function createVoter(
  args: CreateVoterArgs,
  accounts: CreateVoterAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: true },
    { pubkey: accounts.voterAuthority, isSigner: true, isWritable: false },
    { pubkey: accounts.voterWeightRecord, isSigner: false, isWritable: true },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
    { pubkey: accounts.instructions, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([6, 24, 245, 52, 243, 255, 148, 25])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      voterBump: args.voterBump,
      voterWeightRecordBump: args.voterWeightRecordBump,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
