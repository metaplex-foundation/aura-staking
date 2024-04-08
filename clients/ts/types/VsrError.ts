import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface InvalidRateJSON {
  kind: "InvalidRate"
}

export class InvalidRate {
  static readonly discriminator = 0
  static readonly kind = "InvalidRate"
  readonly discriminator = 0
  readonly kind = "InvalidRate"

  toJSON(): InvalidRateJSON {
    return {
      kind: "InvalidRate",
    }
  }

  toEncodable() {
    return {
      InvalidRate: {},
    }
  }
}

export interface RatesFullJSON {
  kind: "RatesFull"
}

export class RatesFull {
  static readonly discriminator = 1
  static readonly kind = "RatesFull"
  readonly discriminator = 1
  readonly kind = "RatesFull"

  toJSON(): RatesFullJSON {
    return {
      kind: "RatesFull",
    }
  }

  toEncodable() {
    return {
      RatesFull: {},
    }
  }
}

export interface VotingMintNotFoundJSON {
  kind: "VotingMintNotFound"
}

export class VotingMintNotFound {
  static readonly discriminator = 2
  static readonly kind = "VotingMintNotFound"
  readonly discriminator = 2
  readonly kind = "VotingMintNotFound"

  toJSON(): VotingMintNotFoundJSON {
    return {
      kind: "VotingMintNotFound",
    }
  }

  toEncodable() {
    return {
      VotingMintNotFound: {},
    }
  }
}

export interface DepositEntryNotFoundJSON {
  kind: "DepositEntryNotFound"
}

export class DepositEntryNotFound {
  static readonly discriminator = 3
  static readonly kind = "DepositEntryNotFound"
  readonly discriminator = 3
  readonly kind = "DepositEntryNotFound"

  toJSON(): DepositEntryNotFoundJSON {
    return {
      kind: "DepositEntryNotFound",
    }
  }

  toEncodable() {
    return {
      DepositEntryNotFound: {},
    }
  }
}

export interface DepositEntryFullJSON {
  kind: "DepositEntryFull"
}

export class DepositEntryFull {
  static readonly discriminator = 4
  static readonly kind = "DepositEntryFull"
  readonly discriminator = 4
  readonly kind = "DepositEntryFull"

  toJSON(): DepositEntryFullJSON {
    return {
      kind: "DepositEntryFull",
    }
  }

  toEncodable() {
    return {
      DepositEntryFull: {},
    }
  }
}

export interface VotingTokenNonZeroJSON {
  kind: "VotingTokenNonZero"
}

export class VotingTokenNonZero {
  static readonly discriminator = 5
  static readonly kind = "VotingTokenNonZero"
  readonly discriminator = 5
  readonly kind = "VotingTokenNonZero"

  toJSON(): VotingTokenNonZeroJSON {
    return {
      kind: "VotingTokenNonZero",
    }
  }

  toEncodable() {
    return {
      VotingTokenNonZero: {},
    }
  }
}

export interface OutOfBoundsDepositEntryIndexJSON {
  kind: "OutOfBoundsDepositEntryIndex"
}

export class OutOfBoundsDepositEntryIndex {
  static readonly discriminator = 6
  static readonly kind = "OutOfBoundsDepositEntryIndex"
  readonly discriminator = 6
  readonly kind = "OutOfBoundsDepositEntryIndex"

  toJSON(): OutOfBoundsDepositEntryIndexJSON {
    return {
      kind: "OutOfBoundsDepositEntryIndex",
    }
  }

  toEncodable() {
    return {
      OutOfBoundsDepositEntryIndex: {},
    }
  }
}

export interface UnusedDepositEntryIndexJSON {
  kind: "UnusedDepositEntryIndex"
}

export class UnusedDepositEntryIndex {
  static readonly discriminator = 7
  static readonly kind = "UnusedDepositEntryIndex"
  readonly discriminator = 7
  readonly kind = "UnusedDepositEntryIndex"

  toJSON(): UnusedDepositEntryIndexJSON {
    return {
      kind: "UnusedDepositEntryIndex",
    }
  }

  toEncodable() {
    return {
      UnusedDepositEntryIndex: {},
    }
  }
}

export interface InsufficientUnlockedTokensJSON {
  kind: "InsufficientUnlockedTokens"
}

export class InsufficientUnlockedTokens {
  static readonly discriminator = 8
  static readonly kind = "InsufficientUnlockedTokens"
  readonly discriminator = 8
  readonly kind = "InsufficientUnlockedTokens"

  toJSON(): InsufficientUnlockedTokensJSON {
    return {
      kind: "InsufficientUnlockedTokens",
    }
  }

  toEncodable() {
    return {
      InsufficientUnlockedTokens: {},
    }
  }
}

export interface UnableToConvertJSON {
  kind: "UnableToConvert"
}

export class UnableToConvert {
  static readonly discriminator = 9
  static readonly kind = "UnableToConvert"
  readonly discriminator = 9
  readonly kind = "UnableToConvert"

  toJSON(): UnableToConvertJSON {
    return {
      kind: "UnableToConvert",
    }
  }

  toEncodable() {
    return {
      UnableToConvert: {},
    }
  }
}

export interface InvalidLockupPeriodJSON {
  kind: "InvalidLockupPeriod"
}

export class InvalidLockupPeriod {
  static readonly discriminator = 10
  static readonly kind = "InvalidLockupPeriod"
  readonly discriminator = 10
  readonly kind = "InvalidLockupPeriod"

  toJSON(): InvalidLockupPeriodJSON {
    return {
      kind: "InvalidLockupPeriod",
    }
  }

  toEncodable() {
    return {
      InvalidLockupPeriod: {},
    }
  }
}

export interface InvalidEndTsJSON {
  kind: "InvalidEndTs"
}

export class InvalidEndTs {
  static readonly discriminator = 11
  static readonly kind = "InvalidEndTs"
  readonly discriminator = 11
  readonly kind = "InvalidEndTs"

  toJSON(): InvalidEndTsJSON {
    return {
      kind: "InvalidEndTs",
    }
  }

  toEncodable() {
    return {
      InvalidEndTs: {},
    }
  }
}

export interface InvalidDaysJSON {
  kind: "InvalidDays"
}

export class InvalidDays {
  static readonly discriminator = 12
  static readonly kind = "InvalidDays"
  readonly discriminator = 12
  readonly kind = "InvalidDays"

  toJSON(): InvalidDaysJSON {
    return {
      kind: "InvalidDays",
    }
  }

  toEncodable() {
    return {
      InvalidDays: {},
    }
  }
}

export interface VotingMintConfigIndexAlreadyInUseJSON {
  kind: "VotingMintConfigIndexAlreadyInUse"
}

export class VotingMintConfigIndexAlreadyInUse {
  static readonly discriminator = 13
  static readonly kind = "VotingMintConfigIndexAlreadyInUse"
  readonly discriminator = 13
  readonly kind = "VotingMintConfigIndexAlreadyInUse"

  toJSON(): VotingMintConfigIndexAlreadyInUseJSON {
    return {
      kind: "VotingMintConfigIndexAlreadyInUse",
    }
  }

  toEncodable() {
    return {
      VotingMintConfigIndexAlreadyInUse: {},
    }
  }
}

export interface OutOfBoundsVotingMintConfigIndexJSON {
  kind: "OutOfBoundsVotingMintConfigIndex"
}

export class OutOfBoundsVotingMintConfigIndex {
  static readonly discriminator = 14
  static readonly kind = "OutOfBoundsVotingMintConfigIndex"
  readonly discriminator = 14
  readonly kind = "OutOfBoundsVotingMintConfigIndex"

  toJSON(): OutOfBoundsVotingMintConfigIndexJSON {
    return {
      kind: "OutOfBoundsVotingMintConfigIndex",
    }
  }

  toEncodable() {
    return {
      OutOfBoundsVotingMintConfigIndex: {},
    }
  }
}

export interface InvalidDecimalsJSON {
  kind: "InvalidDecimals"
}

export class InvalidDecimals {
  static readonly discriminator = 15
  static readonly kind = "InvalidDecimals"
  readonly discriminator = 15
  readonly kind = "InvalidDecimals"

  toJSON(): InvalidDecimalsJSON {
    return {
      kind: "InvalidDecimals",
    }
  }

  toEncodable() {
    return {
      InvalidDecimals: {},
    }
  }
}

export interface InvalidToDepositAndWithdrawInOneSlotJSON {
  kind: "InvalidToDepositAndWithdrawInOneSlot"
}

export class InvalidToDepositAndWithdrawInOneSlot {
  static readonly discriminator = 16
  static readonly kind = "InvalidToDepositAndWithdrawInOneSlot"
  readonly discriminator = 16
  readonly kind = "InvalidToDepositAndWithdrawInOneSlot"

  toJSON(): InvalidToDepositAndWithdrawInOneSlotJSON {
    return {
      kind: "InvalidToDepositAndWithdrawInOneSlot",
    }
  }

  toEncodable() {
    return {
      InvalidToDepositAndWithdrawInOneSlot: {},
    }
  }
}

export interface ShouldBeTheFirstIxInATxJSON {
  kind: "ShouldBeTheFirstIxInATx"
}

export class ShouldBeTheFirstIxInATx {
  static readonly discriminator = 17
  static readonly kind = "ShouldBeTheFirstIxInATx"
  readonly discriminator = 17
  readonly kind = "ShouldBeTheFirstIxInATx"

  toJSON(): ShouldBeTheFirstIxInATxJSON {
    return {
      kind: "ShouldBeTheFirstIxInATx",
    }
  }

  toEncodable() {
    return {
      ShouldBeTheFirstIxInATx: {},
    }
  }
}

export interface ForbiddenCpiJSON {
  kind: "ForbiddenCpi"
}

export class ForbiddenCpi {
  static readonly discriminator = 18
  static readonly kind = "ForbiddenCpi"
  readonly discriminator = 18
  readonly kind = "ForbiddenCpi"

  toJSON(): ForbiddenCpiJSON {
    return {
      kind: "ForbiddenCpi",
    }
  }

  toEncodable() {
    return {
      ForbiddenCpi: {},
    }
  }
}

export interface InvalidMintJSON {
  kind: "InvalidMint"
}

export class InvalidMint {
  static readonly discriminator = 19
  static readonly kind = "InvalidMint"
  readonly discriminator = 19
  readonly kind = "InvalidMint"

  toJSON(): InvalidMintJSON {
    return {
      kind: "InvalidMint",
    }
  }

  toEncodable() {
    return {
      InvalidMint: {},
    }
  }
}

export interface DebugInstructionJSON {
  kind: "DebugInstruction"
}

export class DebugInstruction {
  static readonly discriminator = 20
  static readonly kind = "DebugInstruction"
  readonly discriminator = 20
  readonly kind = "DebugInstruction"

  toJSON(): DebugInstructionJSON {
    return {
      kind: "DebugInstruction",
    }
  }

  toEncodable() {
    return {
      DebugInstruction: {},
    }
  }
}

export interface ClawbackNotAllowedOnDepositJSON {
  kind: "ClawbackNotAllowedOnDeposit"
}

export class ClawbackNotAllowedOnDeposit {
  static readonly discriminator = 21
  static readonly kind = "ClawbackNotAllowedOnDeposit"
  readonly discriminator = 21
  readonly kind = "ClawbackNotAllowedOnDeposit"

  toJSON(): ClawbackNotAllowedOnDepositJSON {
    return {
      kind: "ClawbackNotAllowedOnDeposit",
    }
  }

  toEncodable() {
    return {
      ClawbackNotAllowedOnDeposit: {},
    }
  }
}

export interface DepositStillLockedJSON {
  kind: "DepositStillLocked"
}

export class DepositStillLocked {
  static readonly discriminator = 22
  static readonly kind = "DepositStillLocked"
  readonly discriminator = 22
  readonly kind = "DepositStillLocked"

  toJSON(): DepositStillLockedJSON {
    return {
      kind: "DepositStillLocked",
    }
  }

  toEncodable() {
    return {
      DepositStillLocked: {},
    }
  }
}

export interface InvalidAuthorityJSON {
  kind: "InvalidAuthority"
}

export class InvalidAuthority {
  static readonly discriminator = 23
  static readonly kind = "InvalidAuthority"
  readonly discriminator = 23
  readonly kind = "InvalidAuthority"

  toJSON(): InvalidAuthorityJSON {
    return {
      kind: "InvalidAuthority",
    }
  }

  toEncodable() {
    return {
      InvalidAuthority: {},
    }
  }
}

export interface InvalidTokenOwnerRecordJSON {
  kind: "InvalidTokenOwnerRecord"
}

export class InvalidTokenOwnerRecord {
  static readonly discriminator = 24
  static readonly kind = "InvalidTokenOwnerRecord"
  readonly discriminator = 24
  readonly kind = "InvalidTokenOwnerRecord"

  toJSON(): InvalidTokenOwnerRecordJSON {
    return {
      kind: "InvalidTokenOwnerRecord",
    }
  }

  toEncodable() {
    return {
      InvalidTokenOwnerRecord: {},
    }
  }
}

export interface InvalidRealmAuthorityJSON {
  kind: "InvalidRealmAuthority"
}

export class InvalidRealmAuthority {
  static readonly discriminator = 25
  static readonly kind = "InvalidRealmAuthority"
  readonly discriminator = 25
  readonly kind = "InvalidRealmAuthority"

  toJSON(): InvalidRealmAuthorityJSON {
    return {
      kind: "InvalidRealmAuthority",
    }
  }

  toEncodable() {
    return {
      InvalidRealmAuthority: {},
    }
  }
}

export interface VoterWeightOverflowJSON {
  kind: "VoterWeightOverflow"
}

export class VoterWeightOverflow {
  static readonly discriminator = 26
  static readonly kind = "VoterWeightOverflow"
  readonly discriminator = 26
  readonly kind = "VoterWeightOverflow"

  toJSON(): VoterWeightOverflowJSON {
    return {
      kind: "VoterWeightOverflow",
    }
  }

  toEncodable() {
    return {
      VoterWeightOverflow: {},
    }
  }
}

export interface LockupSaturationMustBePositiveJSON {
  kind: "LockupSaturationMustBePositive"
}

export class LockupSaturationMustBePositive {
  static readonly discriminator = 27
  static readonly kind = "LockupSaturationMustBePositive"
  readonly discriminator = 27
  readonly kind = "LockupSaturationMustBePositive"

  toJSON(): LockupSaturationMustBePositiveJSON {
    return {
      kind: "LockupSaturationMustBePositive",
    }
  }

  toEncodable() {
    return {
      LockupSaturationMustBePositive: {},
    }
  }
}

export interface VotingMintConfiguredWithDifferentIndexJSON {
  kind: "VotingMintConfiguredWithDifferentIndex"
}

export class VotingMintConfiguredWithDifferentIndex {
  static readonly discriminator = 28
  static readonly kind = "VotingMintConfiguredWithDifferentIndex"
  readonly discriminator = 28
  readonly kind = "VotingMintConfiguredWithDifferentIndex"

  toJSON(): VotingMintConfiguredWithDifferentIndexJSON {
    return {
      kind: "VotingMintConfiguredWithDifferentIndex",
    }
  }

  toEncodable() {
    return {
      VotingMintConfiguredWithDifferentIndex: {},
    }
  }
}

export interface InternalProgramErrorJSON {
  kind: "InternalProgramError"
}

export class InternalProgramError {
  static readonly discriminator = 29
  static readonly kind = "InternalProgramError"
  readonly discriminator = 29
  readonly kind = "InternalProgramError"

  toJSON(): InternalProgramErrorJSON {
    return {
      kind: "InternalProgramError",
    }
  }

  toEncodable() {
    return {
      InternalProgramError: {},
    }
  }
}

export interface InsufficientLockedTokensJSON {
  kind: "InsufficientLockedTokens"
}

export class InsufficientLockedTokens {
  static readonly discriminator = 30
  static readonly kind = "InsufficientLockedTokens"
  readonly discriminator = 30
  readonly kind = "InsufficientLockedTokens"

  toJSON(): InsufficientLockedTokensJSON {
    return {
      kind: "InsufficientLockedTokens",
    }
  }

  toEncodable() {
    return {
      InsufficientLockedTokens: {},
    }
  }
}

export interface MustKeepTokensLockedJSON {
  kind: "MustKeepTokensLocked"
}

export class MustKeepTokensLocked {
  static readonly discriminator = 31
  static readonly kind = "MustKeepTokensLocked"
  readonly discriminator = 31
  readonly kind = "MustKeepTokensLocked"

  toJSON(): MustKeepTokensLockedJSON {
    return {
      kind: "MustKeepTokensLocked",
    }
  }

  toEncodable() {
    return {
      MustKeepTokensLocked: {},
    }
  }
}

export interface InvalidLockupKindJSON {
  kind: "InvalidLockupKind"
}

export class InvalidLockupKind {
  static readonly discriminator = 32
  static readonly kind = "InvalidLockupKind"
  readonly discriminator = 32
  readonly kind = "InvalidLockupKind"

  toJSON(): InvalidLockupKindJSON {
    return {
      kind: "InvalidLockupKind",
    }
  }

  toEncodable() {
    return {
      InvalidLockupKind: {},
    }
  }
}

export interface InvalidChangeToClawbackDepositEntryJSON {
  kind: "InvalidChangeToClawbackDepositEntry"
}

export class InvalidChangeToClawbackDepositEntry {
  static readonly discriminator = 33
  static readonly kind = "InvalidChangeToClawbackDepositEntry"
  readonly discriminator = 33
  readonly kind = "InvalidChangeToClawbackDepositEntry"

  toJSON(): InvalidChangeToClawbackDepositEntryJSON {
    return {
      kind: "InvalidChangeToClawbackDepositEntry",
    }
  }

  toEncodable() {
    return {
      InvalidChangeToClawbackDepositEntry: {},
    }
  }
}

export interface InternalErrorBadLockupVoteWeightJSON {
  kind: "InternalErrorBadLockupVoteWeight"
}

export class InternalErrorBadLockupVoteWeight {
  static readonly discriminator = 34
  static readonly kind = "InternalErrorBadLockupVoteWeight"
  readonly discriminator = 34
  readonly kind = "InternalErrorBadLockupVoteWeight"

  toJSON(): InternalErrorBadLockupVoteWeightJSON {
    return {
      kind: "InternalErrorBadLockupVoteWeight",
    }
  }

  toEncodable() {
    return {
      InternalErrorBadLockupVoteWeight: {},
    }
  }
}

export interface DepositStartTooFarInFutureJSON {
  kind: "DepositStartTooFarInFuture"
}

export class DepositStartTooFarInFuture {
  static readonly discriminator = 35
  static readonly kind = "DepositStartTooFarInFuture"
  readonly discriminator = 35
  readonly kind = "DepositStartTooFarInFuture"

  toJSON(): DepositStartTooFarInFutureJSON {
    return {
      kind: "DepositStartTooFarInFuture",
    }
  }

  toEncodable() {
    return {
      DepositStartTooFarInFuture: {},
    }
  }
}

export interface VaultTokenNonZeroJSON {
  kind: "VaultTokenNonZero"
}

export class VaultTokenNonZero {
  static readonly discriminator = 36
  static readonly kind = "VaultTokenNonZero"
  readonly discriminator = 36
  readonly kind = "VaultTokenNonZero"

  toJSON(): VaultTokenNonZeroJSON {
    return {
      kind: "VaultTokenNonZero",
    }
  }

  toEncodable() {
    return {
      VaultTokenNonZero: {},
    }
  }
}

export interface InvalidTimestampArgumentsJSON {
  kind: "InvalidTimestampArguments"
}

export class InvalidTimestampArguments {
  static readonly discriminator = 37
  static readonly kind = "InvalidTimestampArguments"
  readonly discriminator = 37
  readonly kind = "InvalidTimestampArguments"

  toJSON(): InvalidTimestampArgumentsJSON {
    return {
      kind: "InvalidTimestampArguments",
    }
  }

  toEncodable() {
    return {
      InvalidTimestampArguments: {},
    }
  }
}

export interface UnlockMustBeCalledFirstJSON {
  kind: "UnlockMustBeCalledFirst"
}

export class UnlockMustBeCalledFirst {
  static readonly discriminator = 38
  static readonly kind = "UnlockMustBeCalledFirst"
  readonly discriminator = 38
  readonly kind = "UnlockMustBeCalledFirst"

  toJSON(): UnlockMustBeCalledFirstJSON {
    return {
      kind: "UnlockMustBeCalledFirst",
    }
  }

  toEncodable() {
    return {
      UnlockMustBeCalledFirst: {},
    }
  }
}

export interface UnlockAlreadyRequestedJSON {
  kind: "UnlockAlreadyRequested"
}

export class UnlockAlreadyRequested {
  static readonly discriminator = 39
  static readonly kind = "UnlockAlreadyRequested"
  readonly discriminator = 39
  readonly kind = "UnlockAlreadyRequested"

  toJSON(): UnlockAlreadyRequestedJSON {
    return {
      kind: "UnlockAlreadyRequested",
    }
  }

  toEncodable() {
    return {
      UnlockAlreadyRequested: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.VsrErrorKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("InvalidRate" in obj) {
    return new InvalidRate()
  }
  if ("RatesFull" in obj) {
    return new RatesFull()
  }
  if ("VotingMintNotFound" in obj) {
    return new VotingMintNotFound()
  }
  if ("DepositEntryNotFound" in obj) {
    return new DepositEntryNotFound()
  }
  if ("DepositEntryFull" in obj) {
    return new DepositEntryFull()
  }
  if ("VotingTokenNonZero" in obj) {
    return new VotingTokenNonZero()
  }
  if ("OutOfBoundsDepositEntryIndex" in obj) {
    return new OutOfBoundsDepositEntryIndex()
  }
  if ("UnusedDepositEntryIndex" in obj) {
    return new UnusedDepositEntryIndex()
  }
  if ("InsufficientUnlockedTokens" in obj) {
    return new InsufficientUnlockedTokens()
  }
  if ("UnableToConvert" in obj) {
    return new UnableToConvert()
  }
  if ("InvalidLockupPeriod" in obj) {
    return new InvalidLockupPeriod()
  }
  if ("InvalidEndTs" in obj) {
    return new InvalidEndTs()
  }
  if ("InvalidDays" in obj) {
    return new InvalidDays()
  }
  if ("VotingMintConfigIndexAlreadyInUse" in obj) {
    return new VotingMintConfigIndexAlreadyInUse()
  }
  if ("OutOfBoundsVotingMintConfigIndex" in obj) {
    return new OutOfBoundsVotingMintConfigIndex()
  }
  if ("InvalidDecimals" in obj) {
    return new InvalidDecimals()
  }
  if ("InvalidToDepositAndWithdrawInOneSlot" in obj) {
    return new InvalidToDepositAndWithdrawInOneSlot()
  }
  if ("ShouldBeTheFirstIxInATx" in obj) {
    return new ShouldBeTheFirstIxInATx()
  }
  if ("ForbiddenCpi" in obj) {
    return new ForbiddenCpi()
  }
  if ("InvalidMint" in obj) {
    return new InvalidMint()
  }
  if ("DebugInstruction" in obj) {
    return new DebugInstruction()
  }
  if ("ClawbackNotAllowedOnDeposit" in obj) {
    return new ClawbackNotAllowedOnDeposit()
  }
  if ("DepositStillLocked" in obj) {
    return new DepositStillLocked()
  }
  if ("InvalidAuthority" in obj) {
    return new InvalidAuthority()
  }
  if ("InvalidTokenOwnerRecord" in obj) {
    return new InvalidTokenOwnerRecord()
  }
  if ("InvalidRealmAuthority" in obj) {
    return new InvalidRealmAuthority()
  }
  if ("VoterWeightOverflow" in obj) {
    return new VoterWeightOverflow()
  }
  if ("LockupSaturationMustBePositive" in obj) {
    return new LockupSaturationMustBePositive()
  }
  if ("VotingMintConfiguredWithDifferentIndex" in obj) {
    return new VotingMintConfiguredWithDifferentIndex()
  }
  if ("InternalProgramError" in obj) {
    return new InternalProgramError()
  }
  if ("InsufficientLockedTokens" in obj) {
    return new InsufficientLockedTokens()
  }
  if ("MustKeepTokensLocked" in obj) {
    return new MustKeepTokensLocked()
  }
  if ("InvalidLockupKind" in obj) {
    return new InvalidLockupKind()
  }
  if ("InvalidChangeToClawbackDepositEntry" in obj) {
    return new InvalidChangeToClawbackDepositEntry()
  }
  if ("InternalErrorBadLockupVoteWeight" in obj) {
    return new InternalErrorBadLockupVoteWeight()
  }
  if ("DepositStartTooFarInFuture" in obj) {
    return new DepositStartTooFarInFuture()
  }
  if ("VaultTokenNonZero" in obj) {
    return new VaultTokenNonZero()
  }
  if ("InvalidTimestampArguments" in obj) {
    return new InvalidTimestampArguments()
  }
  if ("UnlockMustBeCalledFirst" in obj) {
    return new UnlockMustBeCalledFirst()
  }
  if ("UnlockAlreadyRequested" in obj) {
    return new UnlockAlreadyRequested()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(obj: types.VsrErrorJSON): types.VsrErrorKind {
  switch (obj.kind) {
    case "InvalidRate": {
      return new InvalidRate()
    }
    case "RatesFull": {
      return new RatesFull()
    }
    case "VotingMintNotFound": {
      return new VotingMintNotFound()
    }
    case "DepositEntryNotFound": {
      return new DepositEntryNotFound()
    }
    case "DepositEntryFull": {
      return new DepositEntryFull()
    }
    case "VotingTokenNonZero": {
      return new VotingTokenNonZero()
    }
    case "OutOfBoundsDepositEntryIndex": {
      return new OutOfBoundsDepositEntryIndex()
    }
    case "UnusedDepositEntryIndex": {
      return new UnusedDepositEntryIndex()
    }
    case "InsufficientUnlockedTokens": {
      return new InsufficientUnlockedTokens()
    }
    case "UnableToConvert": {
      return new UnableToConvert()
    }
    case "InvalidLockupPeriod": {
      return new InvalidLockupPeriod()
    }
    case "InvalidEndTs": {
      return new InvalidEndTs()
    }
    case "InvalidDays": {
      return new InvalidDays()
    }
    case "VotingMintConfigIndexAlreadyInUse": {
      return new VotingMintConfigIndexAlreadyInUse()
    }
    case "OutOfBoundsVotingMintConfigIndex": {
      return new OutOfBoundsVotingMintConfigIndex()
    }
    case "InvalidDecimals": {
      return new InvalidDecimals()
    }
    case "InvalidToDepositAndWithdrawInOneSlot": {
      return new InvalidToDepositAndWithdrawInOneSlot()
    }
    case "ShouldBeTheFirstIxInATx": {
      return new ShouldBeTheFirstIxInATx()
    }
    case "ForbiddenCpi": {
      return new ForbiddenCpi()
    }
    case "InvalidMint": {
      return new InvalidMint()
    }
    case "DebugInstruction": {
      return new DebugInstruction()
    }
    case "ClawbackNotAllowedOnDeposit": {
      return new ClawbackNotAllowedOnDeposit()
    }
    case "DepositStillLocked": {
      return new DepositStillLocked()
    }
    case "InvalidAuthority": {
      return new InvalidAuthority()
    }
    case "InvalidTokenOwnerRecord": {
      return new InvalidTokenOwnerRecord()
    }
    case "InvalidRealmAuthority": {
      return new InvalidRealmAuthority()
    }
    case "VoterWeightOverflow": {
      return new VoterWeightOverflow()
    }
    case "LockupSaturationMustBePositive": {
      return new LockupSaturationMustBePositive()
    }
    case "VotingMintConfiguredWithDifferentIndex": {
      return new VotingMintConfiguredWithDifferentIndex()
    }
    case "InternalProgramError": {
      return new InternalProgramError()
    }
    case "InsufficientLockedTokens": {
      return new InsufficientLockedTokens()
    }
    case "MustKeepTokensLocked": {
      return new MustKeepTokensLocked()
    }
    case "InvalidLockupKind": {
      return new InvalidLockupKind()
    }
    case "InvalidChangeToClawbackDepositEntry": {
      return new InvalidChangeToClawbackDepositEntry()
    }
    case "InternalErrorBadLockupVoteWeight": {
      return new InternalErrorBadLockupVoteWeight()
    }
    case "DepositStartTooFarInFuture": {
      return new DepositStartTooFarInFuture()
    }
    case "VaultTokenNonZero": {
      return new VaultTokenNonZero()
    }
    case "InvalidTimestampArguments": {
      return new InvalidTimestampArguments()
    }
    case "UnlockMustBeCalledFirst": {
      return new UnlockMustBeCalledFirst()
    }
    case "UnlockAlreadyRequested": {
      return new UnlockAlreadyRequested()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "InvalidRate"),
    borsh.struct([], "RatesFull"),
    borsh.struct([], "VotingMintNotFound"),
    borsh.struct([], "DepositEntryNotFound"),
    borsh.struct([], "DepositEntryFull"),
    borsh.struct([], "VotingTokenNonZero"),
    borsh.struct([], "OutOfBoundsDepositEntryIndex"),
    borsh.struct([], "UnusedDepositEntryIndex"),
    borsh.struct([], "InsufficientUnlockedTokens"),
    borsh.struct([], "UnableToConvert"),
    borsh.struct([], "InvalidLockupPeriod"),
    borsh.struct([], "InvalidEndTs"),
    borsh.struct([], "InvalidDays"),
    borsh.struct([], "VotingMintConfigIndexAlreadyInUse"),
    borsh.struct([], "OutOfBoundsVotingMintConfigIndex"),
    borsh.struct([], "InvalidDecimals"),
    borsh.struct([], "InvalidToDepositAndWithdrawInOneSlot"),
    borsh.struct([], "ShouldBeTheFirstIxInATx"),
    borsh.struct([], "ForbiddenCpi"),
    borsh.struct([], "InvalidMint"),
    borsh.struct([], "DebugInstruction"),
    borsh.struct([], "ClawbackNotAllowedOnDeposit"),
    borsh.struct([], "DepositStillLocked"),
    borsh.struct([], "InvalidAuthority"),
    borsh.struct([], "InvalidTokenOwnerRecord"),
    borsh.struct([], "InvalidRealmAuthority"),
    borsh.struct([], "VoterWeightOverflow"),
    borsh.struct([], "LockupSaturationMustBePositive"),
    borsh.struct([], "VotingMintConfiguredWithDifferentIndex"),
    borsh.struct([], "InternalProgramError"),
    borsh.struct([], "InsufficientLockedTokens"),
    borsh.struct([], "MustKeepTokensLocked"),
    borsh.struct([], "InvalidLockupKind"),
    borsh.struct([], "InvalidChangeToClawbackDepositEntry"),
    borsh.struct([], "InternalErrorBadLockupVoteWeight"),
    borsh.struct([], "DepositStartTooFarInFuture"),
    borsh.struct([], "VaultTokenNonZero"),
    borsh.struct([], "InvalidTimestampArguments"),
    borsh.struct([], "UnlockMustBeCalledFirst"),
    borsh.struct([], "UnlockAlreadyRequested"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
