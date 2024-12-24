/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/clmm_trading_new.json`.
 */
export type ClmmTradingNew = {
  "address": "devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH",
  "metadata": {
    "name": "clmmTradingNew",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "swapV2",
      "discriminator": [
        43,
        4,
        237,
        11,
        26,
        201,
        30,
        98
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "ammConfig"
        },
        {
          "name": "poolState",
          "writable": true
        },
        {
          "name": "inputTokenAccount",
          "writable": true
        },
        {
          "name": "outputTokenAccount",
          "writable": true
        },
        {
          "name": "inputVault",
          "writable": true
        },
        {
          "name": "outputVault",
          "writable": true
        },
        {
          "name": "observationState"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenProgram2022",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "memoProgram"
        },
        {
          "name": "inputVaultMint"
        },
        {
          "name": "outputVaultMint"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "swapV2Params"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "poolState",
      "discriminator": [
        247,
        237,
        227,
        245,
        215,
        195,
        222,
        70
      ]
    }
  ],
  "events": [
    {
      "name": "swapEvent",
      "discriminator": [
        64,
        198,
        205,
        232,
        38,
        8,
        113,
        226
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "mathOverflow",
      "msg": "Math operation overflowed"
    },
    {
      "code": 6001,
      "name": "invalidPoolState",
      "msg": "Invalid pool state"
    },
    {
      "code": 6002,
      "name": "poolPaused",
      "msg": "Pool is paused"
    },
    {
      "code": 6003,
      "name": "invalidTickSpacing",
      "msg": "Invalid tick spacing"
    },
    {
      "code": 6004,
      "name": "invalidSqrtPrice",
      "msg": "Invalid sqrt price"
    },
    {
      "code": 6005,
      "name": "invalidTickRange",
      "msg": "Invalid tick range"
    },
    {
      "code": 6006,
      "name": "insufficientLiquidity",
      "msg": "Insufficient liquidity"
    },
    {
      "code": 6007,
      "name": "liquidityOverflow",
      "msg": "Liquidity overflow"
    },
    {
      "code": 6008,
      "name": "insufficientInput",
      "msg": "Insufficient input amount"
    },
    {
      "code": 6009,
      "name": "excessivePriceImpact",
      "msg": "Excessive price impact"
    },
    {
      "code": 6010,
      "name": "slippageExceeded",
      "msg": "Slippage tolerance exceeded"
    },
    {
      "code": 6011,
      "name": "invalidFeeRate",
      "msg": "Invalid fee rate"
    },
    {
      "code": 6012,
      "name": "feeOverflow",
      "msg": "Fee calculation overflow"
    },
    {
      "code": 6013,
      "name": "invalidTokenAccountOwner",
      "msg": "Invalid token account owner"
    },
    {
      "code": 6014,
      "name": "invalidTokenMint",
      "msg": "Invalid token mint"
    },
    {
      "code": 6015,
      "name": "invalidAuthority",
      "msg": "Invalid authority"
    },
    {
      "code": 6016,
      "name": "maxTickIndexExceeded",
      "msg": "Maximum tick index exceeded"
    },
    {
      "code": 6017,
      "name": "minTickIndexExceeded",
      "msg": "Minimum tick index exceeded"
    },
    {
      "code": 6018,
      "name": "invalidPosition",
      "msg": "Invalid position"
    },
    {
      "code": 6019,
      "name": "positionNotFound",
      "msg": "Position not found"
    },
    {
      "code": 6020,
      "name": "positionUpdateFailed",
      "msg": "Position update failed"
    },
    {
      "code": 6021,
      "name": "observationStateInvalid",
      "msg": "Observation state invalid"
    },
    {
      "code": 6022,
      "name": "tickArrayInvalid",
      "msg": "Tick array invalid"
    },
    {
      "code": 6023,
      "name": "priceLimitReached",
      "msg": "Price limit reached"
    },
    {
      "code": 6024,
      "name": "zeroLiquidity",
      "msg": "Zero liquidity"
    },
    {
      "code": 6025,
      "name": "insufficientTokenBalance",
      "msg": "Token account balance insufficient"
    },
    {
      "code": 6026,
      "name": "poolIsPaused",
      "msg": "Pool is Paused"
    }
  ],
  "types": [
    {
      "name": "poolState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "tokenMint0",
            "type": "pubkey"
          },
          {
            "name": "tokenMint1",
            "type": "pubkey"
          },
          {
            "name": "tickSpacing",
            "type": "i32"
          },
          {
            "name": "tickSpacingSeed",
            "type": "u16"
          },
          {
            "name": "feeRate",
            "type": "u32"
          },
          {
            "name": "liquidity",
            "type": "u128"
          },
          {
            "name": "currentSqrtPrice",
            "type": "u128"
          },
          {
            "name": "currentTickIndex",
            "type": "i32"
          },
          {
            "name": "feeGrowthGlobal0",
            "type": "u128"
          },
          {
            "name": "feeGrowthGlobal1",
            "type": "u128"
          },
          {
            "name": "feeProtocolToken0",
            "type": "u64"
          },
          {
            "name": "feeProtocolToken1",
            "type": "u64"
          },
          {
            "name": "tokenVault0",
            "type": "pubkey"
          },
          {
            "name": "tokenVault1",
            "type": "pubkey"
          },
          {
            "name": "observationKey",
            "type": "pubkey"
          },
          {
            "name": "poolId",
            "type": "pubkey"
          },
          {
            "name": "isPaused",
            "type": "bool"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "swapEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolId",
            "type": "pubkey"
          },
          {
            "name": "amountIn",
            "type": "u64"
          },
          {
            "name": "amountOutMin",
            "type": "u64"
          },
          {
            "name": "sqrtPriceLimit",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "swapV2Params",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "otherAmountThreshold",
            "type": "u64"
          },
          {
            "name": "sqrtPriceLimitX64",
            "type": "u128"
          },
          {
            "name": "isBaseInput",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
