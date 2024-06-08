# Makefile for running tests with pytest

# Define the directories
PROJECT_DIR := $(shell pwd)
TESTS_DIR := $(PROJECT_DIR)/tests
PYTHONPATH := $(PROJECT_DIR)

# Default target
#.PHONY: test

test_python:
	@echo "Running tests with PYTHONPATH=$(PYTHONPATH)"
	@PYTHONPATH=$(PYTHONPATH) pytest $(TESTS_DIR)
