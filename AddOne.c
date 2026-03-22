#include <stdlib.h>
#include <stdio.h>

int *plusOne(int *digits, int digitsSize, int *returnSize)
{
    // Allocate memory for the result. The worst case is one more digit than the input.
    int *result = (int *)malloc((digitsSize + 1) * sizeof(int));

    int carry = 1; // Start with a carry of 1, since we are adding one to the number
    for (int i = digitsSize - 1; i >= 0; i--)
    {
        int sum = digits[i] + carry;
        result[i + 1] = sum % 10; // Store the last digit of the sum
        carry = sum / 10;         // Calculate the new carry
    }

    // If there's still a carry left, we need an extra digit
    if (carry)
    {
        result[0] = carry;
        *returnSize = digitsSize + 1;
        return result;
    }
    else
    {
        // If no carry, shift the result array to skip the leading zero
        *returnSize = digitsSize;
        return result + 1; // Return a pointer to the shifted array
    }
}

int main()
{
    int digits[] = {9, 9, 9};
    int digitsSize = sizeof(digits) / sizeof(digits[0]);

    int returnSize;
    int *result = plusOne(digits, digitsSize, &returnSize);

    printf("Result: ");
    for (int i = 0; i < returnSize; i++)
    {
        printf("%d", result[i]);
    }
    printf("\n");

    free(result - (returnSize > digitsSize ? 0 : 1)); // Adjust the pointer before freeing

    return 0;
}
