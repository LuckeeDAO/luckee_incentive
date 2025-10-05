use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Decimal, Timestamp};
use cw_storage_plus::{PrimaryKey, Key};

// ===== 实例化消息 =====

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub config: IncentiveConfig,
}

#[cw_serde]
pub struct IncentiveConfig {
    pub max_rewards_per_user: u32,
    pub reward_expiration_days: u64,
    pub auto_claim_enabled: bool,
}

// ===== 执行消息 =====

#[cw_serde]
pub enum ExecuteMsg {
    // 奖励管理
    DistributeReward {
        user: String,
        amount: Uint128,
        activity_type: ActivityType,
    },
    ClaimReward {
        reward_id: String,
    },
    
    // 积分兑换
    MintForPoints {
        user: String,
        points_amount: Uint128,
    },
    
    // 规则管理
    CreateRule {
        rule: RuleDetails,
    },
    UpdateRule {
        rule_id: String,
        rule: RuleDetails,
    },
    DeleteRule {
        rule_id: String,
    },
    
    // 合约注册
    RegisterContract {
        contract_type: ContractType,
        contract_addr: String,
    },
    
    // 用户等级
    UpdateUserLevel {
        user: String,
        points: u32,
    },
    
    // 配置更新
    UpdateConfig {
        config: IncentiveConfig,
    },
}

// ===== 查询消息 =====

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IncentiveConfig)]
    Config {},
    
    #[returns(Vec<UserReward>)]
    UserRewards { user: String },
    
    #[returns(Vec<RuleDetails>)]
    Rules {},
    
    #[returns(Vec<ContractInfo>)]
    Contracts {},
    
    #[returns(UserLevelInfo)]
    UserLevel { user: String },
    
    #[returns(crate::state::SystemStats)]
    SystemStats {},
}

// ===== 数据结构 =====

#[cw_serde]
pub enum ActivityType {
    BlindBoxOpen { nft_kind: String, box_id: String },
    NftExchange { nft_id: String, amount: Uint128 },
    Referral { referrer: String },
    LevelUp { new_level: UserLevel },
    Custom { activity_id: String },
}

#[cw_serde]
pub enum UserLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    GrandMaster,
}

#[cw_serde]
pub enum ContractType {
    Ft,
    BlindBox,
    Nft,
    Custom(String),
}

#[cw_serde]
pub struct RuleDetails {
    pub rule_id: String,
    pub rule_name: String,
    pub rule_type: RuleType,
    pub conditions: Vec<RuleCondition>,
    pub rewards: Vec<RewardDefinition>,
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[cw_serde]
pub enum RuleType {
    ActivityBased,
    LevelBased,
    TimeBased,
    Custom,
}

#[cw_serde]
pub struct RuleCondition {
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub value: String,
}

#[cw_serde]
pub enum ConditionType {
    ActivityType,
    UserLevel,
    TimeRange,
    Amount,
    Custom,
}

#[cw_serde]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    In,
}

#[cw_serde]
pub struct RewardDefinition {
    pub reward_type: RewardType,
    pub amount: Uint128,
    pub multiplier: Decimal,
    pub conditions: Vec<RewardCondition>,
}

#[cw_serde]
pub enum RewardType {
    Token,
    Nft,
    LevelPoints,
    Custom,
}

#[cw_serde]
pub struct RewardCondition {
    pub condition_type: String,
    pub value: String,
}

#[cw_serde]
pub struct UserReward {
    pub reward_id: String,
    pub user: String,
    pub amount: Uint128,
    pub reward_type: RewardType,
    pub activity_type: ActivityType,
    pub created_at: Timestamp,
    pub claimed_at: Option<Timestamp>,
    pub expires_at: Option<Timestamp>,
    pub status: RewardStatus,
}

#[cw_serde]
pub enum RewardStatus {
    Pending,
    Claimed,
    Expired,
    Cancelled,
}

#[cw_serde]
pub struct ContractInfo {
    pub contract_type: ContractType,
    pub contract_addr: Addr,
    pub status: ContractStatus,
    pub capabilities: Vec<String>,
    pub registered_at: Timestamp,
}

#[cw_serde]
pub enum ContractStatus {
    Active,
    Inactive,
    Suspended,
}

#[cw_serde]
pub struct UserLevelInfo {
    pub user: String,
    pub level: UserLevel,
    pub points: u32,
    pub level_up_count: u32,
    pub last_level_up: Option<Timestamp>,
    pub total_rewards: Uint128,
}

// ===== 响应类型 =====

#[cw_serde]
pub struct InstantiateResponse {
    pub contract_addr: Addr,
    pub config: IncentiveConfig,
}

#[cw_serde]
pub struct ExecuteResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
}

#[cw_serde]
pub struct QueryResponse<T> {
    pub data: T,
    pub timestamp: Timestamp,
}

// 为ContractType实现PrimaryKey trait
impl<'a> PrimaryKey<'a> for ContractType {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key<'_>> {
        match self {
            ContractType::Ft => vec![Key::Ref(b"ft")],
            ContractType::BlindBox => vec![Key::Ref(b"blind_box")],
            ContractType::Nft => vec![Key::Ref(b"nft")],
            ContractType::Custom(name) => vec![Key::Ref(b"custom"), Key::Ref(name.as_bytes())],
        }
    }
}
