import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LogVoterInfoArgs {
  depositEntryBegin: number
  depositEntryCount: number
}

export interface LogVoterInfoAccounts {
  registrar: PublicKey
  voter: PublicKey
}

export const layout = borsh.struct([
  borsh.u8("depositEntryBegin"),
  borsh.u8("depositEntryCount"),
])

export function logVoterInfo(
  args: LogVoterInfoArgs,
  accounts: LogVoterInfoAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([171, 72, 233, 90, 143, 151, 113, 51])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      depositEntryBegin: args.depositEntryBegin,
      depositEntryCount: args.depositEntryCount,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
