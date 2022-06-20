#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define hashsize(n) ((unsigned long)1 << (n))
#define hashmask(n) (hashsize(n) - 1)
#define NUM_BITS 17
#define WORD_LENGTH 16

unsigned long oaat(char *key, unsigned long len, unsigned long bits) {
    unsigned long hash, i;
    hash = 0;
    for (i = 0; i < len; i++)
    {
        hash += key[i];
        hash += (hash << 10);
        hash ^= (hash >> 6);
    }
    hash += (hash << 3);
    hash ^= (hash >> 11);
    hash += (hash << 15);
    return hash & hashmask(bits);
}

char *read_line(int size)
{
    char *str;
    int ch;
    int len = 0;
    str = malloc(size);
    if (str == NULL)
    {
        fprintf(stderr, "malloc error\n");
        exit(1);
    }
    while ((ch = getchar()) != EOF && (ch != '\n'))
    {
        str[len++] = ch;
        if (len == size)
        {
            size = size * 2;
            str = realloc(str, size);
            if (str == NULL)
            {
                fprintf(stderr, "realloc error\n");
                exit(1);
            }
        }
    }
    str[len] = '\0';
    return str;
}

typedef struct word_node
{
    char **word;
    struct word_node *next;
} word_node;

int in_hash_table(word_node *hash_table[], char *find, unsigned find_len)
{
    unsigned word_code;
    word_node *wordptr;
    word_code = oaat(find, find_len, NUM_BITS);
    wordptr = hash_table[word_code];
    while (wordptr)
    {
        if ((strlen(*(wordptr->word)) == find_len) && (strncmp(*(wordptr->word), find, find_len)) == 0)
        {
            return 1;
        }
        wordptr = wordptr->next;
    }
    return 0;
}

void identify_compound_words(char *words[], word_node *hash_table[], int total_words)
{
    unsigned len;
    int i, j;
    for (i = 0; i < total_words; i++)
    {
        len = strlen(words[i]);
        for (j = 1; j < len; j++)
        {
            if (in_hash_table(hash_table, words[i], j) && in_hash_table(hash_table, &words[i][j], len-j))
            {
                printf("%s\n", words[i]);
                break;
            }
        }
    }
}

int main(void)
{
    static char *words[1 << NUM_BITS] = {NULL};
    static word_node *hash_table[1 << NUM_BITS] = {NULL};
    int total = 0;
    char *word;
    word_node *wordptr;
    unsigned length, word_code;
    word = read_line(WORD_LENGTH);
    while (*word)
    {
        words[total] = word;
        wordptr = malloc(sizeof(word_node));
        if (wordptr == NULL)
        {
            fprintf(stderr, "malloc error.\n");
            exit(1);
        }
        length = strlen(word);
        word_code = oaat(word, length, NUM_BITS);

        wordptr->next = hash_table[word_code];
        wordptr->word = &words[total];
        hash_table[word_code] = wordptr;

        word = read_line(WORD_LENGTH);
        total++;
    }
    identify_compound_words(words, hash_table, total);
    return 0;
}