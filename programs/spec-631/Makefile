include ../c-build/Makefile

CC=clang++
CFLAGS+=-DSPEC

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/sjeng.cpp)
	$(call add,epd,guest/epd.cpp)
	$(call add,utils,guest/utils.cpp)
	$(call archive,$(OUTDIR)/utils.o,$(OUTDIR)/epd.o,$(OUTDIR)/$(PROGRAM).o)
