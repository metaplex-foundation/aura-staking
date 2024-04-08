import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface UnlockTokensArgs {
  depositEntryIndex: number
}

export interface UnlockTokensAccounts {
  registrar: PublicKey
  voter: PublicKey
  voterAuthority: PublicKey
}

export const layout = borsh.struct([borsh.u8("depositEntryIndex")])

export function unlockTokens(
  args: UnlockTokensArgs,
  accounts: UnlockTokensAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: true },
    { pubkey: accounts.voterAuthority, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([233, 35, 95, 159, 37, 185, 47, 88])
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
