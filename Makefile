define make_dirs
dirs="src/InOneWeekendRust"
if [ -n "$$parallel" ]; then
	trap "kill 0" EXIT
	for dir in $$dirs; do
		$(MAKE) -C $$dir $@ & done
	wait
else
	time echo $$dirs | xargs -n 1 | xargs -IX sh -c "$(MAKE) -C X $@ || exit 255"
fi
endef
export make_dirs
generate:
	bash -c "$${make_dirs}"
run:
	bash -c "$${make_dirs}"