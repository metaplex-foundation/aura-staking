use crate::error::*;
use anchor_lang::prelude::*;

/// Seconds in one day.
pub const SECS_PER_DAY: u64 = 86_400;

/// Seconds in one month.
pub const SECS_PER_MONTH: u64 = 365 * SECS_PER_DAY / 12;

/// Seconds in cooldown (5 days)
pub const COOLDOWN_SECS: u64 = 86_400 * 5;

#[zero_copy]
#[derive(Default)]
pub struct Lockup {
    /// Note, that if start_ts is in the future, the funds are nevertheless
    /// locked up!

    /// Start of the lockup.
    pub start_ts: u64,

    /// End of the lockup.
    pub end_ts: u64,

    /// End of the cooldown.
    pub cooldown_ends_at: u64,

    pub cooldown_requested: bool,
    /// Type of lockup.
    pub kind: LockupKind,

    /// Type of lockup
    pub period: LockupPeriod,

    /// Padding after period to align the struct size to 8 bytes
    pub _reserved1: [u8; 5],
}
const_assert!(std::mem::size_of::<Lockup>() == 3 * 8 + 1 + 1 + 1 + 5);
const_assert!(std::mem::size_of::<Lockup>() % 8 == 0);

impl Lockup {
    /// Create lockup for a given period
    pub fn new(kind: LockupKind, start_ts: u64, period: LockupPeriod) -> Result<Self> {
        require!(
            (kind == LockupKind::None && period == LockupPeriod::None)
                || (kind == LockupKind::Constant && period != LockupPeriod::None),
            VsrError::InvalidLockupKind
        );

        let end_ts = start_ts
            .checked_add(period.to_secs())
            .ok_or(VsrError::InvalidTimestampArguments)?;

        Ok(Self {
            kind,
            start_ts,
            end_ts,
            period,
            // 0 means cooldown hasn't been requested
            cooldown_ends_at: 0,
            cooldown_requested: false,
            _reserved1: [0; 5],
        })
    }

    /// True when the lockup is finished.
    pub fn expired(&self, curr_ts: u64) -> bool {
        self.seconds_left(curr_ts) == 0
    }

    /// Number of seconds left in the lockup.
    /// May be more than end_ts-start_ts if curr_ts < start_ts.
    pub fn seconds_left(&self, curr_ts: u64) -> u64 {
        // if self.kind == LockupKind::Constant{
        //     curr_ts = self.start_ts;
        // };
        if self.kind == LockupKind::None {
            return 0;
        }

        if curr_ts >= self.end_ts {
            0
        } else {
            self.end_ts - curr_ts
        }
    }

    /// Returns the number of periods left on the lockup.
    /// Returns 0 after lockup has expired and periods_total before start_ts.
    pub fn periods_left(&self, curr_ts: u64) -> Result<u64> {
        let period_secs = self.kind.period_secs();
        if period_secs == 0 {
            return Ok(0);
        }
        if curr_ts < self.start_ts {
            return self.periods_total();
        }
        Ok(self
            .seconds_left(curr_ts)
            .checked_add(period_secs.saturating_sub(1))
            .unwrap()
            .checked_div(period_secs)
            .unwrap())
    }

    /// Returns the current period in the vesting schedule.
    /// Will report periods_total() after lockup has expired and 0 before start_ts.
    pub fn period_current(&self, curr_ts: u64) -> Result<u64> {
        Ok(self
            .periods_total()?
            .saturating_sub(self.periods_left(curr_ts)?))
    }

    /// Returns the total amount of periods in the lockup.
    pub fn periods_total(&self) -> Result<u64> {
        let period_secs = self.kind.period_secs();
        if period_secs == 0 {
            return Ok(0);
        }

        let lockup_secs = self.seconds_left(self.start_ts);
        require_eq!(lockup_secs % period_secs, 0, VsrError::InvalidLockupPeriod);

        Ok(lockup_secs.checked_div(period_secs).unwrap())
    }

    /// Remove the vesting periods that are now in the past.
    pub fn remove_past_periods(&mut self, curr_ts: u64) -> Result<()> {
        let periods = self.period_current(curr_ts)?;
        let period_secs = self.kind.period_secs();
        self.start_ts = self
            .start_ts
            .checked_add(
                periods
                    .checked_mul(period_secs)
                    .ok_or(VsrError::InvalidTimestampArguments)?,
            )
            .unwrap();
        require_gte!(self.end_ts, self.start_ts, VsrError::InternalProgramError);
        require_eq!(
            self.period_current(curr_ts)?,
            0,
            VsrError::InternalProgramError
        );
        Ok(())
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockupPeriod {
    None,
    ThreeMonths,
    SixMonths,
    OneYear,
    Flex,
}

impl Default for LockupPeriod {
    fn default() -> Self {
        Self::Flex
    }
}

impl LockupPeriod {
    pub fn to_secs(&self) -> u64 {
        match self {
            LockupPeriod::ThreeMonths => SECS_PER_MONTH * 3,
            LockupPeriod::SixMonths => SECS_PER_MONTH * 6,
            LockupPeriod::OneYear => SECS_PER_MONTH * 12,
            LockupPeriod::Flex => SECS_PER_DAY * 5,
            LockupPeriod::None => 0,
        }
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum LockupKind {
    /// No lockup, tokens can be withdrawn as long as not engaged in a proposal.
    None,
    /// Lock up permanently. The number of days specified becomes the minimum
    /// unlock period when the deposit (or a part of it) is changed to None.
    Constant,
}

impl Default for LockupKind {
    fn default() -> Self {
        Self::Constant
    }
}

impl LockupKind {
    /// The lockup length is specified by passing the number of lockup periods
    /// to create_deposit_entry. This describes a period's length.
    ///
    /// For vesting lockups, the period length is also the vesting period.
    pub fn period_secs(&self) -> u64 {
        match self {
            LockupKind::None => 0,
            LockupKind::Constant => SECS_PER_DAY, // arbitrary choice
        }
    }

    /// Lockups cannot decrease in strictness
    pub fn strictness(&self) -> u8 {
        match self {
            LockupKind::None => 0,
            LockupKind::Constant => 3,
        }
    }
}
