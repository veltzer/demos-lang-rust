##############
# parameters #
##############
# do you want dependency on the Makefile itself ?
DO_ALLDEP:=1

########
# code #
########

# dependency on the makefile itself
ifeq ($(DO_ALLDEP),1)
.EXTRA_PREREQS+=$(foreach mk, ${MAKEFILE_LIST},$(abspath ${mk}))
endif

SRC:=src
SOURCES:=$(shell find $(SRC) -name "*.rs" -and -type f)
EXES:=$(addsuffix .exe, $(basename $(SOURCES)))
FLAGS:=-O

#########
# rules #
#########
.PHONY: all
all: $(EXES)
	@true
.PHONY: clean
clean:
	rm -f $(EXES)
.PHONY: debug
debug:
	$(info SOURCES is $(SOURCES))
	$(info EXES is $(EXES))
	@true
############
# patterns #
############
$(EXES): %.exe: %.rs
	$(info doing [$@])
	@rustc $(FLAGS) $< -o $@
