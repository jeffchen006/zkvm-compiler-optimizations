include ../c-build/Makefile

CFLAGS+=-DSPEC

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,mcf,guest/mcf.c)
	$(call archive,$(OUTDIR)/mcf.o)
