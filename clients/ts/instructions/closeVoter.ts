import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CloseVoterAccounts {
  registrar: PublicKey
  voter: PublicKey
  voterAuthority: PublicKey
  solDestination: PublicKey
  tokenProgram: PublicKey
}

export function closeVoter(
  accounts: CloseVoterAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: true },
    { pubkey: accounts.voterAuthority, isSigner: true, isWritable: false },
    { pubkey: accounts.solDestination, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([117, 35, 234, 247, 206, 131, 182, 149])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
