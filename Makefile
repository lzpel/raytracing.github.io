MAKE_RECURSIVE_DIRS := src/InOneWeekendRust
define MAKE_RECURSIVE
	time printf '%s\n' $(MAKE_RECURSIVE_DIRS) | xargs -IX sh -c '$(MAKE) -C X $@ || exit 255'
endef
export
generate:
	bash -c "$${make_dirs}"
run:
	bash -c "$${make_dirs}"
format:
	bash -c "$${make_dirs}"