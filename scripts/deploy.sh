#!/bin/bash

# LuckeeIncentive 合约部署脚本
# 基于文档要求实现自动化部署

set -e

# 配置变量
CONTRACT_NAME="luckee_incentive"
CONTRACT_VERSION="0.1.0"
NETWORK="testnet"
CHAIN_ID="luckee-testnet-1"
NODE_URL="https://rpc.luckee-testnet.com"
GAS_PRICES="0.025uluckee"
GAS_ADJUSTMENT="1.5"
ADMIN_ADDRESS="luckee1admin..."  # 替换为实际管理员地址

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 显示帮助信息
show_help() {
    echo "LuckeeIncentive 合约部署脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  --admin ADDRESS        管理员地址"
    echo "  --ft-contract ADDRESS FT合约地址"
    echo "  --network NETWORK     网络名称 (默认: testnet)"
    echo "  --node URL           节点URL"
    echo "  --help               显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 --admin luckee1admin... --ft-contract luckee1ft..."
    echo "  $0 --admin luckee1admin... --network mainnet"
}

# 解析命令行参数
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --admin)
                ADMIN_ADDRESS="$2"
                shift 2
                ;;
            --ft-contract)
                FT_CONTRACT="$2"
                shift 2
                ;;
            --network)
                NETWORK="$2"
                case $NETWORK in
                    testnet)
                        CHAIN_ID="luckee-testnet-1"
                        NODE_URL="https://rpc.luckee-testnet.com"
                        ;;
                    mainnet)
                        CHAIN_ID="luckee-mainnet-1"
                        NODE_URL="https://rpc.luckee-mainnet.com"
                        ;;
                    *)
                        log_error "不支持的网络: $NETWORK"
                        exit 1
                        ;;
                esac
                shift 2
                ;;
            --node)
                NODE_URL="$2"
                shift 2
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log_error "未知参数: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# 检查依赖
check_dependencies() {
    log_info "检查部署依赖..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo 未安装"
        exit 1
    fi
    
    if ! command -v cosmwasm-opt &> /dev/null; then
        log_warn "cosmwasm-opt 未安装，将使用未优化的版本"
    fi
    
    if ! command -v wasmd &> /dev/null; then
        log_error "wasmd 未安装"
        exit 1
    fi
    
    if ! command -v jq &> /dev/null; then
        log_error "jq 未安装"
        exit 1
    fi
    
    log_info "依赖检查完成"
}

# 构建合约
build_contract() {
    log_info "构建 LuckeeIncentive 合约..."
    
    cd /home/lc/luckee_dao/luckee_incentive
    
    # 清理之前的构建
    cargo clean
    
    # 构建合约
    cargo build --release --target wasm32-unknown-unknown
    
    if [ $? -ne 0 ]; then
        log_error "合约构建失败"
        exit 1
    fi
    
    # 优化WASM文件
    if command -v cosmwasm-opt &> /dev/null; then
        log_info "优化 WASM 文件..."
        cosmwasm-opt target/wasm32-unknown-unknown/release/luckee_incentive.wasm \
            -o target/wasm32-unknown-unknown/release/luckee_incentive_optimized.wasm
        WASM_FILE="target/wasm32-unknown-unknown/release/luckee_incentive_optimized.wasm"
    else
        WASM_FILE="target/wasm32-unknown-unknown/release/luckee_incentive.wasm"
    fi
    
    log_info "合约构建完成: $WASM_FILE"
}

# 上传合约
upload_contract() {
    log_info "上传合约到链上..."
    
    # 检查WASM文件大小
    WASM_SIZE=$(stat -c%s "$WASM_FILE")
    MAX_SIZE=$((1024 * 1024))  # 1MB限制
    
    if [ $WASM_SIZE -gt $MAX_SIZE ]; then
        log_error "WASM文件过大: ${WASM_SIZE} bytes > ${MAX_SIZE} bytes"
        exit 1
    fi
    
    log_info "WASM文件大小: ${WASM_SIZE} bytes"
    
    # 上传合约
    UPLOAD_RESULT=$(wasmd tx wasm store "$WASM_FILE" \
        --from "$ADMIN_ADDRESS" \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --gas-prices "$GAS_PRICES" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --yes \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约上传失败"
        exit 1
    fi
    
    # 提取代码ID
    CODE_ID=$(echo "$UPLOAD_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ -z "$CODE_ID" ]; then
        log_error "无法获取代码ID"
        exit 1
    fi
    
    log_info "合约上传成功，代码ID: $CODE_ID"
    echo "$CODE_ID" > /tmp/luckee_incentive_code_id.txt
}

# 实例化合约
instantiate_contract() {
    log_info "实例化 LuckeeIncentive 合约..."
    
    CODE_ID=$(cat /tmp/luckee_incentive_code_id.txt)
    
    # 实例化消息
    INSTANTIATE_MSG='{
        "admin": "'$ADMIN_ADDRESS'",
        "config": {
            "auto_claim_enabled": true,
            "max_rewards_per_user": 1000,
            "reward_expiration_days": 30
        }
    }'
    
    # 实例化合约
    INSTANTIATE_RESULT=$(wasmd tx wasm instantiate "$CODE_ID" "$INSTANTIATE_MSG" \
        --from "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS" \
        --label "LuckeeIncentive v$CONTRACT_VERSION" \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --gas-prices "$GAS_PRICES" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --yes \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约实例化失败"
        exit 1
    fi
    
    # 提取合约地址
    CONTRACT_ADDRESS=$(echo "$INSTANTIATE_RESULT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
    
    if [ -z "$CONTRACT_ADDRESS" ]; then
        log_error "无法获取合约地址"
        exit 1
    fi
    
    log_info "合约实例化成功，地址: $CONTRACT_ADDRESS"
    echo "$CONTRACT_ADDRESS" > /tmp/luckee_incentive_contract_address.txt
}

# 验证部署
verify_deployment() {
    log_info "验证合约部署..."
    
    CONTRACT_ADDRESS=$(cat /tmp/luckee_incentive_contract_address.txt)
    
    # 查询配置信息
    CONFIG_INFO=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
        '{"config":{}}' \
        --node "$NODE_URL" \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约验证失败"
        exit 1
    fi
    
    AUTO_CLAIM=$(echo "$CONFIG_INFO" | jq -r '.data.auto_claim_enabled')
    MAX_REWARDS=$(echo "$CONFIG_INFO" | jq -r '.data.max_rewards_per_user')
    EXPIRATION_DAYS=$(echo "$CONFIG_INFO" | jq -r '.data.reward_expiration_days')
    
    log_info "配置信息验证成功:"
    log_info "  自动领取: $AUTO_CLAIM"
    log_info "  最大奖励数: $MAX_REWARDS"
    log_info "  过期天数: $EXPIRATION_DAYS"
    
    # 查询管理员信息
    ADMIN_INFO=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
        '{"admin":{}}' \
        --node "$NODE_URL" \
        --output json)
    
    ADMIN=$(echo "$ADMIN_INFO" | jq -r '.data.admin')
    log_info "  管理员: $ADMIN"
}

# 生成部署报告
generate_report() {
    log_info "生成部署报告..."
    
    CODE_ID=$(cat /tmp/luckee_incentive_code_id.txt)
    CONTRACT_ADDRESS=$(cat /tmp/luckee_incentive_contract_address.txt)
    
    REPORT_FILE="/tmp/luckee_incentive_deployment_report.md"
    
    cat > "$REPORT_FILE" << EOF
# LuckeeIncentive 合约部署报告

## 部署信息
- **合约名称**: $CONTRACT_NAME
- **版本**: $CONTRACT_VERSION
- **网络**: $NETWORK
- **链ID**: $CHAIN_ID
- **部署时间**: $(date)

## 合约详情
- **代码ID**: $CODE_ID
- **合约地址**: $CONTRACT_ADDRESS
- **管理员**: $ADMIN_ADDRESS

## 部署状态
✅ 构建成功
✅ 上传成功
✅ 实例化成功
✅ 验证成功

## 后续步骤
1. 更新前端配置中的合约地址
2. 配置监控和告警
3. 进行功能测试
4. 更新文档

## 相关命令
\`\`\`bash
# 查询配置
wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"config":{}}' --node $NODE_URL

# 查询用户奖励
wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"user_rewards":{"user":"'$ADMIN_ADDRESS'"}}' --node $NODE_URL

# 分发奖励
wasmd tx wasm execute $CONTRACT_ADDRESS '{"distribute_reward":{"user":"'$ADMIN_ADDRESS'","amount":"1000","activity_type":{"blind_box_open":{"box_id":"box1","nft_kind":"rare"}}}}' --from $ADMIN_ADDRESS --node $NODE_URL

# 创建规则
wasmd tx wasm execute $CONTRACT_ADDRESS '{"create_rule":{"rule_id":"test_rule","rule":{"activity_type":{"blind_box_open":{"box_id":"box1","nft_kind":"rare"}},"base_reward":"1000","multiplier":"1.0","enabled":true}}}' --from $ADMIN_ADDRESS --node $NODE_URL
\`\`\`
EOF
    
    log_info "部署报告已生成: $REPORT_FILE"
}

# 主函数
main() {
    log_info "开始部署 LuckeeIncentive 合约..."
    
    parse_args "$@"
    
    # 验证必需参数
    if [ -z "$ADMIN_ADDRESS" ] || [ "$ADMIN_ADDRESS" = "luckee1admin..." ]; then
        log_error "请提供有效的管理员地址"
        show_help
        exit 1
    fi
    
    check_dependencies
    build_contract
    upload_contract
    instantiate_contract
    verify_deployment
    generate_report
    
    log_info "LuckeeIncentive 合约部署完成！"
    
    # 清理临时文件
    rm -f /tmp/luckee_incentive_code_id.txt /tmp/luckee_incentive_contract_address.txt
}

# 执行主函数
main "$@"