EXAMPLES = examples

WINE_DATASETS = winequality-red.csv winequality-white.csv
WINE_HOSTINGS = https://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality
WINE_DIR=$(EXAMPLES)/wine_quality
WINE_TARGETS = $(addprefix $(WINE_DIR)/,$(WINE_DATASETS))

.PHONY: all
all: $(WINE_TARGETS)
	@echo $(WINE_TARGETS)

.PHONY: clean
clean:
	rm -r $(WINE_DIR)

$(WINE_DIR)/%:
	mkdir -p $(EXAMPLES)
	mkdir -p $(WINE_DIR)
	cd $(WINE_DIR) && wget $(patsubst $(WINE_DIR)/%,$(WINE_HOSTINGS)/%,$@)
