#define SIZE 100000
#include <string.h>
#include <stdio.h>

int prefix_len(char s1[], char s2[])
{
    int i = 1;
    while (s1[i] == s2[i])
    {
        i++;
    }
    return i - 1;
}

int suffix_len(char s1[], char s2[], int len)
{
    int i = len;
    while (i >= 2 && s1[i] == s2[i-1])
    {
        i--;
    }
    return len - i;
}

int main(void)
{
    static char s1[SIZE + 2], s2[SIZE + 2];
    int len, prefix, suffix, total;
    fgets(&s1[1], sizeof(s1) - 1, stdin);
    fgets(&s2[1], sizeof(s2) - 1, stdin);
    len = strlen(&s1[1]);
    prefix = prefix_len(s1, s2);
    suffix = suffix_len(s1, s2, len);
    total = prefix + suffix - len + 2;
    if (total <= 0)
    {
        printf("0\n");
    }
    else
    {
        printf("%d\n", total);
        for (int i = 0; i < total; i++)
        {
            printf("%d", len - suffix + i);
            if (i == total - 1)
            {
                printf("\n");
            }
            else
            {
                printf(" ");
            }
        }
    }
}