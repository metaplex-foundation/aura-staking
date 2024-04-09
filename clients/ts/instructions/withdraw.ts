import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface WithdrawArgs {
  depositEntryIndex: number
  amount: BN
}

export interface WithdrawAccounts {
  registrar: PublicKey
  voter: PublicKey
  voterAuthority: PublicKey
  tokenOwnerRecord: PublicKey
  voterWeightRecord: PublicKey
  vault: PublicKey
  destination: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u8("depositEntryIndex"),
  borsh.u64("amount"),
])

export function withdraw(
  args: WithdrawArgs,
  accounts: WithdrawAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: true },
    { pubkey: accounts.voterAuthority, isSigner: true, isWritable: false },
    { pubkey: accounts.tokenOwnerRecord, isSigner: false, isWritable: false },
    { pubkey: accounts.voterWeightRecord, isSigner: false, isWritable: true },
    { pubkey: accounts.vault, isSigner: false, isWritable: true },
    { pubkey: accounts.destination, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([183, 18, 70, 156, 148, 109, 161, 34])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      depositEntryIndex: args.depositEntryIndex,
      amount: args.amount,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
