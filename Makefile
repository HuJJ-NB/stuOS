run:
	@cd os && make run
clean:
	@cd os && make clean
tree:
	@cd os && make tree
debug:
	@cd os && make debug

.PHONY: run clean tree debug

