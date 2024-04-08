import * as VsrError from "./VsrError"
import * as LockupPeriod from "./LockupPeriod"
import * as LockupKind from "./LockupKind"

export { DepositEntry } from "./DepositEntry"
export type { DepositEntryFields, DepositEntryJSON } from "./DepositEntry"
export { VestingInfo } from "./VestingInfo"
export type { VestingInfoFields, VestingInfoJSON } from "./VestingInfo"
export { LockingInfo } from "./LockingInfo"
export type { LockingInfoFields, LockingInfoJSON } from "./LockingInfo"
export { Lockup } from "./Lockup"
export type { LockupFields, LockupJSON } from "./Lockup"
export { VotingMintConfig } from "./VotingMintConfig"
export type {
  VotingMintConfigFields,
  VotingMintConfigJSON,
} from "./VotingMintConfig"
export { VsrError }

export type VsrErrorKind =
  | VsrError.InvalidRate
  | VsrError.RatesFull
  | VsrError.VotingMintNotFound
  | VsrError.DepositEntryNotFound
  | VsrError.DepositEntryFull
  | VsrError.VotingTokenNonZero
  | VsrError.OutOfBoundsDepositEntryIndex
  | VsrError.UnusedDepositEntryIndex
  | VsrError.InsufficientUnlockedTokens
  | VsrError.UnableToConvert
  | VsrError.InvalidLockupPeriod
  | VsrError.InvalidEndTs
  | VsrError.InvalidDays
  | VsrError.VotingMintConfigIndexAlreadyInUse
  | VsrError.OutOfBoundsVotingMintConfigIndex
  | VsrError.InvalidDecimals
  | VsrError.InvalidToDepositAndWithdrawInOneSlot
  | VsrError.ShouldBeTheFirstIxInATx
  | VsrError.ForbiddenCpi
  | VsrError.InvalidMint
  | VsrError.DebugInstruction
  | VsrError.ClawbackNotAllowedOnDeposit
  | VsrError.DepositStillLocked
  | VsrError.InvalidAuthority
  | VsrError.InvalidTokenOwnerRecord
  | VsrError.InvalidRealmAuthority
  | VsrError.VoterWeightOverflow
  | VsrError.LockupSaturationMustBePositive
  | VsrError.VotingMintConfiguredWithDifferentIndex
  | VsrError.InternalProgramError
  | VsrError.InsufficientLockedTokens
  | VsrError.MustKeepTokensLocked
  | VsrError.InvalidLockupKind
  | VsrError.InvalidChangeToClawbackDepositEntry
  | VsrError.InternalErrorBadLockupVoteWeight
  | VsrError.DepositStartTooFarInFuture
  | VsrError.VaultTokenNonZero
  | VsrError.InvalidTimestampArguments
  | VsrError.UnlockMustBeCalledFirst
  | VsrError.UnlockAlreadyRequested
export type VsrErrorJSON =
  | VsrError.InvalidRateJSON
  | VsrError.RatesFullJSON
  | VsrError.VotingMintNotFoundJSON
  | VsrError.DepositEntryNotFoundJSON
  | VsrError.DepositEntryFullJSON
  | VsrError.VotingTokenNonZeroJSON
  | VsrError.OutOfBoundsDepositEntryIndexJSON
  | VsrError.UnusedDepositEntryIndexJSON
  | VsrError.InsufficientUnlockedTokensJSON
  | VsrError.UnableToConvertJSON
  | VsrError.InvalidLockupPeriodJSON
  | VsrError.InvalidEndTsJSON
  | VsrError.InvalidDaysJSON
  | VsrError.VotingMintConfigIndexAlreadyInUseJSON
  | VsrError.OutOfBoundsVotingMintConfigIndexJSON
  | VsrError.InvalidDecimalsJSON
  | VsrError.InvalidToDepositAndWithdrawInOneSlotJSON
  | VsrError.ShouldBeTheFirstIxInATxJSON
  | VsrError.ForbiddenCpiJSON
  | VsrError.InvalidMintJSON
  | VsrError.DebugInstructionJSON
  | VsrError.ClawbackNotAllowedOnDepositJSON
  | VsrError.DepositStillLockedJSON
  | VsrError.InvalidAuthorityJSON
  | VsrError.InvalidTokenOwnerRecordJSON
  | VsrError.InvalidRealmAuthorityJSON
  | VsrError.VoterWeightOverflowJSON
  | VsrError.LockupSaturationMustBePositiveJSON
  | VsrError.VotingMintConfiguredWithDifferentIndexJSON
  | VsrError.InternalProgramErrorJSON
  | VsrError.InsufficientLockedTokensJSON
  | VsrError.MustKeepTokensLockedJSON
  | VsrError.InvalidLockupKindJSON
  | VsrError.InvalidChangeToClawbackDepositEntryJSON
  | VsrError.InternalErrorBadLockupVoteWeightJSON
  | VsrError.DepositStartTooFarInFutureJSON
  | VsrError.VaultTokenNonZeroJSON
  | VsrError.InvalidTimestampArgumentsJSON
  | VsrError.UnlockMustBeCalledFirstJSON
  | VsrError.UnlockAlreadyRequestedJSON

export { LockupPeriod }

export type LockupPeriodKind =
  | LockupPeriod.None
  | LockupPeriod.ThreeMonths
  | LockupPeriod.SixMonths
  | LockupPeriod.OneYear
  | LockupPeriod.Flex
export type LockupPeriodJSON =
  | LockupPeriod.NoneJSON
  | LockupPeriod.ThreeMonthsJSON
  | LockupPeriod.SixMonthsJSON
  | LockupPeriod.OneYearJSON
  | LockupPeriod.FlexJSON

export { LockupKind }

export type LockupKindKind = LockupKind.None | LockupKind.Constant
export type LockupKindJSON = LockupKind.NoneJSON | LockupKind.ConstantJSON
