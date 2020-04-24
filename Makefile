SRC:=src
SOURCES:=$(shell find $(SRC) -name "*.rs" -and -type f)
EXES:=$(addsuffix .exe, $(basename $(SOURCES)))
FLAGS:=-O

.PHONY: all
all: $(EXES) $(ALL_DEP)
	@true

$(EXES): %.exe: %.rs $(ALL_DEP)
	$(info doing [$@])
	@rustc $(FLAGS) $< -o $@

.PHONY: clean
clean:
	rm -f $(EXES)

.PHONY: debug
debug: $(ALL_DEP)
	$(info SOURCES is $(SOURCES))
	$(info EXES is $(EXES))
	@true
