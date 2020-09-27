INPUT = $(wildcard _pages/*.md)
OUTPUT := $(patsubst _pages/%.md,docs/%.html, $(INPUT))

all: $(OUTPUT)

docs/%.html: _pages/%.md templates/header.html
	touch $@
	@cat templates/header.html > $@
	@./render.sh $< >> $@

.PHONY: clean
clean:
	rm docs/*.html
