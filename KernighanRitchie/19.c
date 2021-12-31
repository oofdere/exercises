#include <stdio.h>

int main()
{
    /* removes double+ spaces from input */
    int space = 0;

    int c;
    while ((c = getchar()) != EOF)
    {
        if (c != ' ')
        {
            space = 0;
            putchar(c);
        }
        else
        {
            if (space == 0)
            {
                putchar(c);
            }
            space++;
        }
    }
    return 0;
}
