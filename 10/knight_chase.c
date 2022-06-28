#include <stdio.h>
#define MAX_ROWS 99
#define MAX_COLS 99

typedef struct position
{
    int row;
    int col;
} position;

typedef struct board
{
    int rows;
    int cols;
} board;

typedef position positions[MAX_ROWS * MAX_COLS];

int knight_jumps[8][2] = {
    { 1, 2 },
    { 2, 1 },
    { -1, 2 },
    { 2, -1 },
    { 1, -2 },
    { -2, 1 },
    { -1, -2 },
    { -2, -1 }
};

void add_position(board chess, position from, position p, position current[], int *current_size, int moves[MAX_ROWS + 1][MAX_COLS + 1])
{
    if (
        p.col > 0 &&
        p.col <= chess.cols &&
        p.row > 0 &&
        p.row <= chess.rows &&
        moves[p.row][p.col] == -1
    )
    {
        current[*current_size] = p;
        *current_size += 1;
        moves[p.row][p.col] = moves[from.row][from.col] + 1;
    }
}

int find_distance(position start, position dest, board chess, int moves[MAX_ROWS + 1][MAX_COLS + 1])
{
    if (moves[dest.row][dest.col] != -1)
    {
        return moves[dest.row][dest.col];
    }
    
    positions current_postions, new_postions;
    int current_size, new_size;

    // It takes 0 move from start to start
    moves[start.row][start.col] = 0;
    // Push the start postion to the current
    current_size = 1;
    current_postions[0] = start;
    while (current_size > 0)
    {
        new_size = 0;
        for (int i = 0; i < current_size; i++)
        {
            position p = current_postions[i];
            if (p.row == dest.row && p.col == dest.col)
            {
                return moves[p.row][p.col];
            }
            for (int j = 0; j < 8; j++)
            {
                position new_p;
                new_p.row = p.row + knight_jumps[j][0];
                new_p.col = p.col + knight_jumps[j][1];
                add_position(chess, p, new_p, new_postions, &new_size, moves);
            }
        }
        current_size = new_size;
        for (int i = 0; i < new_size; i++)
        {
            current_postions[i] = new_postions[i];
        }
    }
    return -1;
}

void solve(position knight, position pawn, board chess)
{
    int num_moves, knight_takes;
    position current_pawn, dest;
    current_pawn.col = pawn.col;
    current_pawn.row = pawn.row;

    int moves[MAX_ROWS + 1][MAX_COLS + 1];
    // Initialize the moves data
    for (int row = 1; row <= MAX_ROWS; row++)
    {
        for (int col = 1; col < MAX_COLS; col++)
        {
            moves[row][col] = -1;
        }
    }

    num_moves = 0;
    while (current_pawn.row < chess.rows)
    {
        knight_takes = find_distance(knight, current_pawn, chess, moves);
        if (knight_takes >= 0 && num_moves >= knight_takes && (num_moves - knight_takes) % 2 == 0)
        {
            printf("Win in %d knight move(s).\n", num_moves);
            return;
        }
        num_moves++;
        current_pawn.row++;
    }

    num_moves = 0;
    current_pawn.row = pawn.row;
    while (current_pawn.row < chess.rows)
    {
        dest.col = current_pawn.col;
        dest.row = current_pawn.row + 1;
        knight_takes = find_distance(knight, dest, chess, moves);
        if (knight_takes >= 0 && num_moves >= knight_takes && (num_moves - knight_takes) % 2 == 0)
        {
            printf("Stalemate in %d knight move(s).\n", num_moves);
            return;
        }
        num_moves++;
        current_pawn.row++;
    }

    printf("Loss in %d knight move(s).\n", chess.rows - pawn.row - 1);
}

int main(void)
{
    int case_num;
    position knight_start, pawn_start;
    board chess;
    scanf("%d", &case_num);

    for (int i = 0; i < case_num; i++)
    {
        scanf(
            "%d %d %d %d %d %d",
            &chess.rows, &chess.cols, &pawn_start.row, &pawn_start.col, &knight_start.row, &knight_start.col
        );
        solve(knight_start, pawn_start, chess);
    }
}