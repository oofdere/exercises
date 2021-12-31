#include <stdio.h>

int main()
{
    // Check if EOF
    
    int c;

    while (1)
    {
        c = getchar();
        if (c == EOF)
        {
            printf(" (EOF)");
            exit(0);
        }
        else
        {
            printf("%c", c);
        }
    }
    
}
