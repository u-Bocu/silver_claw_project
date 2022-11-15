# Check SHELL type
ifeq ($(OS),Windows_NT)

ifneq ($(strip $(filter %sh,$(basename $(realpath $(SHELL))))),)
POSIXSHELL := 1
else
POSIXSHELL :=
endif

else
POSIXSHELL := 1
endif

ifneq ($(POSIXSHELL),)
CMDSEP := ;
PSEP := /
CPF := cp -f

else
CMDSEP := &
PSEP := \\
CPF := copy /y
endif

# Python directory and filename
LDIR = ..$(PSEP)python_hand_detection

_LIBS = hand_landmarks_detector.py
LIBS = $(patsubst %, $(LDIR)$(PSEP)%,$(_LIBS))

# Build
build:
	cargo build

build_debug:
	cargo build --bin silver_claw_debug_version

build_unit_tests:
	cargo build --bin silver_claw_unit_tests

build_rls:
	cargo build --release

# Copy Python File
copy: build
	$(CPF) $(LIBS) target$(PSEP)debug$(PSEP)$(_LIBS)

copy_debug: build_debug
	$(CPF) $(LIBS) target$(PSEP)debug$(PSEP)$(_LIBS)

copy_rls: build_rls
	$(CPF) $(LIBS) target$(PSEP)rls$(PSEP)$(_LIBS)

# Run
run_debug: copy_debug
	cargo run --bin silver_claw_debug_version

run_unit_tests: build_unit_tests
	cargo run --bin silver_claw_unit_tests

run_installer: copy
	cargo run --bin silver_claw_installer

run_rls: copy_rls
	cargo run --bin silver_claw_installer --release 