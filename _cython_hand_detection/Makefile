CC=gcc
CY=cython

IDIR=C:/Users/mremond/AppData/Local/Programs/Python/Python310-32/include
CFLAGS=-I$(IDIR)

ODIR=obj
CDIR=src
LDIR=lib

_DLLS=C:\Users\mremond\AppData\Local\Programs\Python\Python310-32
DLLS=-L$(_DLLS)

LIBS=-lpython310

_OBJ = main.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))

_C = main.c
C = $(patsubst %,$(CDIR)/%,$(_C))

$(CDIR)/%.c:
	$(CY) --embed -o $@ main.py

$(ODIR)/%.o: $(C)
	$(CC) -c -o $@ $< $(CFLAGS)

build: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS) $(DLLS) $(LIBS)

.PHONY: clean

clean:
	rm -f $(CDIR)/*.c *~ core $(INCDIR)/*~
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~