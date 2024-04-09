import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateDepositEntryArgs {
  depositEntryIndex: number
  kind: types.LockupKindKind
  startTs: BN | null
  period: types.LockupPeriodKind
}

export interface CreateDepositEntryAccounts {
  registrar: PublicKey
  voter: PublicKey
  vault: PublicKey
  voterAuthority: PublicKey
  payer: PublicKey
  depositMint: PublicKey
  systemProgram: PublicKey
  tokenProgram: PublicKey
  associatedTokenProgram: PublicKey
  rent: PublicKey
}

export const layout = borsh.struct([
  borsh.u8("depositEntryIndex"),
  types.LockupKind.layout("kind"),
  borsh.option(borsh.u64(), "startTs"),
  types.LockupPeriod.layout("period"),
])

export function createDepositEntry(
  args: CreateDepositEntryArgs,
  accounts: CreateDepositEntryAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: true },
    { pubkey: accounts.vault, isSigner: false, isWritable: true },
    { pubkey: accounts.voterAuthority, isSigner: true, isWritable: false },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.depositMint, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.rent, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([185, 131, 167, 186, 159, 125, 19, 67])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      depositEntryIndex: args.depositEntryIndex,
      kind: args.kind.toEncodable(),
      startTs: args.startTs,
      period: args.period.toEncodable(),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
