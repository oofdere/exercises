#include <stdio.h>

int main()
{
    /* prints stats about stdin */
    int blanks = 0;
    int tabs = 0;
    int newlines = 0;
    
    int c;
    while ((c = getchar()) != EOF)
    {
        if (c == ' ')
        {
            blanks++;
        }
        else if (c == '\t')
        {
            tabs++;
        }
        else if (c == '\n')
        {
            newlines++;
        }        
    }
    
    printf("  Blanks: %i\n", blanks);
    printf("    Tabs: %i\n", tabs);
    printf("Newlines: %i\n", newlines);

    return 0;
}
