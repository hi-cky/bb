# Makefile

# 项目名称
PROJECT_NAME = bb_server

# Rust 编译器
RUSTC = cargo

# 配置文件路径
APP_CONFIG_PATH = src/config.yaml
# 数据库URL
# DATABASE_URL = sqlite://bb.sqlite
DATABASE_URL = mysql://root@127.0.0.1:3306/bb

.PHONY: run migrate test test-s clean

# 设置环境变量
export DATABASE_URL
export APP_CONFIG_PATH
export TEST_THREADS = 1  # 设置测试线程数为 1

# diesel 数据库迁移
migrate: 
	diesel setup
	diesel migration run

# 运行项目
run:
	$(RUSTC) run

# 运行所有测试
test:
	$(RUSTC) test $(T)

# 单线程测试
test-s:
	$(RUSTC) test $(T) -- --test-threads=1

	
# 清理构建产物
clean:
	$(RUSTC) clean