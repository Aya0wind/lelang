clang -c out.ll
clang -c print.c
clang ./out.o ./print.o -o leout
rm ./print.o
./leout