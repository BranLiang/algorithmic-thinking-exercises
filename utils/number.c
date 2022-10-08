#include <stdio.h>

int get_number(int *num)
{
    int ch;
    int ret = 0;
    ch = getchar();
    if (ch == ' ')
    {
        return get_number(num);
    }
    
    while (ch != ' ' && ch != '\n')
    {
        ret = 10 * ret + ch - '0';
        ch = getchar();
    }
    *num = ret;
    return ch != '\n';
}