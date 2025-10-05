# LuckeeIncentive 合约使用示例

本目录包含 LuckeeIncentive 合约的各种使用示例，帮助开发者快速集成和使用合约功能。

## 📁 文件说明

- `instantiate.md` - 合约实例化示例
- `distribute_reward.md` - 奖励分发示例
- `rule_management.md` - 规则管理示例
- `user_level.md` - 用户等级管理示例
- `integration.md` - 与其他合约集成示例

## 🚀 快速开始

### 1. 合约实例化

```bash
# 使用 wasmd CLI
wasmd tx wasm instantiate $CODE_ID '{
  "admin": "luckee1admin...",
  "config": {
    "auto_claim_enabled": true,
    "max_rewards_per_user": 1000,
    "reward_expiration_days": 30
  }
}' --from admin --label "luckee-incentive" --yes
```

### 2. 分发奖励

```bash
# 盲盒开奖奖励
wasmd tx wasm execute $CONTRACT_ADDRESS '{
  "distribute_reward": {
    "user": "luckee1user...",
    "amount": "1000",
    "activity_type": {
      "blind_box_open": {
        "box_id": "box1",
        "nft_kind": "rare"
      }
    }
  }
}' --from admin --yes
```

### 3. 创建规则

```bash
# 创建盲盒规则
wasmd tx wasm execute $CONTRACT_ADDRESS '{
  "create_rule": {
    "rule_id": "blind_box_rule",
    "rule": {
      "activity_type": {
        "blind_box_open": {
          "box_id": "box1",
          "nft_kind": "rare"
        }
      },
      "base_reward": "1000",
      "multiplier": "1.0",
      "enabled": true
    }
  }
}' --from admin --yes
```

## 📚 详细示例

请查看各个示例文件获取更详细的使用说明和代码示例。
