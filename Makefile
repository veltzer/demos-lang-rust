##############
# parameters #
##############
# do you want dependency on the Makefile itself ?
DO_ALLDEP:=1
# do you want to show the commands executed ?
DO_MKDBG?=0
# do you compile rust files?
DO_EXECS:=1

########
# code #
########
ALL:=

# dependency on the makefile itself
ifeq ($(DO_ALLDEP),1)
.EXTRA_PREREQS+=$(foreach mk, ${MAKEFILE_LIST},$(abspath ${mk}))
endif # DO_ALLDEP

# silent stuff
ifeq ($(DO_MKDBG),1)
Q:=
# we are not silent in this branch
else # DO_MKDBG
Q:=@
#.SILENT:
endif # DO_MKDBG

SRC:=src
SOURCES:=$(shell find $(SRC) -name "*.rs" -and -type f)
EXES_DBG:=$(addsuffix .dbg.elf, $(basename $(SOURCES)))
EXES_REL:=$(addsuffix .rel.elf, $(basename $(SOURCES)))
FLAGS_REL:=-Dwarnings -O -C debuginfo=0
FLAGS_DBG:=-g -Dwarnings

ifeq ($(DO_EXECS),1)
ALL+=$(EXES_DBG)
ALL+=$(EXES_REL)
endif # DO_EXECS

#########
# rules #
#########
.PHONY: all
all: $(ALL)
	@true
.PHONY: clean
clean:
	$(info doing [$@])
	$(Q)rm -f $(EXES)
.PHONY: clean_hard
clean_hard:
	$(info doing [$@])
	$(Q)git clean -qffxd
.PHONY: debug
debug:
	$(info SOURCES is $(SOURCES))
	$(info EXES_DBG is $(EXES_DBG))
	$(info EXES_REL is $(EXES_REL))

############
# patterns #
############
$(EXES_REL): %.rel.elf: %.rs
	$(info doing [$@])
	$(Q)rustc $(FLAGS_REL) $< -o $@
	$(Q)strip $@
$(EXES_DBG): %.dbg.elf: %.rs
	$(info doing [$@])
	$(Q)rustc $(FLAGS_DBG) $< -o $@
# $(Q)strip $@
