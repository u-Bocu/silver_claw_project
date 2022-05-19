CC=gcc

IDIR=C:/Users/mremond/AppData/Local/Programs/Python/Python310/include
CFLAGS=-I$(IDIR)

ODIR=obj
LDIR=lib

_DLLS=C:/Users/mremond/AppData/Local/Programs/Python/Python310
DLLS=-L$(_DLLS)

LIBS=-lpython310
MUNICODE=-municode

_OBJ = main.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

build: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS) $(DLLS) $(LIBS) $(MUNICODE)

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~