#include <stdio.h>

int main()
{
    printf(" °C     °F\n");
    printf("---  -----\n");


    int min = 0;
    int max = 300;
    int step = 20;

    float c, f;
    c = min;

    while (c <= max)
    {
        f = (9.0/5.0) * c + 32;
        printf("%3.0f %6.1f\n", c, f);
        c += step;
    }

    return 0;
}