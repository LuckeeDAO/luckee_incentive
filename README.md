# Luckee Incentive (激励合约)

基于 CosmWasm 的智能激励合约，专注于奖励分发和用户激励管理。在重构后的三合约架构中，作为核心激励引擎，负责协调其他合约并分发奖励。

## 📋 项目信息

- **版本**：v0.1.0
- **职责**：核心激励引擎
- **权限**：管理员权限控制

## 🎯 核心功能

- **奖励分发**: 根据业务事件分发奖励给用户及其推荐链
- **积分兑换**: 根据用户积分铸造代币
- **规则管理**: 动态管理激励规则配置
- **合约注册**: 注册业务合约并管理权限
- **用户等级**: 管理用户等级和积分系统
- **配置管理**: 更新系统配置参数

## 活动类型支持

| 活动类型 | 描述 | 奖励计算 |
|---------|------|---------|
| BlindBoxOpen | 盲盒开奖 | 基础奖励 × 等级倍数 |
| NftMint | NFT铸造 | 基础奖励 × 等级倍数 |
| Voting | 投票参与 | 基础奖励 × 等级倍数 |
| ReferralReward | 推荐奖励 | 推荐比例 × 被推荐人奖励 |
| SpecialEvent | 特殊活动 | 特殊倍数 × 基础奖励 |

## 用户等级系统

| 等级 | 积分要求 | 奖励倍数 | 描述 |
|------|---------|---------|------|
| Bronze | 0-999 | 1.0x | 青铜等级 |
| Silver | 1000-4999 | 1.2x | 白银等级 |
| Gold | 5000-19999 | 1.5x | 黄金等级 |
| Platinum | 20000+ | 2.0x | 铂金等级 |

## 项目结构

```
luckee_incentive/
├── src/
│   ├── contract.rs           # 合约入口点
│   ├── msg.rs               # 消息定义
│   ├── state.rs             # 状态管理
│   ├── error.rs             # 错误定义
│   └── lib.rs               # 库入口
├── tests/                   # 测试文件
├── docs/                    # 文档
├── scripts/                 # 部署脚本
└── examples/                # 示例代码
```

## 快速开始

### 1. 构建合约

```bash
# 安装依赖
cargo build

# 优化构建
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

# 压缩wasm文件
wasm-opt -Os target/wasm32-unknown-unknown/release/luckee_incentive.wasm -o target/wasm32-unknown-unknown/release/luckee_incentive_optimized.wasm
```

### 2. 运行测试

```bash
cargo test
```

### 3. 部署到测试网

```bash
# 设置环境变量
export ADMIN_ADDRESS="your_admin_address"
export FT_CONTRACT="your_ft_contract_address"

# 运行部署脚本
./scripts/deploy.sh --admin $ADMIN_ADDRESS --ft-contract $FT_CONTRACT
```

## 合约接口

### 实例化消息

```rust
pub struct InstantiateMsg {
    pub admin: Option<String>,              // 管理员地址
    pub config: IncentiveConfig,           // 系统配置
}

pub struct IncentiveConfig {
    pub auto_claim_enabled: bool,          // 自动领取开关
    pub max_rewards_per_user: u32,        // 每用户最大奖励数
    pub reward_expiration_days: u32,       // 奖励过期天数
}
```

### 执行消息

```rust
pub enum ExecuteMsg {
    // 奖励管理
    DistributeReward { 
        user: String, 
        amount: Uint128, 
        activity_type: ActivityType 
    },
    ClaimReward { reward_id: String },
    MintForPoints { user: String, points_amount: Uint128 },
    
    // 规则管理
    CreateRule { rule_id: String, rule: RuleDetails },
    UpdateRule { rule_id: String, rule: RuleDetails },
    DeleteRule { rule_id: String },
    
    // 合约管理
    RegisterContract { contract_type: ContractType, contract_addr: String },
    
    // 用户管理
    UpdateUserLevel { user: String, points: u32 },
    
    // 配置管理
    UpdateConfig { config: IncentiveConfig },
}
```

### 查询消息

```rust
pub enum QueryMsg {
    Config {},                              // 查询配置
    UserRewards { user: String },          // 查询用户奖励
    Rules {},                               // 查询规则
    Contracts {},                           // 查询合约
    UserLevel { user: String },            // 查询用户等级
    SystemStats {},                        // 查询系统统计
}
```

## 使用示例

### 1. 实例化合约

```rust
let msg = InstantiateMsg {
    admin: Some("luckee1admin...".to_string()),
    config: IncentiveConfig {
        auto_claim_enabled: true,
        max_rewards_per_user: 1000,
        reward_expiration_days: 30,
    },
};
```

### 2. 分发奖励

```rust
let msg = ExecuteMsg::DistributeReward {
    user: "luckee1user...".to_string(),
    amount: Uint128::from(1000u128),
    activity_type: ActivityType::BlindBoxOpen {
        box_id: "box1".to_string(),
        nft_kind: "rare".to_string(),
    },
};
```

### 3. 积分兑换

```rust
let msg = ExecuteMsg::MintForPoints {
    user: "luckee1user...".to_string(),
    points_amount: Uint128::from(10000u128),
};
```

### 4. 创建规则

```rust
let msg = ExecuteMsg::CreateRule {
    rule_id: "blind_box_rule".to_string(),
    rule: RuleDetails {
        activity_type: ActivityType::BlindBoxOpen {
            box_id: "box1".to_string(),
            nft_kind: "rare".to_string(),
        },
        base_reward: Uint128::from(1000u128),
        multiplier: Decimal::from_str("1.0").unwrap(),
        enabled: true,
    },
};
```

## 🔗 合约协作

```
业务合约 → LuckeeIncentive (分发奖励) → LuckeeFT (铸造代币)
                ↓
            DD Registry (查询积分)
```

**权限控制**：
- 只有管理员可以管理规则和配置
- 只有注册的合约可以分发奖励
- 用户只能查询自己的信息

## 🚀 快速开始

### 构建
```bash
cargo build --release --target wasm32-unknown-unknown
cosmwasm-opt target/wasm32-unknown-unknown/release/luckee_incentive.wasm
```

### 部署
```bash
# 上传合约
wasmd tx wasm store luckee_incentive_optimized.wasm --from admin --gas auto --yes

# 实例化合约
wasmd tx wasm instantiate $CODE_ID '{
  "admin":"luckee1admin...",
  "config":{
    "auto_claim_enabled":true,
    "max_rewards_per_user":1000,
    "reward_expiration_days":30
  }
}' --from admin --label "luckee-incentive" --yes
```

## 📚 文档

- [结构化需求规格说明书](docs/01-结构化需求规格说明书.md)
- [精确接口契约](docs/02-精确接口契约.md)
- [架构与设计约束](docs/03-架构与设计约束.md)
- [实现分析报告](实现分析报告.md)
- [项目状态报告](项目状态报告.md)

## 📄 许可证

MIT License