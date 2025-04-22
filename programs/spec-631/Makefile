include ../c-build/Makefile

CFLAGS+=-DSPEC

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/spec.c)
	$(call archive,$(OUTDIR)/$(PROGRAM).o)
