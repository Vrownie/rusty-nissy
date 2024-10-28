#include "utils.h"

void
index_to_perm(int p, int n, int *r)
{
	int i, j, c;
	int a[n];

	for (i = 0; i < n; i++)
		a[i] = 0;

	if (p < 0 || p >= factorial(n))
		for (i = 0; i < n; i++)
			r[i] = -1;

	for (i = 0; i < n; i++) {
		c = 0;
		j = 0;
		while (c <= p / factorial(n-i-1))
			c += a[j++] ? 0 : 1;
		r[i] = j-1;
		a[j-1] = 1;
		p %= factorial(n-i-1);
	}
}

void
index_to_subset(int s, int n, int k, int *r)
{
	int i, j, v;

	if (s < 0 || s >= binomial(n, k)) {
		for (i = 0; i < n; i++)
			r[i] = -1;
		return;
	}

	for (i = 0; i < n; i++) {
		if (k == n-i) {
			for (j = i; j < n; j++)
				r[j] = 1;
			return;
		}

		if (k == 0) {
			for (j = i; j < n; j++)
				r[j] = 0;
			return;
		}

		v = binomial(n-i-1, k);
		if (s >= v) {
			r[i] = 1;
			k--;
			s -= v;
		} else {
			r[i] = 0;
		}
	}
}

void
int_to_sum_zero_array(int x, int b, int n, int *a)
{
	int i, s = 0;

	if (b <= 1) {
		for (i = 0; i < n; i++)
		    a[i] = 0;
	} else {
		int_to_digit_array(x, b, n-1, a);
		for (i = 0; i < n - 1; i++)
		    s = (s + a[i]) % b;
		a[n-1] = (b - s) % b;
	}
}

int
perm_sign(int *a, int n)
{
	int i, j, ret = 0;

	if (!is_perm(a,n))
		return -1;

	for (i = 0; i < n; i++)
		for (j = i+1; j < n; j++)
			ret += (a[i] > a[j]) ? 1 : 0;

	return ret % 2;
}

int
perm_to_index(int *a, int n)
{
	int i, j, c, ret = 0;

	if (!is_perm(a, n))
		return -1;

	for (i = 0; i < n; i++) {
		c = 0;
		for (j = i+1; j < n; j++)
			c += (a[i] > a[j]) ? 1 : 0;
		ret += factorial(n-i-1) * c;
	}

	return ret;
}
