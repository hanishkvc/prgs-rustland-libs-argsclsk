
cbuild:
	cargo build

cclean:
	cargo clean

test:
	target/debug/argsclsk testme --key1 what else --key2 this also --key3 -- these are remaining arguments kept outside my perview

