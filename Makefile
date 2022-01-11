IDIR=include
SRC=src
ODIR=obj

CXX=g++
CXXFLAGS=-I $(IDIR) -std=c++11 -pthread -O3 -Wall -Wextra 
LDFLAGS=-lSDL2

DEPS=$(IDIR)/*.hpp
SRCS=$(wildcard $(SRC)/*.cpp)
OBJ=$(SRCS:$(SRC)/%.cpp=$(ODIR)/%.o)

all: build_files emuboy

emuboy: $(OBJ)  
	$(CXX) -o bin/$@ $^ $(CXXFLAGS) $(LDFLAGS)

build_files: 
	mkdir -p $(ODIR) 
	mkdir -p bin 

$(ODIR)/%.o: $(SRC)/%.cpp  
	$(CXX) -c -o $@ $< $(CXXFLAGS)

clean:
	rm -rf $(ODIR)
	rm -rf bin

run: 
	bin/emuboy

debug: all run
