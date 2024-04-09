import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ConfigureVotingMintArgs {
  idx: number
  digitShift: number
  baselineVoteWeightScaledFactor: BN
  maxExtraLockupVoteWeightScaledFactor: BN
  lockupSaturationSecs: BN
  grantAuthority: PublicKey | null
}

export interface ConfigureVotingMintAccounts {
  registrar: PublicKey
  realmAuthority: PublicKey
  mint: PublicKey
}

export const layout = borsh.struct([
  borsh.u16("idx"),
  borsh.i8("digitShift"),
  borsh.u64("baselineVoteWeightScaledFactor"),
  borsh.u64("maxExtraLockupVoteWeightScaledFactor"),
  borsh.u64("lockupSaturationSecs"),
  borsh.option(borsh.publicKey(), "grantAuthority"),
])

export function configureVotingMint(
  args: ConfigureVotingMintArgs,
  accounts: ConfigureVotingMintAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: true },
    { pubkey: accounts.realmAuthority, isSigner: true, isWritable: false },
    { pubkey: accounts.mint, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([113, 153, 141, 236, 184, 9, 135, 15])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      idx: args.idx,
      digitShift: args.digitShift,
      baselineVoteWeightScaledFactor: args.baselineVoteWeightScaledFactor,
      maxExtraLockupVoteWeightScaledFactor:
        args.maxExtraLockupVoteWeightScaledFactor,
      lockupSaturationSecs: args.lockupSaturationSecs,
      grantAuthority: args.grantAuthority,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
