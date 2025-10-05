//! # Luckee Incentive Contract
//! 
//! 这是一个基于 CosmWasm 的智能合约，专门为 Luckee 生态系统设计，
//! 提供全面的激励和奖励分发功能。
//! 
//! ## 核心功能
//! 
//! - 奖励计算和分发
//! - 规则管理和配置
//! - 合约注册和协调
//! - 用户等级管理
//! - 权限控制和安全机制
//! 
//! ## 使用示例
//! 
//! ```rust
//! use luckee_incentive::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, IncentiveConfig, ActivityType};
//! use luckee_incentive::contract::{instantiate, execute, query};
//! use cosmwasm_std::Uint128;
//! 
//! // 实例化合约
//! let msg = InstantiateMsg {
//!     admin: Some("admin".to_string()),
//!     config: IncentiveConfig {
//!         auto_claim_enabled: true,
//!         max_rewards_per_user: 1000,
//!         reward_expiration_days: 30,
//!     },
//! };
//! 
//! // 执行操作
//! let execute_msg = ExecuteMsg::DistributeReward {
//!     user: "user1".to_string(),
//!     amount: Uint128::from(1000u128),
//!     activity_type: ActivityType::BlindBoxOpen {
//!         box_id: "box1".to_string(),
//!         nft_kind: "rare".to_string(),
//!     },
//! };
//! 
//! // 查询状态
//! let query_msg = QueryMsg::Config {};
//! ```

pub mod msg;
pub mod state;
pub mod contract;
pub mod error;

// 测试模块
#[cfg(test)]
mod tests;

pub use crate::error::ContractError;
pub use crate::contract::{instantiate, execute, query};
pub use crate::msg::*;
