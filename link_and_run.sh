clang -c print.c -o print.o
clang ./out.ll ./print.o -o leout
./leout