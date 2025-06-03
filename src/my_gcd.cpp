#include <algorithm>

extern "C" {
    // Compute the GCD of two integers using the Euclidean algorithm.
    int gcd(int a, int b) {
        if (a == b)
            return a;
        if (b > a)
            std::swap(a, b);
        while (b > 0) {
            int temp = a;
            a = b;
            b = temp % b;
        }
        return a;
    }
}