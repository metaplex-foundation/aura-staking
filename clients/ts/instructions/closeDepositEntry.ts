import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CloseDepositEntryArgs {
  depositEntryIndex: number
}

export interface CloseDepositEntryAccounts {
  voter: PublicKey
  voterAuthority: PublicKey
}

export const layout = borsh.struct([borsh.u8("depositEntryIndex")])

export function closeDepositEntry(
  args: CloseDepositEntryArgs,
  accounts: CloseDepositEntryAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.voter, isSigner: false, isWritable: true },
    { pubkey: accounts.voterAuthority, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([236, 190, 87, 34, 251, 131, 138, 237])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      depositEntryIndex: args.depositEntryIndex,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
