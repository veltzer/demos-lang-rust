##############
# parameters #
##############
# do you want dependency on the Makefile itself ?
DO_ALLDEP:=1
# do you want to show the commands executed ?
DO_MKDBG?=0
# do you compile rust files?
DO_EXECS:=1
# do you want to run mdl on md files?
DO_MD_MDL:=1
# do spell check on all?
DO_MD_ASPELL:=1
# do you want to build everything with cargo?
DO_CARGO:=1

########
# code #
########
ALL:=

# silent stuff
ifeq ($(DO_MKDBG),1)
Q:=
# we are not silent in this branch
else # DO_MKDBG
Q:=@
#.SILENT:
endif # DO_MKDBG

SRC:=src
SOURCES:=$(shell find $(SRC) -type f -and -name "*.rs")
CARGO_SRC:=$(shell find examples exercises -type f -and -name "*.rs")
CARGO_TOML:=$(shell find examples exercises -type f -and -name "Cargo.toml")
EXES_DBG:=$(addsuffix .dbg.elf, $(basename $(SOURCES)))
EXES_REL:=$(addsuffix .rel.elf, $(basename $(SOURCES)))
FLAGS_REL:=-Dwarnings -O -C debuginfo=0
FLAGS_DBG:=-g -Dwarnings
MD_SRC:=$(shell find src exercises examples -type f -and -name "*.md")
MD_BAS:=$(basename $(MD_SRC))
MD_MDL:=$(addprefix out/,$(addsuffix .mdl,$(MD_BAS)))
MD_ASPELL:=$(addprefix out/,$(addsuffix .aspell,$(MD_BAS)))

ifeq ($(DO_EXECS),1)
ALL+=$(EXES_DBG)
ALL+=$(EXES_REL)
endif # DO_EXECS

ifeq ($(DO_MD_MDL),1)
ALL+=$(MD_MDL)
endif # DO_MD_MDL

ifeq ($(DO_MD_ASPELL),1)
ALL+=$(MD_ASPELL)
endif # DO_MD_ASPELL

ifeq ($(DO_CARGO),1)
ALL+=out/cargo.stamp
endif # DO_CARGO

#########
# rules #
#########
.PHONY: all
all: $(ALL)
	@true
.PHONY: clean
clean:
	$(info doing [$@])
	$(Q)rm -f $(ALL)
.PHONY: clean_hard
clean_hard:
	$(info doing [$@])
	$(Q)git clean -qffxd
.PHONY: debug
debug:
	$(info SOURCES is $(SOURCES))
	$(info CARGO_SRC is $(CARGO_SRC))
	$(info CARGO_TOML is $(CARGO_TOML))
	$(info EXES_DBG is $(EXES_DBG))
	$(info EXES_REL is $(EXES_REL))
	$(info MD_SRC is $(MD_SRC))
	$(info MD_BAS is $(MD_BAS))
	$(info MD_ASPELL is $(MD_ASPELL))
	$(info MD_MDL is $(MD_MDL))

.PHONY: spell_many
spell_many:
	$(info doing [$@])
	$(Q)aspell_many.sh $(MD_SRC)

out/cargo.stamp: $(CARGO_SRC) $(CARGO_TOML)
	$(info doing [$@])
	$(Q)cargo build --jobs $$(nproc)
	$(Q)pymakehelper touch_mkdir $@
# $(Q)cargo build --quiet --jobs $$(nproc)

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
$(MD_MDL): out/%.mdl: %.md .mdlrc .mdl.style.rb
	$(info doing [$@])
	$(Q)GEM_HOME=gems gems/bin/mdl $<
	$(Q)mkdir -p $(dir $@)
	$(Q)touch $@
$(MD_ASPELL): out/%.aspell: %.md .aspell.conf .aspell.en.prepl .aspell.en.pws
	$(info doing [$@])
	$(Q)aspell --conf-dir=. --conf=.aspell.conf list < $< | pymakehelper error_on_print sort -u
	$(Q)pymakehelper touch_mkdir $@

##########
# alldep #
##########
ifeq ($(DO_ALLDEP),1)
.EXTRA_PREREQS+=$(foreach mk, ${MAKEFILE_LIST},$(abspath ${mk}))
endif # DO_ALLDEP
