buildtype = release

PROJECT = rust-nes-emulator
TARGET = asmjs-unknown-emscripten

DOCS_DIR = docs
DOCS_PORT = 8080

JS_FILE = $(PROJECT).js

CARGO_OUTDIR = target/$(TARGET)/$(buildtype)

CARGO = cargo
EMCC = emcc

CARGO_OPTION = --target $(TARGET)
EMCC_OPTION = -s USE_SDL=2

ifeq ($(buildtype),release)
CARGO_OPTION += --release
EMCC_OPTION += -O3

else ifeq ($(buildtype),debug)
CARGO_OPTION +=
EMCC_OPTION += -g4
DOCS_FILES = $(DOCS_DIR)/$(JS_FILE)

else
$(error "unknown buildtype")
endif

EMCC_OPTION += --preload-file assets

all: $(DOCS_DIR)/$(JS_FILE)
.PHONY: all

clean:
	$(CARGO) clean
	$(RM) $(DOCS_DIR)/*.js $(DOCS_DIR)/*.js.mem
.PHONY: clean

serve: all
	ruby -run -e httpd $(DOCS_DIR) -p $(DOCS_PORT)

FORCE:
.PHONY: FORCE

$(CARGO_OUTDIR)/$(JS_FILE): build-deps FORCE
	$(RM) $(DOCS_DIR)/*.js $(DOCS_DIR)/*.js.mem
	EMMAKEN_CFLAGS="$(EMCC_OPTION)" $(CARGO) build $(CARGO_OPTION)

$(DOCS_DIR)/$(JS_FILE): $(CARGO_OUTDIR)/$(JS_FILE) FORCE
	find $(CARGO_OUTDIR) \( -name '*.js' -or -name '*.js.mem' -or -name '*.data' \) -exec cp {} $(DOCS_DIR) \;

# https://github.com/kripken/emscripten/issues/4151#issuecomment-193909827
build-deps:
	embuilder.py build sdl2 gl libc dlmalloc
.PHONY: build-deps
