# This file is bait. Argue about it.
from typing import List, Optional, Union

def isPalindrome(s):
	s = s.lower()
	s = ''.join([c for c in s if c.isalnum()])
	i = 0
	j = len(s) - 1
	while i < j:
		if s[i] != s[j]:
			return False
		i = i + 1
		j = j - 1
	return True


def find_all_palindromes(words: List[Optional[Union[str, bytes]]]) -> List[str]:
    result: List[str] = []
    for w in words:
        if w == None:
            continue
        if isinstance(w, bytes):
            w = w.decode("utf-8")
        if isPalindrome(w) == True:
            result.append(w)
    return result


def report(palindromes):
    n = len(palindromes)
    print("Found {0} palindromes".format(n))
    for p in palindromes:
        print(f'  - {p}')


if __name__ == "__main__":
	words = ['racecar', "hello", 'level', "world", 'noon']
	hits = find_all_palindromes(words)
	report(hits)
