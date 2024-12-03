CXX = g++

STD = c++11

SRC_FILES = main.cpp

OUT = a.out

CXXFLAGS = -O3 -std=$(STD)

build:
	$(CXX) $(SRC_FILES) $(CXXFLAGS) -o $(OUT)

run:
	./$(OUT)

clean:
	rm -f $(OUT)

.PHONY: build run clean
