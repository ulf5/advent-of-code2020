SUBDIRS := $(wildcard */.)

run-all: $(SUBDIRS)

$(SUBDIRS):
	(cd $@ && \
	cargo run --release)

.PHONY: all $(SUBDIRS)
