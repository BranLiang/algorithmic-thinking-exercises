#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define MAX_LANGUAGES 101
#define MAX_TRANSLATORS 4500

typedef struct edge
{
    int to_lang;
    int cost;
    struct edge *next;
} edge;

char *read_language()
{
    int size = 16;
    char *language = malloc(sizeof(char) * size);
    char ch = getchar();
    int index = 0;
    while (ch != ' ' && ch != '\n' && ch != EOF)
    {
        language[index] = ch;
        index += 1;
        ch = getchar();
        if (index == size - 1)
        {
            size = size * 2;
            language = realloc(language, sizeof(char) * size);
            if (language == NULL)
            {
                printf("realloc error!\n");
                exit(1);
            }
            
        }
    }
    language[index] = '\0';
    return language;
}

int find_index(char *languages[], int languages_num, char *language)
{
    for (int i = 0; i < languages_num; i++)
    {
        if (strcmp(languages[i], language) == 0)
        {
            return i;
        }
    }
    return -1;
}

void add_language(edge *adj_list[], int from_lang, int to_lang, int cost)
{
    edge *e = malloc(sizeof(edge));
    if (e == NULL)
    {
        fprintf(stderr, "malloc error\n");
        exit(1);
    }
    
    e->to_lang = to_lang;
    e->cost = cost;
    e->next = adj_list[from_lang];
    adj_list[from_lang] = e;
}

void find_distances(edge *adj_list[], int languages_num, int min_costs[])
{
    int min_moves[MAX_LANGUAGES];
    for (int i = 0; i < languages_num; i++)
    {
        min_moves[i] = -1;
        min_costs[i] = -1;
    }
    min_moves[0] = 0;
    min_costs[0] = 0;

    int cur_positions[MAX_LANGUAGES], next_positions[MAX_LANGUAGES];
    int cur_positions_num, next_positions_num;
    int from_lang, added_lang, best;
    edge *e;

    cur_positions[0] = 0;
    cur_positions_num = 1;
    
    while (cur_positions_num > 0)
    {
        next_positions_num = 0;
        for (int i = 0; i < cur_positions_num; i++)
        {
            from_lang = cur_positions[i];
            e = adj_list[from_lang];

            while (e)
            {
                if (min_moves[e->to_lang] == -1)
                {
                    min_moves[e->to_lang] = 1 + min_moves[from_lang];
                    next_positions[next_positions_num] = e->to_lang;
                    next_positions_num += 1;
                }
                e = e->next;
            }
        }

        for (int i = 0; i < next_positions_num; i++)
        {
            added_lang = next_positions[i];
            e = adj_list[added_lang];
            best = -1;
            while (e)
            {
                if (min_moves[e->to_lang] + 1 == min_moves[added_lang] && (best == -1 || e->cost < best))
                {
                    best = e->cost;
                }
                e = e->next;
            }
            min_costs[added_lang] = best;
        }
        

        for (int i = 0; i < next_positions_num; i++)
        {
            cur_positions[i] = next_positions[i];
        }
        cur_positions_num = next_positions_num;
    }
}

void solve(int min_costs[], int languages_num)
{
    int total = 0;
    for (int i = 0; i < languages_num; i++)
    {
        if (min_costs[i] == -1)
        {
            printf("Impossible\n");
            return;
        }
        total += min_costs[i];
    }
    printf("%d\n", total);
}

int main(void)
{
    static edge *adj_list[MAX_LANGUAGES] = {NULL};

    // Read the languages and translators number
    int languages_num, translators_num;
    char *languages[MAX_LANGUAGES];
    scanf("%d %d ", &languages_num, &translators_num);
    languages_num += 1;

    // Add variables for each translator
    char *from_lang, *to_lang;
    int cost, from_index, to_index;

    // min costs
    int min_costs[MAX_LANGUAGES];

    // Set te english to be index 0, and other languages as follows
    languages[0] = "English";
    for (int i = 1; i < languages_num; i++)
    {
        languages[i] = read_language();
    }

    // Read translators
    for (int i = 0; i < translators_num; i++)
    {
        from_lang = read_language();
        from_index = find_index(languages, languages_num, from_lang);

        to_lang = read_language();
        to_index = find_index(languages, languages_num, to_lang);
        scanf("%d ", &cost);

        if (from_index == -1)
        {
            printf("Unknown language: %s\n", from_lang);
            continue;
        }

        if (to_index == -1)
        {
            printf("Unknown language: %s\n", to_lang);
            continue;
        }
        
        add_language(adj_list, from_index, to_index, cost);
        add_language(adj_list, to_index, from_index, cost);
    }
    find_distances(adj_list, languages_num, min_costs);
    solve(min_costs, languages_num);
}