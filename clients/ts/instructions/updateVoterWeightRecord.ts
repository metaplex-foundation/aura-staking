import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface UpdateVoterWeightRecordAccounts {
  registrar: PublicKey
  voter: PublicKey
  voterWeightRecord: PublicKey
  systemProgram: PublicKey
}

export function updateVoterWeightRecord(
  accounts: UpdateVoterWeightRecordAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.registrar, isSigner: false, isWritable: false },
    { pubkey: accounts.voter, isSigner: false, isWritable: false },
    { pubkey: accounts.voterWeightRecord, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([45, 185, 3, 36, 109, 190, 115, 169])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
