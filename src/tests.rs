#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, message_info};
    use cosmwasm_std::{coins, from_json, Uint128, Addr};
    use crate::msg::*;
    use crate::contract::{instantiate, execute, query};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = Addr::unchecked("creator");
        let info = message_info(&creator, &coins(1000, "earth"));

        let msg = InstantiateMsg {
            admin: None, // 使用默认管理员（调用者）
            config: IncentiveConfig {
                max_rewards_per_user: 1000,
                reward_expiration_days: 30,
                auto_claim_enabled: true,
            },
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_query_config() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = Addr::unchecked("creator");
        let info = message_info(&creator, &coins(1000, "earth"));

        let msg = InstantiateMsg {
            admin: None, // 使用默认管理员（调用者）
            config: IncentiveConfig {
                max_rewards_per_user: 1000,
                reward_expiration_days: 30,
                auto_claim_enabled: true,
            },
        };

        instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        let res = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
        let config: IncentiveConfig = from_json(&res).unwrap();
        assert_eq!(1000, config.max_rewards_per_user);
    }

    #[test]
    fn test_distribute_reward() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = Addr::unchecked("creator");
        let info = message_info(&creator, &coins(1000, "earth"));

        // 实例化
        let msg = InstantiateMsg {
            admin: None, // 使用默认管理员（调用者）
            config: IncentiveConfig {
                max_rewards_per_user: 1000,
                reward_expiration_days: 30,
                auto_claim_enabled: true,
            },
        };

        instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // 分发奖励
        let admin_info = message_info(&creator, &coins(1000, "earth"));
        let execute_msg = ExecuteMsg::DistributeReward {
            user: "luckee1user123456789012345678901234567890".to_string(),
            amount: Uint128::from(1000u128),
            activity_type: ActivityType::BlindBoxOpen {
                nft_kind: "rare".to_string(),
                box_id: "box1".to_string(),
            },
        };

        let res = execute(deps.as_mut(), env.clone(), admin_info, execute_msg).unwrap();
        assert_eq!(res.attributes.len(), 4);
    }

    #[test]
    fn test_unauthorized_access() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = Addr::unchecked("creator");
        let info = message_info(&creator, &coins(1000, "earth"));

        // 实例化
        let msg = InstantiateMsg {
            admin: None, // 使用默认管理员（调用者）
            config: IncentiveConfig {
                max_rewards_per_user: 1000,
                reward_expiration_days: 30,
                auto_claim_enabled: true,
            },
        };

        instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // 尝试未授权操作
        let user = Addr::unchecked("user");
        let user_info = message_info(&user, &coins(1000, "earth"));
        let execute_msg = ExecuteMsg::DistributeReward {
            user: "luckee1user223456789012345678901234567890".to_string(),
            amount: Uint128::from(1000u128),
            activity_type: ActivityType::BlindBoxOpen {
                nft_kind: "rare".to_string(),
                box_id: "box1".to_string(),
            },
        };

        let res = execute(deps.as_mut(), env, user_info, execute_msg);
        assert!(res.is_err());
    }
}
