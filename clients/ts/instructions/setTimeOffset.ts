import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SetTimeOffsetArgs {
  timeOffset: BN
}

export interface SetTimeOffsetAccounts {
  registrar: PublicKey
  realmAuthority: PublicKey
}

export const layout = borsh.struct([borsh.i64("timeOffset")])

export function setTimeOffset(
  args: SetTimeOffsetArgs,
  accounts: SetTimeOffsetAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: true },
    { pubkey: accounts.realmAuthority, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([89, 238, 89, 160, 239, 113, 25, 123])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      timeOffset: args.timeOffset,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
