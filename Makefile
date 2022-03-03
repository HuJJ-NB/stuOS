run:
	@cd os && make run
run-log:
	@cd os && make run FLAGS=--features\ LOG_DEBUG
clean:
	@cd os && make clean
tree:
	@cd os && make tree
debug:
	@cd os && make debug

.PHONY: run clean tree debug

