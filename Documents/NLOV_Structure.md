# Neurolov ($NLOV)  Tokenomics Structure

## 1. Token Fundamentals
```typescript
interface TokenDetails {
    name: "NEUROLOV",
    symbol: "NLOV",
    type: "SPL Token (Solana)",
    totalSupply: "500,000,000",
    decimals: 9,
    initialMarketCap: "$2.76M",
    fullyDilutedValuation: "$35M",
    initialCirculatingSupply: "39.5M (7.9%)"
}
```

## 2. Complete Allocation Breakdown

| Category | Allocation | Tokens | TGE Unlock | Vesting/Cliff | Initial Unlock | Lock/Vesting Period |
|----------|------------|--------|------------|---------------|----------------|-------------------|
| Seed Sale | 2% | 10M | 5% | 3m cliff | 500K | 12m linear |
| Private Sale | 5% | 25M | 8% | 2m cliff | 2M | 10m linear |
| Presale | 4% | 20M | 10% | None | 2M | 8m linear |
| Public Sale | 7% | 35M | 15% | None | 5.25M | 6m linear |
| Team | 15% | 75M | 0% | 12m cliff | 0 | 36m linear + milestones |
| Treasury | 12% | 60M | 10% | 3m cliff | 6M | 36m linear |
| Development | 5% | 25M | 10% | 2m cliff | 2.5M | 24m linear |
| Advisors | 3% | 15M | 0% | 6m cliff | 0 | 24m linear |
| Ecosystem | 15% | 75M | 5% | 1m cliff | 3.75M | 48m linear |
| Node Rewards | 8% | 40M | 5% | Dynamic | 2M | Performance-based |
| Liquidity | 8% | 40M | 30% | None | 12M | 24m linear |
| Partnerships | 5% | 25M | 0% | 3m cliff | 0 | 36m case-by-case |
| Marketing | 3% | 15M | 10% | None | 1.5M | 24m linear |
| Community | 8% | 40M | 5% | None | 2M | 48m linear |

## 3. Monthly Emission Schedule

```typescript
interface MonthlyEmission {
    // First 6 months (in millions NLOV)
    month1: {
        newUnlocks: 3.5,
        totalCirculating: 43.0,
        percentCirculating: "8.6%"
    },
    month2: {
        newUnlocks: 4.2,
        totalCirculating: 47.2,
        percentCirculating: "9.4%"
    },
    month3: {
        newUnlocks: 5.8,
        totalCirculating: 53.0,
        percentCirculating: "10.6%"
    },
    month4: {
        newUnlocks: 8.5,
        totalCirculating: 61.5,
        percentCirculating: "12.3%"
    },
    month5: {
        newUnlocks: 10.2,
        totalCirculating: 71.7,
        percentCirculating: "14.3%"
    },
    month6: {
        newUnlocks: 12.5,
        totalCirculating: 84.2,
        percentCirculating: "16.8%"
    }
}
```

## 4. Unlock Triggers & Milestones

```typescript
interface UnlockTriggers {
    team: {
        milestone1: {
            trigger: "100,000 active users",
            unlock: "25% of allocation",
            expectedTime: "Month 12"
        },
        milestone2: {
            trigger: "$10M platform revenue",
            unlock: "25% of allocation",
            expectedTime: "Month 18"
        },
        milestone3: {
            trigger: "Mainnet launch",
            unlock: "25% of allocation",
            expectedTime: "Month 24"
        },
        milestone4: {
            trigger: "Time-based completion",
            unlock: "25% of allocation",
            time: "Month 36"
        }
    },
    
    ecosystem: {
        development: {
            trigger: "Platform feature completions",
            unlock: "10% per major milestone",
            frequency: "Quarterly review"
        },
        adoption: {
            trigger: "User growth targets",
            unlock: "5% per 50,000 users",
            cap: "Monthly max 2%"
        }
    }
}
```

## 5. Node Reward Distribution Mechanism

```typescript
interface NodeRewardStructure {
    monthlyAllocation: {
        validatorRewards: "15% of monthly unlock",
        enterpriseNodes: "35% of monthly unlock",
        professionalNodes: "25% of monthly unlock",
        consumerNodes: "15% of monthly unlock",
        mobileNodes: "10% of monthly unlock"
    },

    rewardCalculation: {
        baseFormula: "Stake * Time * Performance * Type Multiplier",
        typeMultipliers: {
            validator: 1.5,
            enterprise: 1.3,
            professional: 1.2,
            consumer: 1.0,
            mobile: 0.8
        },
        performanceScoring: {
            uptime: "40%",
            taskCompletion: "30%",
            userRating: "20%",
            networkContribution: "10%"
        }
    },

    distributionFrequency: {
        validators: "Daily",
        enterprise: "Weekly",
        others: "Monthly"
    },

    bonusStructure: {
        consistencyBonus: "5% extra for 100% uptime",
        volumeBonus: "10% extra for high utilization",
        loyaltyBonus: "2% increase per month, cap 20%"
    }
}
```

## 6. Supply Growth & Circulating Supply Chart

```mermaid
gantt
    title Token Unlocks & Circulating Supply Growth
    dateFormat YYYY-MM
    section Initial
    TGE (7.9%)             :milestone, 2024-12
    section Month 3
    Sales Complete (15%)    :milestone, 2025-03
    section Month 6
    Public Vesting (25%)    :milestone, 2025-06
    section Month 12
    Team Cliff End (35%)    :milestone, 2025-12
    section Month 24
    Mid Point (60%)         :milestone, 2026-12
    section Month 36
    Team Complete (80%)     :milestone, 2027-12
    section Month 48
    Full Unlock (100%)      :milestone, 2028-12
```

## 7. Price Protection & Stability

```typescript
interface PriceStability {
    initialProtection: {
        tradingLimits: {
            day1: "5% of holdings per transaction",
            week1: "15% of holdings per day",
            month1: "30% of holdings per week"
        },
        stakingIncentives: {
            apy: "25-35% first month",
            lockBonus: "50% extra rewards",
            earlyStaking: "2x multiplier"
        }
    },

    ongoingStability: {
        buyback: "10% of platform revenue",
        burn: "2% of all fees",
        liquidityManagement: {
            minimum: "$2M per DEX",
            rebalancing: "Every 6 hours",
            depth: "$100K per 1% price movement"
        }
    }
}
```

## 8. Treasury & Fund Usage

```typescript
interface TreasuryManagement {
    allocation: {
        development: "40% of raised funds",
        marketing: "25% of raised funds",
        operations: "15% of raised funds",
        reserves: "10% of raised funds",
        contingency: "10% of raised funds"
    },

    investmentStrategy: {
        stablecoins: "40% of treasury",
        cryptoAssets: "30% of treasury",
        yieldFarming: "20% of treasury",
        strategicInvestments: "10% of treasury"
    }
}
```
