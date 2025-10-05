# Contributing to LuckeeIncentive

感谢您对 LuckeeIncentive 项目的关注！我们欢迎各种形式的贡献。

## 🚀 快速开始

### 开发环境设置

1. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **安装 WASM 目标**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **安装 cosmwasm-opt (可选)**
   ```bash
   wget https://github.com/CosmWasm/cosmwasm/releases/download/v1.5.0/cosmwasm-opt
   chmod +x cosmwasm-opt
   sudo mv cosmwasm-opt /usr/local/bin/
   ```

4. **克隆仓库**
   ```bash
   git clone git@github.com:LuckeeDAO/luckee_incentive.git
   cd luckee_incentive
   ```

### 构建和测试

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 构建 WASM
cargo build --release --target wasm32-unknown-unknown

# 优化 WASM (需要 cosmwasm-opt)
cosmwasm-opt target/wasm32-unknown-unknown/release/luckee_incentive.wasm \
  -o target/wasm32-unknown-unknown/release/luckee_incentive_optimized.wasm
```

## 📝 贡献指南

### 代码规范

1. **格式化代码**
   ```bash
   cargo fmt
   ```

2. **运行 Clippy**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **编写测试**
   - 为新功能编写单元测试
   - 确保测试覆盖率 > 90%
   - 使用描述性的测试名称

### 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**类型**：
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 重构代码
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

**示例**：
```
feat: add user level management system
fix: resolve reward calculation overflow issue
docs: update API documentation
test: add integration tests for reward distribution
```

### Pull Request 流程

1. **Fork 仓库**
2. **创建功能分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **提交更改**
   ```bash
   git commit -m "feat: add your feature"
   ```
4. **推送分支**
   ```bash
   git push origin feature/your-feature-name
   ```
5. **创建 Pull Request**

### PR 要求

- [ ] 代码通过所有测试
- [ ] 代码通过 Clippy 检查
- [ ] 代码格式化正确
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] 提交信息符合规范

## 🐛 报告问题

### Bug 报告

使用 [GitHub Issues](https://github.com/LuckeeDAO/luckee_incentive/issues) 报告 bug，请包含：

1. **环境信息**
   - Rust 版本
   - 操作系统
   - CosmWasm 版本

2. **复现步骤**
   - 详细的操作步骤
   - 预期结果
   - 实际结果

3. **错误信息**
   - 完整的错误日志
   - 相关的代码片段

### 功能请求

使用 [GitHub Issues](https://github.com/LuckeeDAO/luckee_incentive/issues) 提出功能请求，请包含：

1. **功能描述**
   - 详细的功能说明
   - 使用场景

2. **实现建议**
   - 技术方案
   - API 设计

3. **优先级**
   - 重要性评估
   - 紧急程度

## 📚 文档

- [项目文档](docs/)
- [API 文档](docs/02-精确接口契约.md)
- [架构设计](docs/03-架构与设计约束.md)

## 🤝 社区

- **Discord**: [LuckeeDAO Discord](https://discord.gg/luckeedao)
- **Telegram**: [LuckeeDAO Telegram](https://t.me/luckeedao)
- **Twitter**: [@LuckeeDAO](https://twitter.com/luckeedao)

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

感谢所有为 LuckeeIncentive 项目做出贡献的开发者！
