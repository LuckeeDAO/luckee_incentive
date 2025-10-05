use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONFIG, ADMIN, USER_REWARDS, RULES, CONTRACTS, USER_LEVELS, get_next_reward_id, get_next_rule_id};

const CONTRACT_NAME: &str = "luckee-incentive";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg.admin
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    ADMIN.save(deps.storage, &admin)?;
    CONFIG.save(deps.storage, &msg.config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin)
        .add_attribute("config", format!("{:?}", msg.config)))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DistributeReward { user, amount, activity_type } => {
            execute_distribute_reward(deps, env, info, user, amount, activity_type)
        },
        ExecuteMsg::ClaimReward { reward_id } => {
            execute_claim_reward(deps, env, info, reward_id)
        },
        ExecuteMsg::MintForPoints { user, points_amount } => {
            execute_mint_for_points(deps, env, info, user, points_amount)
        },
        ExecuteMsg::CreateRule { rule } => {
            execute_create_rule(deps, env, info, rule)
        },
        ExecuteMsg::UpdateRule { rule_id, rule } => {
            execute_update_rule(deps, env, info, rule_id, rule)
        },
        ExecuteMsg::DeleteRule { rule_id } => {
            execute_delete_rule(deps, env, info, rule_id)
        },
        ExecuteMsg::RegisterContract { contract_type, contract_addr } => {
            execute_register_contract(deps, env, info, contract_type, contract_addr)
        },
        ExecuteMsg::UpdateUserLevel { user, points } => {
            execute_update_user_level(deps, env, info, user, points)
        },
        ExecuteMsg::UpdateConfig { config } => {
            execute_update_config(deps, env, info, config)
        },
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::UserRewards { user } => to_json_binary(&query_user_rewards(deps, user)?),
        QueryMsg::Rules {} => to_json_binary(&query_rules(deps)?),
        QueryMsg::Contracts {} => to_json_binary(&query_contracts(deps)?),
        QueryMsg::UserLevel { user } => to_json_binary(&query_user_level(deps, user)?),
        QueryMsg::SystemStats {} => to_json_binary(&query_system_stats(deps)?),
    }
}

// ===== 执行函数 =====

fn execute_distribute_reward(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: String,
    amount: Uint128,
    activity_type: crate::msg::ActivityType,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 创建奖励
    let reward_id = get_next_reward_id(deps.storage)?;
    let reward = crate::msg::UserReward {
        reward_id: reward_id.clone(),
        user: user.clone(),
        amount,
        reward_type: crate::msg::RewardType::Token,
        activity_type,
        created_at: env.block.time,
        claimed_at: None,
        expires_at: None,
        status: crate::msg::RewardStatus::Pending,
    };

    // 保存奖励
    let mut user_rewards = USER_REWARDS.load(deps.storage, user.clone()).unwrap_or_default();
    user_rewards.push(reward);
    USER_REWARDS.save(deps.storage, user.clone(), &user_rewards)?;

    Ok(Response::new()
        .add_attribute("method", "distribute_reward")
        .add_attribute("user", user)
        .add_attribute("reward_id", reward_id)
        .add_attribute("amount", amount))
}

fn execute_claim_reward(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    reward_id: String,
) -> Result<Response, ContractError> {
    // 查找奖励
    let user_rewards = USER_REWARDS.load(deps.storage, info.sender.to_string()).unwrap_or_default();
    let mut reward_found = false;
    
    for mut reward in user_rewards {
        if reward.reward_id == reward_id {
            if reward.status == crate::msg::RewardStatus::Pending {
                reward.status = crate::msg::RewardStatus::Claimed;
                reward.claimed_at = Some(env.block.time);
                reward_found = true;
                break;
            }
        }
    }

    if !reward_found {
        return Err(ContractError::RewardNotFound {});
    }

    Ok(Response::new()
        .add_attribute("method", "claim_reward")
        .add_attribute("reward_id", reward_id)
        .add_attribute("user", info.sender))
}

fn execute_mint_for_points(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: String,
    points_amount: Uint128,
) -> Result<Response, ContractError> {
    // 检查权限 - 只有管理员可以执行积分兑换
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 检查最小兑换限制
    if points_amount < Uint128::from(1000u128) {
        return Err(ContractError::InvalidAmount {});
    }

    // 创建代币铸造奖励
    let reward_id = get_next_reward_id(deps.storage)?;
    let reward = crate::msg::UserReward {
        reward_id: reward_id.clone(),
        user: user.clone(),
        amount: points_amount, // 1:1 兑换比例
        reward_type: crate::msg::RewardType::Token,
        activity_type: crate::msg::ActivityType::Custom { activity_id: "points_exchange".to_string() },
        created_at: env.block.time,
        claimed_at: None,
        expires_at: None,
        status: crate::msg::RewardStatus::Pending,
    };

    // 保存奖励
    let mut user_rewards = USER_REWARDS.load(deps.storage, user.clone()).unwrap_or_default();
    user_rewards.push(reward);
    USER_REWARDS.save(deps.storage, user.clone(), &user_rewards)?;

    Ok(Response::new()
        .add_attribute("method", "mint_for_points")
        .add_attribute("user", user)
        .add_attribute("points_amount", points_amount)
        .add_attribute("reward_id", reward_id))
}

fn execute_create_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut rule: crate::msg::RuleDetails,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 生成规则ID
    let rule_id = get_next_rule_id(deps.storage)?;
    rule.rule_id = rule_id.clone();
    rule.created_at = env.block.time;
    rule.updated_at = env.block.time;

    // 保存规则
    RULES.save(deps.storage, rule_id.clone(), &rule)?;

    Ok(Response::new()
        .add_attribute("method", "create_rule")
        .add_attribute("rule_id", rule_id))
}

fn execute_update_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rule_id: String,
    mut rule: crate::msg::RuleDetails,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 更新规则
    rule.rule_id = rule_id.clone();
    rule.updated_at = env.block.time;
    RULES.save(deps.storage, rule_id.clone(), &rule)?;

    Ok(Response::new()
        .add_attribute("method", "update_rule")
        .add_attribute("rule_id", rule_id))
}

fn execute_delete_rule(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rule_id: String,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 删除规则
    RULES.remove(deps.storage, rule_id.clone());

    Ok(Response::new()
        .add_attribute("method", "delete_rule")
        .add_attribute("rule_id", rule_id))
}

fn execute_register_contract(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract_type: crate::msg::ContractType,
    contract_addr: String,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 验证地址
    let addr = deps.api.addr_validate(&contract_addr)?;

    // 创建合约信息
    let contract_info = crate::msg::ContractInfo {
        contract_type: contract_type.clone(),
        contract_addr: addr,
        status: crate::msg::ContractStatus::Active,
        capabilities: vec![],
        registered_at: env.block.time,
    };

    // 保存合约信息
    CONTRACTS.save(deps.storage, contract_type.clone(), &contract_info)?;

    Ok(Response::new()
        .add_attribute("method", "register_contract")
        .add_attribute("contract_type", format!("{:?}", contract_type)))
}

fn execute_update_user_level(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user: String,
    points: u32,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 更新用户等级
    let mut user_level = USER_LEVELS.load(deps.storage, user.clone()).unwrap_or_else(|_| {
        crate::msg::UserLevelInfo {
            user: user.clone(),
            level: crate::msg::UserLevel::Bronze,
            points: 0,
            level_up_count: 0,
            last_level_up: None,
            total_rewards: Uint128::zero(),
        }
    });

    user_level.points = points;
    USER_LEVELS.save(deps.storage, user.clone(), &user_level)?;

    Ok(Response::new()
        .add_attribute("method", "update_user_level")
        .add_attribute("user", user)
        .add_attribute("points", points.to_string()))
}

fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    config: crate::msg::IncentiveConfig,
) -> Result<Response, ContractError> {
    // 检查权限
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
        }
        
        // 更新配置
    CONFIG.save(deps.storage, &config)?;
        
        Ok(Response::new()
        .add_attribute("method", "update_config"))
}

// ===== 查询函数 =====

fn query_config(deps: Deps) -> StdResult<crate::msg::IncentiveConfig> {
    CONFIG.load(deps.storage)
}

fn query_user_rewards(deps: Deps, user: String) -> StdResult<Vec<crate::msg::UserReward>> {
    Ok(USER_REWARDS.load(deps.storage, user.clone()).unwrap_or_default())
}

fn query_rules(deps: Deps) -> StdResult<Vec<crate::msg::RuleDetails>> {
    let rules: Result<Vec<_>, _> = RULES.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    Ok(rules?.into_iter().map(|(_, rule)| rule).collect())
}

fn query_contracts(_deps: Deps) -> StdResult<Vec<crate::msg::ContractInfo>> {
    // 简化实现：返回空列表，因为ContractType作为key比较复杂
    Ok(vec![])
}

fn query_user_level(deps: Deps, user: String) -> StdResult<crate::msg::UserLevelInfo> {
    USER_LEVELS.load(deps.storage, user.clone())
}

fn query_system_stats(_deps: Deps) -> StdResult<crate::state::SystemStats> {
    // 简化实现：返回基础统计信息
    Ok(crate::state::SystemStats {
        total_users: 0, // 实际实现中应该统计用户数量
        total_rewards_distributed: Uint128::zero(), // 实际实现中应该统计总奖励
        total_rules: 0, // 实际实现中应该统计规则数量
        total_contracts: 0, // 实际实现中应该统计合约数量
        last_updated: cosmwasm_std::Timestamp::from_seconds(0), // 实际实现中应该使用当前时间
    })
}
