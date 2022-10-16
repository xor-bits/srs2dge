##
# srs2dge
#
# @file
# @version 0.1

# gui_main loading_screen main
EXAMPLES            = asteroids ecs gizmos loading_screen gui multi platformer post_process sdf simple tetris text texture
RUSTFLAGS           =
RUSTFLAGS_WASM      = --cfg=web_sys_unstable_apis
FLAGS               = --profile="release"
WASM_FLAGS          = --no-default-features \
                      --features="all" \
                      --target="wasm32-unknown-unknown" \
                      --profile="release-wasm"

WASM_DEPS           = $(filter-out %: ,$(file < $(WASM_DIR)/.d))
WASM_DIR            = target/wasm32-unknown-unknown/release-wasm/examples
WASM_WWW_DIR        = target/www



$(WASM_DIR)/%.wasm : $(WASM_DEPS)
	@echo "Compiling $@"
#   compile wasm
	@RUSTFLAGS="${RUSTFLAGS_WASM}" \
		cargo build \
		$(WASM_FLAGS) \
		--example=$(patsubst $(WASM_DIR)/%.wasm,%, $@)

$(WASM_WWW_DIR)/%_bg.wasm: $(WASM_DIR)/%.wasm
#   generate bindgen for load.js
#   generates %.d.ts %.js %_bg.wasm %_bg.wasm.d.ts
	@RUST_LOG="error" \
		wasm-bindgen \
		--web "$^" \
		--out-dir="$(WASM_WWW_DIR)"
#   optimize wasm
	@RUST_LOG="error" wasm-opt -Oz -o \
		"$@" \
		"$@"

$(WASM_WWW_DIR)/%/index.html: generated/index.html
#   generate index.html
	@mkdir -p "$(shell dirname $@)"
	@cp -f "$^" "$@"

$(WASM_WWW_DIR)/%/load.js:
#   generate load.js
	@mkdir -p "$(shell dirname $@)"
	@echo "import init from \"../$(patsubst $(WASM_WWW_DIR)/%/load.js,%.js, $@)\"; init();" > "$@"

wasm-all: $(patsubst %,%-wasm, $(EXAMPLES))
wasm-run-all: $(patsubst %,%-wasm, $(EXAMPLES))
	@cp -f "generated/all.html" "$(WASM_WWW_DIR)/index.html"
	@miniserve $(WASM_WWW_DIR)/ --index=index.html
%-wasm: $(WASM_WWW_DIR)/%_bg.wasm $(WASM_WWW_DIR)/%/index.html $(WASM_WWW_DIR)/%/load.js
	@echo "Generating $@"
	@cp "$(patsubst %-wasm,$(WASM_WWW_DIR)/%.js, $@)" "$(patsubst %-wasm,$(WASM_WWW_DIR)/%/, $@)$(patsubst %-wasm,%.js, $@)"
	@cp "$(patsubst %-wasm,$(WASM_WWW_DIR)/%_bg.wasm, $@)" "$(patsubst %-wasm,$(WASM_WWW_DIR)/%/, $@)$(patsubst %-wasm,%_bg.wasm, $@)"
%-wasm-run: %-wasm
	@echo "Running $@"
	@miniserve $(patsubst %-wasm,$(WASM_WWW_DIR)/%/, $@) --index=index.html

native-all: $(patsubst %,%-native, $(EXAMPLES))
native-run-all: $(patsubst %,%-native-run, $(EXAMPLES))
%-native:
	@cargo build --release --example=$(patsubst %-native,%, $@)
%-native-run:
	@cargo run --release --example=$(patsubst %-native-run,%, $@)

dbg:
	@echo "$(WASM_WWW_DIR)/%/%_bg.wasm"

.PHONY: list %-wasm-run %-native-run wasm-all wasm-run-all native-all native-run-all dbg
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

# end
