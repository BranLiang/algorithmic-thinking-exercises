#include <stdio.h>

#define MAX_ROWS 3001
#define MAX_COLS 3001

typedef int board[MAX_ROWS][MAX_COLS];

int can_make_quality(int quality, int r, int c, int h, int w, board q)
{
    int i, j, top_row, left_col, bottom_row, right_col, total;
    static board masked, sum;

    // Setup the board to be of +1 or -1
    for (i = 0; i < r; i++)
    {
        for (j = 0; j < c; j++)
        {
            if (q[i][j] <= quality)
            {
                masked[i][j] = -1;
            }
            else
            {
                masked[i][j] = 1;
            }
        }
    }

    // Setup another board with sum values (dynamic programming)
    for (i = 0; i < r; i++)
    {
        for (j = 0; j < c; j++)
        {
            sum[i][j] = masked[i][j];
            if (i > 0 )
            {
                sum[i][j] += sum[i-1][j];
            }
            if (j > 0)
            {
                sum[i][j] += sum[i][j-1];
            }
            if (i > 0 && j > 0)
            {
                sum[i][j] -= sum[i-1][j-1];
            }
        }
    }

    // Check all median qualities
    for (top_row = 0; top_row < r - h + 1; top_row++)
    {
        for (left_col = 0; left_col < c - w + 1; left_col++)
        {
            bottom_row = top_row + h - 1;
            right_col = left_col + w - 1;
            total = sum[bottom_row][right_col];
            if (top_row > 0)
            {
                total -= sum[top_row - 1][right_col];
            }
            if (left_col > 0)
            {
                total -= sum[bottom_row][left_col - 1];
            }
            if (top_row > 0 && left_col > 0)
            {
                total += sum[top_row - 1][left_col - 1];
            }
            if (total <= 0)
            {
                return 1;
            }
        }
    }
    return 0;
}

int rectangle(int r, int c, int h, int w, board q)
{
    int low, mid, high;
    low = 0;
    high = r * c + 1;
    while (high - low > 1)
    {
        mid = (high + low) / 2;
        if (can_make_quality(mid, r, c, h, w, q))
        {
            high = mid;
        }
        else
        {
            low = mid;
        }
    }
    return high;
}

int main(void)
{
    board q = {
        {48, 16, 15, 45, 40, 28, 8},
        {20, 11, 36, 19, 24, 6, 33},
        {22, 39, 30, 7, 9, 1, 18},
        {14, 35, 2, 13, 31, 12, 46},
        {32, 37, 21, 3, 41, 23, 29},
        {42, 49, 38, 10, 17, 47, 5},
        {43, 4, 34, 25, 26, 27, 44}
    };
    int result = rectangle(7, 7, 5, 3, q);
    printf("%d\n", result);
}