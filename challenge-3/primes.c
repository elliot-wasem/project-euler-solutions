/**
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 */

// Program to print all prime factors
# include <stdio.h>
# include <math.h>

// A function to print all prime factors of a given number n
void primeFactors(long int n)
{
  // Print the number of 2s that divide n
  while (n%2 == 0)
    {
      printf("%d ", 2);
      n = n/2;
    }
 
  // n must be odd at this point.  So we can skip 
  // one element (Note i = i +2)
for (int i = 3; i <= sqrt((double) n); i = i+2)
    {
      // While i divides n, print i and divide n
      while (n%i == 0)
        {
	  printf("%d ", i);
	  n = n/i;
        }
    }
 
  // This condition is to handle the case when n 
  // is a prime number greater than 2
  if (n > 2)
    printf ("%d ", (int) n);
}

int main() {
  primeFactors(600851475143);
  return 0;
}
