#include <stdio.h>

int main()
{
    /* removes double+ spaces from input */
    int c;
    while ((c = getchar()) != EOF)
    {
        if (c == '\t')
        {
            printf("\\t");
        }
        else if (c == '\b')
        {
            printf("\\b");
        }
        else if (c == '\\')
        {
            printf("\\\\");
        }
        else
        {
            putchar(c);
        }
        
        
        
    }
    return 0;
}
