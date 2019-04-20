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
from variance_finder import find_matches

items = [1.0, 2.1, 3.1, 4.2, 5.6, -1.2, -2.1]
matches = find_matches(items=items, total=4.5, fuzz=0.1, max_iterations=1_000_000_000, max_matches=50)
[[items[m] for m in match] for match in matches]
# returns [[1.0, 5.6, -2.1], [2.1, 5.6, -1.2, -2.1]]
```
