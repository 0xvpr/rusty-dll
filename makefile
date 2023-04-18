PROJECT    = rusty_dll

BIN        = bin
LIB        = lib

MAKEFLAGS += $(addprefix -j,$(shell nproc))

all: $(PROJECT) c_app

.PHONY: c_app
c_app: $(BIN)
	make -C $@
	mv $@/$@.exe ./bin/$@.exe

$(PROJECT): $(LIB)
	cargo build --release --target "x86_64-pc-windows-gnu"
	cp ./target/x86_64-pc-windows-gnu/release/$(PROJECT).dll ./lib/$(PROJECT).dll

$(BIN):
	mkdir -p $@

$(LIB):
	mkdir -p $@

.PHONY: extra-clean
clean:
	rm -fr ./lib/* ./bin/* ./target/* `find . -name "*.o*"`

.PHONY: extra-clean
extra-clean:
	rm -fr ./lib ./bin ./target ./Cargo.lock  `find . -name "*.o"`
