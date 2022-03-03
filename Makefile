run:
	@cd os && make run
run-log:
	@cd os && make run FLAGS=--features\ LOG_DEBUG
clean:
	@cd os && make clean
tree:
	@cd os && make tree
tree-all:
	@cd os && make tree-all
debug:
	@cd os && make debug

.PHONY: run run-log clean tree tree-all debug

