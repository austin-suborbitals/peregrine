KCOV_BUILD=kcov-master/build
KCOV_INSTALL=~/.bin

COV_TARGET=../target/cov
COV_BIN_BASE=../target/debug/peregrine

kcov:
	@build_kcov.sh $(KCOV_INSTALL) $(KCOV_BUILD)

coverage: kcov
	@echo "currently in: "$(shell pwd)
	 $(KCOV_INSTALL)/bin/kcov --verify $(COV_TARGET) $(COV_BIN_BASE)-*
