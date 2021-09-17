#include <stdio.h>

int child_main_impl(void *argument)
{
    (void)argument;

    printf("Hello, world!\n");
    return 0;
}
