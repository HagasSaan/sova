#include <stdio.h>
#include <unistd.h>

int main()
{
    // TODO: check result of execv and execve

    char * c[3]={"/usr/bin/touch","test.txt",NULL};
    execv(c[0], c);

    // TODO: test execv with NULL argv

    // TODO: test execve with argv and envp
    // TODO: test execve with NULL argv and envp
    // TODO: test execve with with argv and NULL envp
    // TODO: test execve with with NULL argv and NULL envp

  return 0;   
} 