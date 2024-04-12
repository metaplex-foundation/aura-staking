export type VoterStakeRegistry = {
  "version": "0.2.4",
  "name": "voter_stake_registry",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "registrarBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "configureVotingMint",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "idx",
          "type": "u16"
        },
        {
          "name": "digitShift",
          "type": "i8"
        },
        {
          "name": "baselineVoteWeightScaledFactor",
          "type": "u64"
        },
        {
          "name": "maxExtraLockupVoteWeightScaledFactor",
          "type": "u64"
        },
        {
          "name": "lockupSaturationSecs",
          "type": "u64"
        },
        {
          "name": "grantAuthority",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "createVoter",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voterBump",
          "type": "u8"
        },
        {
          "name": "voterWeightRecordBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createDepositEntry",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "depositMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        },
        {
          "name": "kind",
          "type": {
            "defined": "LockupKind"
          }
        },
        {
          "name": "startTs",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "period",
          "type": {
            "defined": "LockupPeriod"
          }
        }
      ]
    },
    {
      "name": "deposit",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdraw",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenOwnerRecord",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destination",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "closeDepositEntry",
      "accounts": [
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unlockTokens",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "closeVoter",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "solDestination",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "logVoterInfo",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryBegin",
          "type": "u8"
        },
        {
          "name": "depositEntryCount",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setTimeOffset",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "timeOffset",
          "type": "i64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "registrar",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "type": "publicKey"
          },
          {
            "name": "realm",
            "type": "publicKey"
          },
          {
            "name": "realmGoverningTokenMint",
            "type": "publicKey"
          },
          {
            "name": "realmAuthority",
            "type": "publicKey"
          },
          {
            "name": "votingMints",
            "type": {
              "array": [
                {
                  "defined": "VotingMintConfig"
                },
                4
              ]
            }
          },
          {
            "name": "timeOffset",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "voter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voterAuthority",
            "type": "publicKey"
          },
          {
            "name": "registrar",
            "type": "publicKey"
          },
          {
            "name": "deposits",
            "type": {
              "array": [
                {
                  "defined": "DepositEntry"
                },
                32
              ]
            }
          },
          {
            "name": "voterBump",
            "type": "u8"
          },
          {
            "name": "voterWeightRecordBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "DepositEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lockup",
            "type": {
              "defined": "Lockup"
            }
          },
          {
            "name": "amountDepositedNative",
            "type": "u64"
          },
          {
            "name": "isUsed",
            "type": "bool"
          },
          {
            "name": "votingMintConfigIdx",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "VestingInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "nextTimestamp",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "LockingInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "endTimestamp",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "vesting",
            "type": {
              "option": {
                "defined": "VestingInfo"
              }
            }
          }
        ]
      }
    },
    {
      "name": "Lockup",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "endTs",
            "type": "i64"
          },
          {
            "name": "cooldownEndsTs",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "kind",
            "type": {
              "defined": "LockupKind"
            }
          },
          {
            "name": "period",
            "type": {
              "defined": "LockupPeriod"
            }
          }
        ]
      }
    },
    {
      "name": "VotingMintConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "grantAuthority",
            "type": "publicKey"
          },
          {
            "name": "baselineVoteWeightScaledFactor",
            "type": "u64"
          },
          {
            "name": "maxExtraLockupVoteWeightScaledFactor",
            "type": "u64"
          },
          {
            "name": "lockupSaturationSecs",
            "type": "u64"
          },
          {
            "name": "digitShift",
            "type": "i8"
          }
        ]
      }
    },
    {
      "name": "VsrError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidRate"
          },
          {
            "name": "RatesFull"
          },
          {
            "name": "VotingMintNotFound"
          },
          {
            "name": "DepositEntryNotFound"
          },
          {
            "name": "DepositEntryFull"
          },
          {
            "name": "VotingTokenNonZero"
          },
          {
            "name": "OutOfBoundsDepositEntryIndex"
          },
          {
            "name": "UnusedDepositEntryIndex"
          },
          {
            "name": "InsufficientUnlockedTokens"
          },
          {
            "name": "UnableToConvert"
          },
          {
            "name": "InvalidLockupPeriod"
          },
          {
            "name": "InvalidEndTs"
          },
          {
            "name": "InvalidDays"
          },
          {
            "name": "VotingMintConfigIndexAlreadyInUse"
          },
          {
            "name": "OutOfBoundsVotingMintConfigIndex"
          },
          {
            "name": "InvalidDecimals"
          },
          {
            "name": "InvalidToDepositAndWithdrawInOneSlot"
          },
          {
            "name": "ShouldBeTheFirstIxInATx"
          },
          {
            "name": "ForbiddenCpi"
          },
          {
            "name": "InvalidMint"
          },
          {
            "name": "DebugInstruction"
          },
          {
            "name": "ClawbackNotAllowedOnDeposit"
          },
          {
            "name": "DepositStillLocked"
          },
          {
            "name": "InvalidAuthority"
          },
          {
            "name": "InvalidTokenOwnerRecord"
          },
          {
            "name": "InvalidRealmAuthority"
          },
          {
            "name": "VoterWeightOverflow"
          },
          {
            "name": "LockupSaturationMustBePositive"
          },
          {
            "name": "VotingMintConfiguredWithDifferentIndex"
          },
          {
            "name": "InternalProgramError"
          },
          {
            "name": "InsufficientLockedTokens"
          },
          {
            "name": "MustKeepTokensLocked"
          },
          {
            "name": "InvalidLockupKind"
          },
          {
            "name": "InvalidChangeToClawbackDepositEntry"
          },
          {
            "name": "InternalErrorBadLockupVoteWeight"
          },
          {
            "name": "DepositStartTooFarInFuture"
          },
          {
            "name": "VaultTokenNonZero"
          },
          {
            "name": "InvalidTimestampArguments"
          },
          {
            "name": "UnlockMustBeCalledFirst"
          },
          {
            "name": "UnlockAlreadyRequested"
          }
        ]
      }
    },
    {
      "name": "LockupPeriod",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "ThreeMonths"
          },
          {
            "name": "SixMonths"
          },
          {
            "name": "OneYear"
          },
          {
            "name": "Flex"
          }
        ]
      }
    },
    {
      "name": "LockupKind",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Constant"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "VoterInfo",
      "fields": [
        {
          "name": "votingPower",
          "type": "u64",
          "index": false
        },
        {
          "name": "votingPowerBaseline",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "DepositEntryInfo",
      "fields": [
        {
          "name": "depositEntryIndex",
          "type": "u8",
          "index": false
        },
        {
          "name": "votingMintConfigIndex",
          "type": "u8",
          "index": false
        },
        {
          "name": "unlocked",
          "type": "u64",
          "index": false
        },
        {
          "name": "votingPower",
          "type": "u64",
          "index": false
        },
        {
          "name": "votingPowerBaseline",
          "type": "u64",
          "index": false
        },
        {
          "name": "locking",
          "type": {
            "option": {
              "defined": "LockingInfo"
            }
          },
          "index": false
        }
      ]
    }
  ]
};

export const IDL: VoterStakeRegistry = {
  "version": "0.2.4",
  "name": "voter_stake_registry",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "registrarBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "configureVotingMint",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "idx",
          "type": "u16"
        },
        {
          "name": "digitShift",
          "type": "i8"
        },
        {
          "name": "baselineVoteWeightScaledFactor",
          "type": "u64"
        },
        {
          "name": "maxExtraLockupVoteWeightScaledFactor",
          "type": "u64"
        },
        {
          "name": "lockupSaturationSecs",
          "type": "u64"
        },
        {
          "name": "grantAuthority",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "createVoter",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "instructions",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voterBump",
          "type": "u8"
        },
        {
          "name": "voterWeightRecordBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createDepositEntry",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "depositMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        },
        {
          "name": "kind",
          "type": {
            "defined": "LockupKind"
          }
        },
        {
          "name": "startTs",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "period",
          "type": {
            "defined": "LockupPeriod"
          }
        }
      ]
    },
    {
      "name": "deposit",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdraw",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenOwnerRecord",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "destination",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "closeDepositEntry",
      "accounts": [
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unlockTokens",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "depositEntryIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "closeVoter",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voterAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "solDestination",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "logVoterInfo",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "depositEntryBegin",
          "type": "u8"
        },
        {
          "name": "depositEntryCount",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setTimeOffset",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "timeOffset",
          "type": "i64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "registrar",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "type": "publicKey"
          },
          {
            "name": "realm",
            "type": "publicKey"
          },
          {
            "name": "realmGoverningTokenMint",
            "type": "publicKey"
          },
          {
            "name": "realmAuthority",
            "type": "publicKey"
          },
          {
            "name": "votingMints",
            "type": {
              "array": [
                {
                  "defined": "VotingMintConfig"
                },
                4
              ]
            }
          },
          {
            "name": "timeOffset",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "voter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voterAuthority",
            "type": "publicKey"
          },
          {
            "name": "registrar",
            "type": "publicKey"
          },
          {
            "name": "deposits",
            "type": {
              "array": [
                {
                  "defined": "DepositEntry"
                },
                32
              ]
            }
          },
          {
            "name": "voterBump",
            "type": "u8"
          },
          {
            "name": "voterWeightRecordBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "DepositEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lockup",
            "type": {
              "defined": "Lockup"
            }
          },
          {
            "name": "amountDepositedNative",
            "type": "u64"
          },
          {
            "name": "isUsed",
            "type": "bool"
          },
          {
            "name": "votingMintConfigIdx",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "VestingInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "nextTimestamp",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "LockingInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "endTimestamp",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "vesting",
            "type": {
              "option": {
                "defined": "VestingInfo"
              }
            }
          }
        ]
      }
    },
    {
      "name": "Lockup",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "endTs",
            "type": "i64"
          },
          {
            "name": "cooldownEndsTs",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "kind",
            "type": {
              "defined": "LockupKind"
            }
          },
          {
            "name": "period",
            "type": {
              "defined": "LockupPeriod"
            }
          }
        ]
      }
    },
    {
      "name": "VotingMintConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "grantAuthority",
            "type": "publicKey"
          },
          {
            "name": "baselineVoteWeightScaledFactor",
            "type": "u64"
          },
          {
            "name": "maxExtraLockupVoteWeightScaledFactor",
            "type": "u64"
          },
          {
            "name": "lockupSaturationSecs",
            "type": "u64"
          },
          {
            "name": "digitShift",
            "type": "i8"
          }
        ]
      }
    },
    {
      "name": "VsrError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidRate"
          },
          {
            "name": "RatesFull"
          },
          {
            "name": "VotingMintNotFound"
          },
          {
            "name": "DepositEntryNotFound"
          },
          {
            "name": "DepositEntryFull"
          },
          {
            "name": "VotingTokenNonZero"
          },
          {
            "name": "OutOfBoundsDepositEntryIndex"
          },
          {
            "name": "UnusedDepositEntryIndex"
          },
          {
            "name": "InsufficientUnlockedTokens"
          },
          {
            "name": "UnableToConvert"
          },
          {
            "name": "InvalidLockupPeriod"
          },
          {
            "name": "InvalidEndTs"
          },
          {
            "name": "InvalidDays"
          },
          {
            "name": "VotingMintConfigIndexAlreadyInUse"
          },
          {
            "name": "OutOfBoundsVotingMintConfigIndex"
          },
          {
            "name": "InvalidDecimals"
          },
          {
            "name": "InvalidToDepositAndWithdrawInOneSlot"
          },
          {
            "name": "ShouldBeTheFirstIxInATx"
          },
          {
            "name": "ForbiddenCpi"
          },
          {
            "name": "InvalidMint"
          },
          {
            "name": "DebugInstruction"
          },
          {
            "name": "ClawbackNotAllowedOnDeposit"
          },
          {
            "name": "DepositStillLocked"
          },
          {
            "name": "InvalidAuthority"
          },
          {
            "name": "InvalidTokenOwnerRecord"
          },
          {
            "name": "InvalidRealmAuthority"
          },
          {
            "name": "VoterWeightOverflow"
          },
          {
            "name": "LockupSaturationMustBePositive"
          },
          {
            "name": "VotingMintConfiguredWithDifferentIndex"
          },
          {
            "name": "InternalProgramError"
          },
          {
            "name": "InsufficientLockedTokens"
          },
          {
            "name": "MustKeepTokensLocked"
          },
          {
            "name": "InvalidLockupKind"
          },
          {
            "name": "InvalidChangeToClawbackDepositEntry"
          },
          {
            "name": "InternalErrorBadLockupVoteWeight"
          },
          {
            "name": "DepositStartTooFarInFuture"
          },
          {
            "name": "VaultTokenNonZero"
          },
          {
            "name": "InvalidTimestampArguments"
          },
          {
            "name": "UnlockMustBeCalledFirst"
          },
          {
            "name": "UnlockAlreadyRequested"
          }
        ]
      }
    },
    {
      "name": "LockupPeriod",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "ThreeMonths"
          },
          {
            "name": "SixMonths"
          },
          {
            "name": "OneYear"
          },
          {
            "name": "Flex"
          }
        ]
      }
    },
    {
      "name": "LockupKind",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "Constant"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "VoterInfo",
      "fields": [
        {
          "name": "votingPower",
          "type": "u64",
          "index": false
        },
        {
          "name": "votingPowerBaseline",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "DepositEntryInfo",
      "fields": [
        {
          "name": "depositEntryIndex",
          "type": "u8",
          "index": false
        },
        {
          "name": "votingMintConfigIndex",
          "type": "u8",
          "index": false
        },
        {
          "name": "unlocked",
          "type": "u64",
          "index": false
        },
        {
          "name": "votingPower",
          "type": "u64",
          "index": false
        },
        {
          "name": "votingPowerBaseline",
          "type": "u64",
          "index": false
        },
        {
          "name": "locking",
          "type": {
            "option": {
              "defined": "LockingInfo"
            }
          },
          "index": false
        }
      ]
    }
  ]
};
