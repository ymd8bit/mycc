#include <stdio.h>
#include <stdlib.h>

int main()
{
  char str[30] = "20+4-5";
  char *ptr;
  printf("String part is |%p|\n", ptr);
  long ret;

  ret = strtol(str, &ptr, 10);
  printf("The number(unsigned long integer) is %ld\n", ret);
  printf("String part is |%p|", ptr);

  return (0);
}