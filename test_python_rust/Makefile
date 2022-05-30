LDIR = ../python_hand_detection

_LIBS = python_landmarks_detector.py
LIBS = $(patsubst %, $(LDIR)/%,$(_LIBS))

build:
	cargo build

copy_debug: build
	copy $(LIBS) /target/debug

copy_rls:
	copy $(LIBS) /target/rls

run_debug: copy_debug
	cargo run