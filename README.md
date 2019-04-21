# Purpose
variance_finder is a Python package written in Rust to quickly find combinations
of items in a list that add up to a specified total and return their indices so 
they can be matched against their original rows.

# Installation
```bash
pip install variance_finder
```

# Example
```python
from variance_finder import find_matches, find_multiple_matches

# find a single total
items = [1.0, 2.1, 3.1, 4.2, 5.6, -1.2, -2.1]
matches = find_matches(items=items, total=4.5, fuzz=0.1, max_iterations=1_000_000_000, max_matches=50)
print(matches)
# [[0, 4, 6], [1, 4, 5, 6]]
print([[items[ix] for ix in match] for match in matches])
# [[1.0, 5.6, -2.1], [2.1, 5.6, -1.2, -2.1]]


# multiple totals at once
items = [1.0, 2.1, 3.1, 4.2, 5.6, -1.2, -2.1]
totals = [4.5, 0.2, 7.1]
matches = find_multiple_matches(items=items, totals=totals, fuzz=0.1, max_iterations=1_000_000_000, max_matches=50)
print(matches)
# [[[0, 4, 6], [1, 4, 5, 6]], [], [[0, 2, 3, 5], [0, 1, 2, 3, 5, 6]]]
for total, matches_for_total in zip(totals, matches):
    matches = [[(ix, items[ix]) for ix in match] for match in matches_for_total]
    print("total = {}, matches = {}".format(total, matches))
# total = 4.5, matches = [[(0, 1.0), (4, 5.6), (6, -2.1)], [(1, 2.1), (4, 5.6), (5, -1.2), (6, -2.1)]]
# total = 0.2, matches = []
# total = 7.1, matches = [[(0, 1.0), (2, 3.1), (3, 4.2), (5, -1.2)], [(0, 1.0), (1, 2.1), (2, 3.1), (3, 4.2), (5, -1.2), (6, -2.1)]]
```
