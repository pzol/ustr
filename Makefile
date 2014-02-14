# See http://site.icu-project.org/download/52#TOC-ICU4C-Download
ROOT ?= .

SRCDIR = $(ROOT)/src/ustr
BINDIR = $(ROOT)/bin
LIBDIR = $(ROOT)/lib
WORKSPACE = $(ROOT)/.rust

LIBICU=icu4c-52_1-src

build:
	mkdir -p $(LIBDIR) $(BINDIR)
	rustc -L $(LIBDIR) --out-dir $(LIBDIR) $(SRCDIR)/lib.rs

test: clean build
	rustc -L $(LIBDIR) -o $(BINDIR)/test --test $(SRCDIR)/test.rs
	./bin/test

rust-encoding: .rust/rust-encoding/libencoding.dummy
	
.rust/rust-encoding/libencoding.dummy:
	git clone https://github.com/lifthrasiir/rust-encoding.git .rust/rust-encoding
	cd .rust/rust-encoding && ./configure && make && cp libencoding*.rlib ../../lib

tmp:
	mkdir tmp

./tmp/libicu-src.tgz: tmp
	curl -o tmp/libicu-src.tgz http://download.icu-project.org/files/icu4c/52.1/$(LIBICU).tgz

./tmp/icu: ./tmp/libicu-src.tgz
	cd tmp; tar xzf libicu-src.tgz

clean: 
	rm -rf $(BINDIR) $(LIBDIR)/libustr-* .rust


# all: vmod_rpm_test

# vmod_rpm.bc: vmod_rpm.rs
# 	rustc --emit-llvm vmod_rpm.rs        

# vmod_rpm.o: vmod_rpm.bc
# 	clang -c vmod_rpm.bc

# vmod_rpm_test: vmod_rpm.c vmod_rpm.o
# 	clang -o vmod_rpm_test vmod_rpm.c vmod_rpm.o /usr/local/lib/rustlib/x86_64-apple-darwin/lib/libstd-966edb7e-0.10-pre.dylib /usr/local/lib/rustlib/x86_64-apple-darwin/lib/libextra-64ade3d6-0.10-pre.dylib

# clean:
# 	rm *.bc
# 	rm *.o
# 	rm vmod_rpm_test
