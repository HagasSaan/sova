#include <stdio.h>
#include <unistd.h>

int main()
{
  char * c[3]={"/usr/bin/touch","test.txt",NULL};
  execv(c[0], c);
  return 0;   
} 