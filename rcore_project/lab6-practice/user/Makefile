.PHONY: build

TARGET      := riscv64imac-unknown-none-elf
MODE        := debug

# 用户程序目录
SRC_DIR		:= src/bin
# 编译后执行文件目录
TARGET_DIR	:= target/$(TARGET)/$(MODE)
# 用户程序源文件
SRC_FILES	:= $(wildcard $(SRC_DIR)/*.rs)
# 根据源文件取得编译后的执行文件
BIN_FILES	:= $(patsubst $(SRC_DIR)/%.rs, $(TARGET_DIR)/%, $(SRC_FILES))

OUT_DIR		:= build/disk
IMG_FILE	:= build/raw.img
QCOW_FILE	:= build/disk.img

# 安装 rcore-fs-fuse 工具
dependency:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
endif

TEST_FILE	:= test.file

build: dependency
	# 编译
	@cargo build
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	# 移除原有的所有文件
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	# 复制编译生成的 ELF 至目标目录
	@cp $(BIN_FILES) $(OUT_DIR)
	@cp $(TEST_FILE) $(OUT_DIR)
	# 使用 rcore-fs-fuse 工具进行打包
	@rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
	# 将镜像文件的格式转换为 QEMU 使用的高级格式
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	# 提升镜像文件的容量（并非实际大小），来允许更多数据写入
	@qemu-img resize $(QCOW_FILE) +1G

clean:
	@cargo clean
	@rm -rf $(OUT_DIR) $(IMG_FILE) $(QCOW_FILE)