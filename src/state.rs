use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Timestamp, Storage};
use cw_storage_plus::{Item, Map};
use crate::msg::*;

// ===== 配置存储 =====

pub const CONFIG: Item<IncentiveConfig> = Item::new("config");
pub const ADMIN: Item<Addr> = Item::new("admin");

// ===== 奖励存储 =====

pub const USER_REWARDS: Map<String, Vec<UserReward>> = Map::new("user_rewards");
pub const REWARD_COUNTER: Item<u64> = Item::new("reward_counter");

// ===== 规则存储 =====

pub const RULES: Map<String, RuleDetails> = Map::new("rules");
pub const RULE_COUNTER: Item<u64> = Item::new("rule_counter");

// ===== 合约注册存储 =====

pub const CONTRACTS: Map<ContractType, ContractInfo> = Map::new("contracts");

// ===== 用户等级存储 =====

pub const USER_LEVELS: Map<String, UserLevelInfo> = Map::new("user_levels");

// ===== 统计存储 =====

pub const STATS: Item<SystemStats> = Item::new("stats");

// ===== 数据结构 =====

#[cw_serde]
pub struct SystemStats {
    pub total_users: u32,
    pub total_rewards_distributed: Uint128,
    pub total_rules: u32,
    pub total_contracts: u32,
    pub last_updated: Timestamp,
}

// ===== 辅助函数 =====

pub fn get_next_reward_id(storage: &mut dyn Storage) -> Result<String, cosmwasm_std::StdError> {
    let counter = REWARD_COUNTER.may_load(storage)?.unwrap_or(0);
    REWARD_COUNTER.save(storage, &(counter + 1))?;
    Ok(format!("reward_{}", counter))
}

pub fn get_next_rule_id(storage: &mut dyn Storage) -> Result<String, cosmwasm_std::StdError> {
    let counter = RULE_COUNTER.may_load(storage)?.unwrap_or(0);
    RULE_COUNTER.save(storage, &(counter + 1))?;
    Ok(format!("rule_{}", counter))
}
