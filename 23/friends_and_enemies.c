#include <stdio.h>

#define MAX_N 10000

int find(int parent[], int person)
{
    int p = person, temp;
    while (person != parent[person])
    {
        person = parent[person];
    }

    // Path compression
    while (p != person)
    {
        temp = parent[p];
        parent[p] = person;
        p = temp;
    }

    return person;
}

void union_sets(int parent[], int p1, int p2, int size[])
{
    int set1, set2;
    set1 = find(parent, p1);
    set2 = find(parent, p2);

    if (set1 == set2)
    {
        return;
    }

    if (size[set1] > size[set2])
    {
        parent[set2] = set1;
        size[set1] += size[set2];
    }
    else
    {
        parent[set1] = set2;
        size[set2] += size[set1];
    }
}

void set_friends(int parent[], int p1, int p2, int size[], int enemy_of[])
{
    int set1, set2;
    set1 = find(parent, p1);
    set2 = find(parent, p2);

    union_sets(parent, p1, p2, size);
    if (enemy_of[set1] != -1 && enemy_of[set2] != -1)
    {
        union_sets(parent, enemy_of[set1], enemy_of[set2], size);
    }
    else if (enemy_of[set1] == -1)
    {
        enemy_of[set1] = enemy_of[set2];
    }
    else if (enemy_of[set2] == -1)
    {
        enemy_of[set2] = enemy_of[set1];
    }
}

void set_enemies(int parent[], int p1, int p2, int size[], int enemy_of[])
{
    int set1, set2, enemy;
    set1 = find(parent, p1);
    set2 = find(parent, p2);

    enemy = enemy_of[set1];
    if (enemy == -1)
    {
        enemy_of[set1] = p2;
    }
    else
    {
        union_sets(parent, enemy, p2, size);
    }
    
    enemy = enemy_of[set2];
    if (enemy == -1)
    {
        enemy_of[set2] = p1;
    }
    else
    {
        union_sets(parent, enemy, p1, size);
    }
}

int are_friends(int parent[], int p1, int p2)
{
    int set1, set2;
    set1 = find(parent, p1);
    set2 = find(parent, p2);
    if (set1 == set2)
    {
        return 1;
    }
    else
    {
        return 0;
    }
}

int are_enemies(int parent[], int p1, int p2, int enemy_of[])
{
    int set1, set2, enemy, enemy_set;
    set1 = find(parent, p1);
    enemy = enemy_of[set1];
    if (enemy == -1)
    {
        return 0;
    }
    
    set2 = find(parent, p2);
    enemy_set = find(parent, enemy);
    return set2 == enemy_set;
}

int main(void)
{
    int n;
    int cmd, p1, p2;
    int parent[MAX_N];
    int enemy_of[MAX_N];
    int size[MAX_N];

    scanf("%d", &n);

    for (int i = 0; i < n; i++)
    {
        parent[i] = i;
        enemy_of[i] = -1;
        size[i] = 0;
    }

    scanf("%d %d %d", &cmd, &p1, &p2);
    while (cmd != 0)
    {
        if (cmd == 1)
        {
            // set_friends
            if (are_enemies(parent, p1, p2, enemy_of))
            {
                puts("-1");
            }
            else
            {
                set_friends(parent, p1, p2, size, enemy_of);
            }
            
        }
        else if (cmd == 2)
        {
            // set_enemies
            if (are_friends(parent, p1, p2))
            {
                puts("-1");
            }
            else
            {
                set_enemies(parent, p1, p2, size, enemy_of);
            }
        }
        else if (cmd == 3)
        {
            // are_friends
            printf("%d\n", are_friends(parent, p1, p2));
        }
        else if (cmd == 4)
        {
            // are_enemies
            printf("%d\n", are_enemies(parent, p1, p2, enemy_of));
        }
        
        scanf("%d %d %d", &cmd, &p1, &p2);
    }
    
}