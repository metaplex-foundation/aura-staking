use crate::error::*;
use crate::vote_weight_record;
use anchor_lang::prelude::*;
use std::convert::TryFrom;

// Generate a VoteWeightRecord Anchor wrapper, owned by the current program.
// VoteWeightRecords are unique in that they are defined by the SPL governance
// program, but they are actually owned by this program.
vote_weight_record!(crate::ID);

/// Seconds in one day.
pub const SECS_PER_DAY: u64 = 86_400;

/// Seconds in one month.
pub const SECS_PER_MONTH: u64 = 365 * SECS_PER_DAY / 12;

/// Maximum acceptable number of lockup periods.

/// This setting limits the maximum lockup duration for lockup methods
/// with daily periods to 200 years.
pub const MAX_LOCKUP_PERIODS: u32 = 365 * 200;

pub const MAX_LOCKUP_IN_FUTURE_SECS: i64 = 100 * 365 * 24 * 60 * 60;

/// Seconds in cooldown (5 days)
pub const COOLDOWN_SECS: u64 = SECS_PER_DAY * 5;

#[zero_copy]
#[derive(Default)]
pub struct Lockup {
    /// Start of the lockup.
    ///
    /// Note, that if start_ts is in the future, the funds are nevertheless
    /// locked up!
    ///
    /// Similarly vote power computations don't care about start_ts and always
    /// assume the full interval from now to end_ts.
    pub(crate) start_ts: i64,

    /// End of the lockup.
    pub(crate) end_ts: i64,

    /// Type of lockup.
    pub kind: LockupKind,

    /// Type of lockup
    pub period: LockupPeriod,

    /// padding + reserved
    pub reserved: [u8; 6],
}
const_assert!(std::mem::size_of::<Lockup>() == 2 * 8 + 1 + 1 + 6);
const_assert!(std::mem::size_of::<Lockup>() % 8 == 0);

impl Lockup {
    /// Create lockup for a given period
    pub fn new(
        kind: LockupKind,
        curr_ts: i64,
        start_ts: i64,
        period: LockupPeriod,
    ) -> Result<Self> {
        require_gt!(
            curr_ts + MAX_LOCKUP_IN_FUTURE_SECS,
            start_ts,
            VsrError::DepositStartTooFarInFuture
        );

        let end_ts = start_ts
            .checked_add(
                i64::try_from(period.to_secs()).map_err(|_| VsrError::InvalidTimestampArguments)?,
            )
            .ok_or(VsrError::InvalidTimestampArguments)?;

        Ok(Self {
            kind,
            start_ts,
            end_ts,
            period,
            reserved: [0; 6],
        })
    }

    /// True when the lockup is finished.
    pub fn expired(&self, curr_ts: i64) -> bool {
        self.seconds_left(curr_ts) == 0
    }

    /// Number of seconds left in the lockup.
    /// May be more than end_ts-start_ts if curr_ts < start_ts.
    pub fn seconds_left(&self, mut curr_ts: i64) -> u64 {
        if self.kind == LockupKind::Constant {
            curr_ts = self.start_ts;
        }
        if curr_ts >= self.end_ts {
            0
        } else {
            (self.end_ts - curr_ts) as u64
        }
    }

    /// Returns the number of periods left on the lockup.
    /// Returns 0 after lockup has expired and periods_total before start_ts.
    pub fn periods_left(&self, curr_ts: i64) -> Result<u64> {
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
    pub fn period_current(&self, curr_ts: i64) -> Result<u64> {
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
    pub fn remove_past_periods(&mut self, curr_ts: i64) -> Result<()> {
        let periods = self.period_current(curr_ts)?;
        let period_secs = self.kind.period_secs();
        self.start_ts = self
            .start_ts
            .checked_add(i64::try_from(periods.checked_mul(period_secs).unwrap()).unwrap())
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

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum LockupPeriod {
    TwoWeeks,
    TwoMonths,
    TwoYears,
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
            LockupPeriod::TwoWeeks => SECS_PER_DAY * 14,
            LockupPeriod::TwoMonths => SECS_PER_MONTH * 2,
            LockupPeriod::TwoYears => SECS_PER_MONTH * 12,
            LockupPeriod::Flex => u64::MAX,
        }
    }

    pub fn get_coef(&self) -> f64 {
        match self {
            LockupPeriod::TwoWeeks => 1.05,
            LockupPeriod::TwoMonths => 1.1,
            LockupPeriod::TwoYears => 1.3,
            LockupPeriod::Flex => 1.01,
        }
    }
}

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

    pub fn is_vesting(&self) -> bool {
        match self {
            LockupKind::None => false,
            LockupKind::Constant => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // intentionally not a multiple of a day
    const MAX_SECS_LOCKED: u64 = 365 * 24 * 60 * 60 + 7 * 60 * 60;
    const MAX_DAYS_LOCKED: f64 = MAX_SECS_LOCKED as f64 / (24.0 * 60.0 * 60.0);

    #[test]
    pub fn period_computations() -> Result<()> {
        let period = LockupPeriod::TwoWeeks;
        let lockup = Lockup::new(LockupKind::Constant, 1000, 1000, period)?;
        let day = SECS_PER_DAY as i64;
        assert_eq!(lockup.periods_total()?, 3);
        assert_eq!(lockup.period_current(0)?, 0);
        assert_eq!(lockup.periods_left(0)?, 3);
        assert_eq!(lockup.period_current(999)?, 0);
        assert_eq!(lockup.periods_left(999)?, 3);
        assert_eq!(lockup.period_current(1000)?, 0);
        assert_eq!(lockup.periods_left(1000)?, 3);
        assert_eq!(lockup.period_current(1000 + day - 1)?, 0);
        assert_eq!(lockup.periods_left(1000 + day - 1)?, 3);
        assert_eq!(lockup.period_current(1000 + day)?, 1);
        assert_eq!(lockup.periods_left(1000 + day)?, 2);
        assert_eq!(lockup.period_current(1000 + 3 * day - 1)?, 2);
        assert_eq!(lockup.periods_left(1000 + 3 * day - 1)?, 1);
        assert_eq!(lockup.period_current(1000 + 3 * day)?, 3);
        assert_eq!(lockup.periods_left(1000 + 3 * day)?, 0);
        assert_eq!(lockup.period_current(100 * day)?, 3);
        assert_eq!(lockup.periods_left(100 * day)?, 0);
        Ok(())
    }

    #[test]
    pub fn days_left_start() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 10,
            days_total: 10.0,
            curr_day: 0.0,
        })
    }

    #[test]
    pub fn days_left_one_half() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 10,
            days_total: 10.0,
            curr_day: 0.5,
        })
    }

    #[test]
    pub fn days_left_one() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 9,
            days_total: 10.0,
            curr_day: 1.0,
        })
    }

    #[test]
    pub fn days_left_one_and_one_half() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 9,
            days_total: 10.0,
            curr_day: 1.5,
        })
    }

    #[test]
    pub fn days_left_9() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 1,
            days_total: 10.0,
            curr_day: 9.0,
        })
    }

    #[test]
    pub fn days_left_9_dot_one() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 1,
            days_total: 10.0,
            curr_day: 9.1,
        })
    }

    #[test]
    pub fn days_left_9_dot_nine() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 1,
            days_total: 10.0,
            curr_day: 9.9,
        })
    }

    #[test]
    pub fn days_left_ten() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 0,
            days_total: 10.0,
            curr_day: 10.0,
        })
    }

    #[test]
    pub fn days_left_eleven() -> Result<()> {
        run_test_days_left(TestDaysLeft {
            expected_days_left: 0,
            days_total: 10.0,
            curr_day: 11.0,
        })
    }

    #[test]
    pub fn months_left_start() -> Result<()> {
        run_test_months_left(TestMonthsLeft {
            expected_months_left: 10,
            months_total: 10.0,
            curr_month: 0.,
        })
    }

    #[test]
    pub fn months_left_one_half() -> Result<()> {
        run_test_months_left(TestMonthsLeft {
            expected_months_left: 10,
            months_total: 10.0,
            curr_month: 0.5,
        })
    }

    #[test]
    pub fn months_left_one_and_a_half() -> Result<()> {
        run_test_months_left(TestMonthsLeft {
            expected_months_left: 9,
            months_total: 10.0,
            curr_month: 1.5,
        })
    }

    #[test]
    pub fn months_left_ten() -> Result<()> {
        run_test_months_left(TestMonthsLeft {
            expected_months_left: 9,
            months_total: 10.0,
            curr_month: 1.5,
        })
    }

    #[test]
    pub fn months_left_eleven() -> Result<()> {
        run_test_months_left(TestMonthsLeft {
            expected_months_left: 0,
            months_total: 10.0,
            curr_month: 11.,
        })
    }

    struct TestDaysLeft {
        expected_days_left: u64,
        days_total: f64,
        curr_day: f64,
    }

    struct TestMonthsLeft {
        expected_months_left: u64,
        months_total: f64,
        curr_month: f64,
    }

    fn run_test_days_left(t: TestDaysLeft) -> Result<()> {
        let start_ts = 1634929833;
        let end_ts = start_ts + days_to_secs(t.days_total);
        let curr_ts = start_ts + days_to_secs(t.curr_day);
        let l = Lockup {
            kind: LockupKind::Constant,
            start_ts,
            end_ts,
            period: LockupPeriod::TwoWeeks,
            reserved: [0u8; 6],
        };
        let days_left = l.periods_left(curr_ts)?;
        assert_eq!(days_left, 0);
        assert_eq!(days_left, t.expected_days_left);
        Ok(())
    }

    fn run_test_months_left(t: TestMonthsLeft) -> Result<()> {
        let start_ts = 1634929833;
        let end_ts = start_ts + months_to_secs(t.months_total);
        let curr_ts = start_ts + months_to_secs(t.curr_month);
        let l = Lockup {
            kind: LockupKind::Constant,
            start_ts,
            end_ts,
            period: LockupPeriod::TwoWeeks,
            reserved: [0u8; 6],
        };
        let months_left = l.periods_left(curr_ts)?;
        assert_eq!(months_left, t.expected_months_left);
        Ok(())
    }

    fn days_to_secs(days: f64) -> i64 {
        let d = (SECS_PER_DAY as f64) * days;
        d.round() as i64
    }

    fn months_to_secs(months: f64) -> i64 {
        let d = (SECS_PER_MONTH as f64) * months;
        d.round() as i64
    }
}
